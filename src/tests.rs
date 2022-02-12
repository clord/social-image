use super::rocket;
use rocket::http::{ContentType,Status};
use rocket::local::blocking::Client;

#[test]
fn tests() {
    std::env::set_var("APP_KEY", "XO");
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    assert_eq!(client.get("/").dispatch().status(), Status::Ok);
    assert_eq!(
        client.get("/not-found").dispatch().status(),
        Status::NotFound
    );

    assert_eq!(
        client.post("/image")
         .header(ContentType::new("multipart", "form-data"))
        .dispatch().status(),
        Status::BadRequest
    );

    assert_eq!(
        client.post("/image")
         .header(ContentType::new("multipart", "form-data"))
         .header(ContentType::new("x-api-key", "XO"))
        .dispatch().status(),
        Status::BadRequest
    );

    // let req = client
    //     .post("/image")
    //     .header(ContentType::new("multipart", "form-data"))
    //     .header(Header::new("accept", "image/png"))
    //     .body(r#"{ "value": 42 }"#);

    // let response3 = req.dispatch();
    // assert_eq!(response3.status(), Status::BadRequest);
}
