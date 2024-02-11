use std::{
    fs::{self, File},
    io::Read,
};

use axum::{
    extract::{Path, Request},
    http::{header, StatusCode},
    response::{Html, IntoResponse},
};
use tracing_subscriber::fmt::format;

use crate::{
    content::PostFrontmatter,
    errors::{ShoudevError, ShoudevErrorType},
    markdown, templates, POSTS,
};

type HtmlResponse = (StatusCode, Html<String>);

pub async fn root() -> Result<HtmlResponse, ShoudevError> {
    let homepage = fs::read_to_string("content/pages/homepage.md")?;
    let html_output = markdown::parse(&homepage);
    let response = templates::render_homepage(&html_output);
    Ok(response)
}

pub async fn about() -> Result<HtmlResponse, ShoudevError>{
    tracing::info!("about");
    let aboutpage = fs::read_to_string("content/pages/about.md")?;
    let html_output = markdown::parse(&aboutpage);
    let response = templates::render_generic("About", &html_output)?;
    Ok(response)
}

pub async fn debug_posts() -> Result<HtmlResponse, ShoudevError> {
    let posts: Vec<PostFrontmatter> = POSTS
        .iter()
        .map(|p| {
            tracing::info!("{}", p.1);
            return p.0.clone();
        })
        .collect();

    let response = templates::render_posts(&posts);
    Ok(response)
}

pub async fn debug_post(Path(post_alias): Path<String>) -> Result<HtmlResponse, ShoudevError> {
    let (post, post_dir) = match POSTS.iter().find(|item| item.0.alias == post_alias) {
        Some(p) => p,
        None => {
            return Err(ShoudevError::new(
                ShoudevErrorType::PostNotFound,
                StatusCode::NOT_FOUND,
            ));
        }
    };

    let post_body = match fs::read_to_string(post_dir) {
        Ok(b) => b,
        Err(_) => {
            return Err(ShoudevError::new(
                ShoudevErrorType::PostNotFound,
                StatusCode::NOT_FOUND,
            ))
        }
    };

    let html_output = markdown::parse(&post_body);

    let result = templates::render_post(post, &html_output);
    Ok(result)
}

pub async fn get_static_content(
    Path(file_name): Path<String>,
) -> Result<impl IntoResponse, ShoudevError> {
    tracing::info!("Requested resource {}", file_name);
    let mut file = String::new();
    File::open(format!("static/{}", file_name))?.read_to_string(&mut file)?;

    let file_type = match mime_guess::from_path(file_name).first() {
        Some(mime) => mime.to_string(),
        None => "text/plain".to_owned(),
    };
    let response = (StatusCode::OK, [(header::CONTENT_TYPE, file_type)], file);

    Ok(response)
}

pub async fn fail() -> Result<(), ShoudevError> {
    Err(ShoudevError::err())
}

pub async fn fallback(request: Request) -> Result<impl IntoResponse, ShoudevError> {
    let path = request.uri();
    let mut file = Vec::new();
    tracing::info!("requested resource static{}", path);
    File::open(format!("static{}", path))?.read_to_end(&mut file)?;

    let file_type = match mime_guess::from_path(path.to_string()).first() {
        Some(mime) => mime.to_string(),
        None => "text/plain".to_owned(),
    };

    let response = (StatusCode::OK, [(header::CONTENT_TYPE, file_type)], file);
    Ok(response)
}
