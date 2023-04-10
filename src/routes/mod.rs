use std::{
    fs::{self, File},
    io::Read,
};

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
};

use crate::{content::PostFrontmatter, errors::ShoudevError, markdown, templates, POSTS};

pub async fn root() -> (StatusCode, Html<String>) {
    let homepage = fs::read_to_string("content/pages/homepage.md").unwrap_or("oopsies".to_owned());
    let html_output = markdown::parse(&homepage);
    templates::render_homepage(&html_output)
}

pub async fn debug_posts() -> (StatusCode, Html<String>) {
    let posts: Vec<PostFrontmatter> = POSTS
        .iter()
        .map(|p| {
            tracing::info!("{}", p.1);
            return p.0.clone();
        })
        .collect();

    templates::render_posts(&posts)
}

pub async fn debug_post(Path(post_alias): Path<String>) -> (StatusCode, Html<String>) {
    let (post, post_dir) = match POSTS.iter().find(|item| item.0.alias == post_alias) {
        Some(p) => p,
        None => {
            return templates::error_404().await;
        }
    };

    let post_body = match fs::read_to_string(post_dir) {
        Ok(b) => b,
        Err(_) => return templates::error_404().await,
    };

    let html_output = markdown::parse(&post_body);

    templates::render_post(post, &html_output)
}

pub async fn get_static_content(Path(file_name): Path<String>) -> impl IntoResponse {
    let response = match File::open(format!("static/{}", file_name)) {
        Ok(mut f) => {
            let mut file = String::new();
            _ = f.read_to_string(&mut file);
            let file_type = match mime_guess::from_path(file_name).first() {
                Some(mime) => mime.to_string(),
                None => "text/plain".to_owned(),
            };
            (StatusCode::OK, [(header::CONTENT_TYPE, file_type)], file)
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain".to_string())],
            String::new(),
        ),
    };

    response
}

pub async fn fail() -> Result<(), ShoudevError> {
    Err(ShoudevError::err())
}
