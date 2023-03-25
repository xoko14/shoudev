use std::{net::SocketAddr, fs::{self, File}, io::Read};

use axum::{Router, routing::get, response::{Html, IntoResponse}, http::{StatusCode, header}, extract::Path};
use tera::Tera;

use crate::content::PostFrontmatter;

mod content;
mod templates;
mod markdown;
mod code_highlighting;
mod errors;

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

    dotenv::dotenv().ok();
 
    let app = Router::new()
        .route("/", get(root))
        .route("/:file", get(get_static_content))
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

    let html_output = markdown::parse(&post_body);
    
    templates::render_post(post, &html_output)
}

async fn get_static_content(Path(file_name): Path<String>) -> impl IntoResponse{
    let response = match File::open(format!("content/static/{}", file_name)){
        Ok(mut f) => {
            let mut file = String::new();
            _ = f.read_to_string(&mut file);
            let file_type = match mime_guess::from_path(file_name).first(){
                Some(mime) => mime.to_string(),
                None => "text/plain".to_owned()
            };
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, file_type)],
                file,
            )
        },
        Err(_) => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain".to_string())],
            String::new()
        )
    };

    response
}
