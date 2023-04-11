use std::{
    fs::{self, File},
    io::Read,
};

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
};

use crate::{
    content::PostFrontmatter,
    errors::{ShoudevError, ShoudevErrorType},
    markdown, templates, POSTS,
};

type HtmlResponse = (StatusCode, Html<String>);

pub async fn root() -> Result<HtmlResponse, ShoudevError> {
    let homepage = fs::read_to_string("content/pages/homepage.md").unwrap_or("oopsies".to_owned());
    let html_output = markdown::parse(&homepage);
    let response = templates::render_homepage(&html_output);
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

pub async fn fallback_404() -> Result<HtmlResponse, ShoudevError> {
    templates::error_404()
}
