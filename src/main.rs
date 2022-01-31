use std::path::PathBuf;
use rocket::fs::TempFile;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "social-image"
}

#[get("/<filename..>")]
async fn get_file(filename: PathBuf) -> String {
    format!("hey: ")
}

#[post("/<filename..>", format = "image/svg+xml", data = "<file>")]
async fn create_file(filename: PathBuf, file: TempFile<'_>) -> String {
    format!("create: ")
}

#[delete("/<filename..>")]
async fn delete_file(filename: PathBuf) -> String {
    format!("update: ")
}

#[put("/<filename..>")]
async fn update_file(filename: PathBuf) -> String {
    format!("update: ")
}

#[launch]
fn rocket() -> _ {
    // GET /                    -> help
    // GET /images              -> list of images if you have the token
    // GET /images/filename     -> load the image with correct caching set
    // POST /images             -> POST svg to images, get redirected to image if valid or error
    // DELETE /images/filename  -> Remove the image from the system if owner or token permits
    // PUT /images/filename     -> replace the image with a new SVG (will expire etags)
    rocket::build().mount("/social", routes![index]).mount(
        "/social/images",
        routes![get_file, create_file, delete_file, update_file],
    )
}
