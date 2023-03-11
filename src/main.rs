use std::{net::SocketAddr, fs};

use axum::{Router, routing::get, response::Html, http::StatusCode, extract::Path};
use pulldown_cmark::{Parser, html};
use tera::Tera;

use crate::content::PostFrontmatter;

mod content;
mod templates;

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
            Router::new()
            .route("/posts", get(debug_posts))
            .route("/post/:post_alias", get(debug_post))
        );
    
    let app = app.fallback(templates::error_404);

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    tracing::info!("listening on http://{}", addr);
    _ = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;
}

async fn root() -> &'static str{
    "hello axum!"
}

async fn debug_posts() -> (StatusCode, Html<String>){
    let posts: Vec<PostFrontmatter> = POSTS.iter()
        .map(|p| {
        tracing::info!("{}", p.1);
        return p.0.clone();
    }).collect();

    templates::render_posts(&posts)
}

async fn debug_post(Path(post_alias): Path<String>) -> (StatusCode, Html<String>){
    let (post, post_dir) = match POSTS.iter().find(|item| item.0.alias ==  post_alias){
        Some(p) => p,
        None => {return templates::error_404().await;}
    };

    let post_body = match fs::read_to_string(post_dir){
        Ok(b) => b,
        Err(_) => {return templates::error_404().await}
    };

    let parser = Parser::new(&post_body);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    templates::render_post(post, &html_output)
}
