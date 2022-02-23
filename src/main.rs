use crate::types::SvgDescription;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use std::{env, path, result};

use rocket::{
    fairing::AdHoc,
    form::Form,
    http::{ContentType, Status},
    serde::{
        json::{json, Value},
        Deserialize, Serialize,
    },
    Request,
};

#[macro_use]
extern crate rocket;

mod apikey;
mod index;
mod instrumentation;
mod render;
#[cfg(test)]
mod tests;
mod types;

#[post("/image", format = "multipart/form-data", data = "<svg_form>")]
async fn render_svg(
    svg_form: Form<SvgDescription<'_>>,
    _api_key: apikey::ApiKey<'_>,
) -> result::Result<(ContentType, Vec<u8>), Status> {
    let result = render::png_from_svg(svg_form.into_inner())
        .await
        .map_err(|e| {
            error!("Error while rendering: {e:?}");
            Status::InternalServerError
        })?;
    Ok((ContentType::PNG, result))
}

#[derive(Deserialize, Serialize)]
struct AppConfig {
    key: String,
    temp_path: path::PathBuf,
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            key: "default".into(),
            temp_path: "/tmp".into(),
        }
    }
}

#[catch(500)]
fn internal_error() -> Value {
    json!({"error": "internal_error"})
}

#[catch(404)]
fn not_found(_req: &Request) -> Value {
    json!({"error": "not_found"})
}

#[catch(default)]
fn default(status: Status, _req: &Request) -> Value {
    json!({"status": status.code, "reason": status.reason() })
}

#[launch]
fn rocket() -> _ {
    color_eyre::install().unwrap();
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(AppConfig::default()))
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"));

    instrumentation::init_logging();

    let rocket = rocket::custom(figment);

    let prometheus = rocket_prometheus::PrometheusMetrics::new();

    let config: AppConfig = rocket.figment().extract().expect("config");

    std::fs::create_dir_all(&config.temp_path).expect("failed to create temp_path directories");
    env::set_current_dir(config.temp_path).expect("failed to set PWD to temp_path. check config");

    rocket
        .mount("/", routes![index::index, render_svg])
        .mount("/metrics", prometheus.clone())
        .register("/", catchers![internal_error, not_found, default])
        .attach(prometheus)
        .attach(instrumentation::TracingFairing)
        .attach(AdHoc::config::<AppConfig>())
}
