use std::collections::HashMap;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponseBuilder};
use actix_web::http::StatusCode;
use dotenv::dotenv;
use std::env;
use log::{info, LevelFilter};

// Redirects configured via ENV variables
// Syntax: REDIRECT_<NAME>="<destination url>"
// Examples:
// REDIRECT_INDEX=https://team-revolution.net => / -> https://team-revolution.net
// REDIRECT_DDG=https://duckduckgo.com => /ddg -> https://duckduckgo.com

#[derive(Default)]
struct AppState {
    redirects: HashMap<String, String>,
    not_found_redirect: String,
}

impl AppState {
    async fn get_response(&self, link: String) -> impl Responder {
        let destination = match self.redirects.get(&link) {
            Some(destination) => {
                info!("Redirecting /{link} to {destination}");
                destination
            },
            None => {
                info!("Not found: /{link}");
                &self.not_found_redirect
            },
        };
        HttpResponseBuilder::new(StatusCode::PERMANENT_REDIRECT)
            .insert_header(("Location", destination.as_str())).await
    }
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    data.get_response("index".to_owned()).await
}

#[get("/{link}")]
async fn redirect(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let link = path.into_inner();
    data.get_response(link).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    HttpServer::new(|| {
        let not_found_redirect = env::var("NOT_FOUND_REDIRECT")
            .unwrap_or("https://en.wikipedia.org/wiki/Lp0_on_fire".to_owned());

        let mut redirects = HashMap::new();

        let env_marker = "REDIRECT_";

        for (key, value) in env::vars() {
            if key.starts_with(env_marker) {
                let redirect_source = key
                    .get(env_marker.len()..)
                    .and_then(|value| Some(value.to_lowercase()));

                if let Some(redirect_value) = redirect_source {
                    redirects.insert(redirect_value, value);
                }
            }
        }

        let app_state = web::Data::new(AppState {
            redirects,
            not_found_redirect,
        });

        App::new()
            .app_data(app_state)
            .service(index)
            .service(redirect)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}