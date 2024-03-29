use std::{env, net::SocketAddr};

use axum::{routing::get, Router};
use content::badges::Badges;
use tera::Tera;

use crate::content::{PostFrontmatter, badges::get_badge_data};

mod code_highlighting;
mod content;
mod errors;
mod markdown;
mod routes;
mod templates;

lazy_static::lazy_static! {
    static ref POSTS: Vec<(PostFrontmatter, String)> = content::find_posts().expect("TODO");
    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
    static ref BADGES: Badges = get_badge_data();

}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let serve_env = env::var("SERVE_ENV").unwrap_or("dev".to_owned());

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/about", get(routes::about))
        .route("/posts", get(routes::get_posts))
        .route("/posts/:post_alias", get(routes::get_post))
        //.route("/:file", get(routes::get_static_content))
        .nest(
            "/debug",
            Router::new()
                .route("/fail", get(routes::fail)),
        );

    let app = app.fallback(routes::fallback);

    let addr = match serve_env == "prod" {
        true => SocketAddr::from(([0, 0, 0, 0], 3000)),
        false => SocketAddr::from(([127, 0, 0, 1], 3000)),
    };

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
