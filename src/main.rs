use std::net::SocketAddr;

use axum::{Router, routing::get, Json, response::Html};
use tera::{Tera, Context};

use crate::content::PostFrontmatter;

mod content;

lazy_static::lazy_static!{
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
}

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();
 
    let app = Router::new()
        .route("/", get(root))
        .nest("/debug", 
            Router::new().route("/posts", get(debug_posts))
        );

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    tracing::info!("listening on http://{}", addr);
    _ = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;
}

async fn root() -> &'static str{
    "hello axum!"
}

async fn debug_posts() -> Html<String>{
    let posts: Vec<PostFrontmatter> = POSTS.iter()
        .map(|p| {
        tracing::info!("{}", p.1);
        return p.0.clone();
    }).collect();

    let mut tpl_ctx = Context::new();
    tpl_ctx.insert("posts", &posts);

    let rendered = TEMPLATES.render("posts_test.html", &tpl_ctx).expect("rendering error");
    Html(rendered)
}
