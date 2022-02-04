use crate::identifier::FileId;
use color_eyre::Result;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use filetime::FileTime;
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

#[macro_use]
extern crate rocket;

mod identifier;
mod render;

#[get("/")]
fn index() -> &'static str {
    "Post SVGs and then request renderings in other formats (currently only png)

    USAGE

     GET /
        -> this help content

     GET /images/<name>
        -> load the image with correct etag caching set. Use accept header to change type

     PUT /images/<name>
        -> Put a new svg on top of an existing filename

     POST /images
        -> POST new svg to images, get redirected to image if valid or error (creates new)

     DELETE /images/<name>
        -> Remove the image from the system

    "
}

#[get("/<filename..>")]
async fn get_image_file(filename: PathBuf) -> std::result::Result<NamedFile, Status> {
    let mut png_path = std::path::PathBuf::new();
    png_path.push("data");
    png_path.push(filename);
    png_path.push("img");
    let mut svg_path = png_path.clone();
    png_path.set_extension("png");
    svg_path.set_extension("svg");

    if png_path.is_file() {
        NamedFile::open(&png_path)
            .await
            .map_err(|_| Status::InternalServerError)
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

#[post("/", format = "image/svg+xml", data = "<file>")]
async fn create_file_with_png(file: Vec<u8>) -> std::result::Result<Redirect, Status> {
    let id = FileId::new(&file);
    let mut png_path = std::path::PathBuf::new();
    png_path.push("data");
    png_path.push(id.dir());
    png_path.push(id.name());
    if let Err(e) = std::fs::create_dir_all(&png_path) {
        error!("Failed to create a directory: {e:?}");
        return Err(Status::InternalServerError);
    }

    png_path.push("img");
    png_path.set_extension("png");
    match render::svg_to_png(&file, &png_path) {
        Ok(()) => {
            let dir = id.dir();
            let name = id.name();
            let res = Redirect::to(format!("/images/{dir}/{name}"));
            Ok(res)
        }
        Err(e) => Err(e),
    }
}

#[delete("/<filename..>")]
async fn delete_file(filename: PathBuf) -> &'static str {
    let mut png_path = std::path::PathBuf::new();
    png_path.push("data");
    png_path.push(filename);

    match std::fs::remove_dir(png_path) {
        Ok(_) => "Ok",
        Err(e) => {
            error!("while removing file: {e:?}");
            "Error"
        }
    }
}

#[put("/<path>/<filename>", format = "image/svg+xml", data = "<file>")]
async fn update_file(
    path: PathBuf,
    filename: PathBuf,
    file: Vec<u8>,
) -> std::result::Result<Redirect, Status> {
    let mut png_path = std::path::PathBuf::new();
    png_path.push("data");
    png_path.push(&path);
    png_path.push(&filename);

    if let Err(e) = std::fs::create_dir_all(&png_path) {
        error!("Failed to create a directory: {e:?}");
        return Err(Status::InternalServerError);
    }

    png_path.push("img");
    png_path.set_extension("png");
    match render::svg_to_png(&file, &png_path) {
        Ok(()) => {
            let path = path.to_string_lossy();
            let filename = filename.to_string_lossy();
            let res = Redirect::to(format!("/images/{path}/{filename}"));
            Ok(res)
        }
        Err(e) => Err(e),
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
        if let Some(ext) = path.extension() {
            if ext == "png" {
                let metadata = std::fs::metadata(path)?;
                let mtime = FileTime::from_last_modification_time(&metadata);
                if FileTime::now().seconds() > (mtime.seconds() + expire) {
                    std::fs::remove_file(path)?;
                }
            }
        }
    }
    Ok(())
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
                delete_file,
                update_file
            ],
        )
        .attach(AdHoc::config::<AppConfig>())
}
