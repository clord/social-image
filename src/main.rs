use crate::identifier::FileId;
use color_eyre::Result;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use filetime::FileTime;
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use std::path::PathBuf;
use walkdir::WalkDir;

#[macro_use]
extern crate rocket;

mod apikey;
mod identifier;
mod render;
mod resources;

#[get("/")]
fn index() -> &'static str {
    "Post SVGs and then request renderings in other formats (currently only png)

    USAGE

     GET /
        -> this help content

     GET /images/<name>
        -> load the image. if not cached, will create cached copy. 

     PUT /images/<name>
        -> Put a new svg on top of an existing filename. resets cache for <name>.

     POST /images
        -> POST new svg to images, get redirected to image if valid or error (creates new) (does not cache)

     POST /images/<name>/resource/<resource>
        -> POST relevant files that the SVG will need to render (e.g., referred PNGs) (invalidates <name>)

     DELETE /images/<name>
        -> Remove the image from the system

    "
}

#[get("/<filename..>")]
async fn get_image_file(filename: PathBuf) -> std::result::Result<NamedFile, Status> {
    let mut png_path = std::path::PathBuf::new();
    png_path.push(filename);
    png_path.push("img");
    let mut svg_path = png_path.clone();
    png_path.set_extension("png");
    svg_path.set_extension("svg");

    if png_path.is_file() {
        NamedFile::open(&png_path)
            .await
            .map_err(|_| Status::NotFound)
    } else {
        match std::fs::read(svg_path) {
            Err(e) => {
                error!("can't read svg file {e:?}");
                Err(Status::InternalServerError)
            }
            Ok(file) => match render::svg_to_png(&file, &png_path) {
                Ok(()) => NamedFile::open(&png_path)
                    .await
                    .map_err(|_| Status::InternalServerError),
                Err(e) => Err(e),
            },
        }
    }
}

/// Some SVG files require additional resources in order to render correctly.
/// you can post resources and they will become available on the next render (as <name>).
#[post("/<dir>/<filename>/resource/<name>", data = "<file>")]
async fn attach_resource(
    dir: &str,
    filename: &str,
    name: &str,
    mut file: TempFile<'_>,
    _api_key: apikey::ApiKey<'_>,
) -> std::result::Result<Redirect, Status> {
    let mut path = std::path::PathBuf::new();
    path.push(dir);
    path.push(filename);

    if !path.is_dir() {
        return Err(Status::NotFound);
    }

    let mut png_path = path.clone();

    path.push(format!("{name}.tmp"));
    file.persist_to(&path).await.expect("persist to work");

    png_path.push("img");
    png_path.set_extension("png");
    std::fs::remove_file(png_path).unwrap_or(());

    match resources::get_resource_path(&path) {
        Ok(()) => Ok(Redirect::to(format!("/images/{dir}/{filename}"))),
        Err(e) => {
            error!("Failed to persist resource: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", format = "image/svg+xml", data = "<file>")]
async fn create_file_with_png(
    file: Vec<u8>,
    _api_key: apikey::ApiKey<'_>,
) -> std::result::Result<Redirect, Status> {
    let id = FileId::new(&file);
    let mut png_path = std::path::PathBuf::new();
    png_path.push(id.dir());
    png_path.push(id.name());
    if let Err(e) = std::fs::create_dir_all(&png_path) {
        error!("Failed to create a directory: {e:?}");
        return Err(Status::InternalServerError);
    }

    png_path.push("img");
    let mut svg_path = png_path.clone();
    svg_path.set_extension("svg");
    if let Err(e) = std::fs::write(svg_path, &file) {
        error!("Failed to write svg file: {e:?}");
        return Err(Status::InternalServerError);
    }

    let dir = id.dir();
    let name = id.name();
    Ok(Redirect::to(format!("/images/{dir}/{name}")))
}

#[delete("/<filename..>")]
async fn delete_file(filename: PathBuf, _api_key: apikey::ApiKey<'_>) -> &'static str {
    let mut image_dir = std::path::PathBuf::new();
    image_dir.push(filename);

    match std::fs::remove_dir(image_dir) {
        Ok(_) => "Ok",
        Err(e) => {
            error!("while removing file: {e:?}");
            "Error"
        }
    }
}

#[put("/<path>/<filename>", format = "image/svg+xml", data = "<file>")]
async fn update_file(
    path: &str,
    filename: &str,
    mut file: TempFile<'_>,
    _api_key: apikey::ApiKey<'_>,
) -> std::result::Result<Redirect, Status> {
    let mut png_path = std::path::PathBuf::new();
    png_path.push(&path);
    png_path.push(&filename);

    if let Err(e) = std::fs::create_dir_all(&png_path) {
        error!("Failed to create a directory: {e:?}");
        return Err(Status::InternalServerError);
    }

    png_path.push("img");
    let mut svg_file = png_path.clone();
    svg_file.set_extension("svg");
    png_path.set_extension("png");
    std::fs::remove_file(png_path).unwrap_or(());

    match file.persist_to(&svg_file).await {
        Ok(()) => Ok(Redirect::to(format!("/images/{path}/{filename}"))),
        Err(e) => {
            error!("Failed to persist resource: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[derive(Deserialize, Serialize)]
struct AppConfig {
    key: String,
    store: PathBuf,
    expire_png_secs: i64,
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            key: "default".into(),
            store: "/tmp/data".into(),
            expire_png_secs: 72 * 60 * 60,
        }
    }
}

fn trim_expired_files(store: &std::path::Path, expire: i64) -> Result<()> {
    for entry in WalkDir::new(&store) {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name() {
            if let Some(ext) = path.extension() {
                if name == "img" && ext == "png" {
                    let metadata = std::fs::metadata(path)?;
                    let mtime = FileTime::from_last_modification_time(&metadata);
                    if FileTime::now().seconds() > (mtime.seconds() + expire) {
                        std::fs::remove_file(path)?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[catch(500)]
fn internal_error() -> &'static str {
    "{\"error\": \"internal_error\"}"
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "{\"error\": \"not_found\"}"
}

#[catch(default)]
fn default(status: Status, _req: &Request) -> String {
    format!("{{\"status\": \"{status}\"}}")
}

#[launch]
fn rocket() -> _ {
    color_eyre::install().unwrap();
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(AppConfig::default()))
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"));

    let rocket = rocket::custom(figment);

    let config: AppConfig = rocket.figment().extract().expect("config");
    let expire = config.expire_png_secs;
    let store = config.store.clone();

    info!("Using {store:?} as store");
    std::env::set_current_dir(config.store).unwrap();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(12 * 60));
        loop {
            interval.tick().await;
            match trim_expired_files(&store, expire) {
                Ok(()) => (),
                Err(e) => {
                    error!("Error while scanning: {e:?}")
                }
            };
        }
    });

    rocket
        .mount("/", routes![index])
        .mount(
            "/images",
            routes![
                get_image_file,
                create_file_with_png,
                attach_resource,
                delete_file,
                update_file
            ],
        )
        .register("/", catchers![internal_error, not_found, default])
        .attach(AdHoc::config::<AppConfig>())
}
