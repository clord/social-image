use crate::identifier::FileId;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Redirect;
use std::path::PathBuf;

#[macro_use]
extern crate rocket;

mod identifier;
mod render;

#[get("/")]
fn index() -> &'static str {
    "Post SVGs and then request renderings in other formats (currently png)

    USAGE

     GET /
        -> this help content

     GET /images/<name>
        -> load the image with correct etag caching set. Use accept header to change type

     PUT /images/<name>
        -> Put a new svg on top of an existing filename

     POST /images
        -> POST svg to images, get redirected to image if valid or error

     DELETE /images/<name>
        -> Remove the image from the system

    "
}

#[get("/<filename..>")]
async fn get_image_file(filename: PathBuf) -> Option<NamedFile> {
    let mut png_path = std::path::PathBuf::new();
    png_path.push("cache");
    png_path.push(filename);
    png_path.push("img");
    png_path.set_extension("png");
    NamedFile::open(&png_path).await.ok()
}

#[post("/", format = "image/svg+xml", data = "<file>")]
async fn create_file(file: Vec<u8>) -> std::result::Result<Redirect, Status> {
    let id = FileId::new(&file);
    let mut png_path = std::path::PathBuf::new();
    png_path.push("cache");
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
    png_path.push("cache");
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
    png_path.push("cache");
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/images",
        routes![get_image_file, create_file, delete_file, update_file],
    )
}
