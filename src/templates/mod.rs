use axum::{http::StatusCode, response::Html};
use tera::Context;

use crate::{content::PostFrontmatter, errors::ShoudevError, TEMPLATES, BADGES};

type HtmlResponse = (StatusCode, Html<String>);

pub fn render_posts(posts: &Vec<PostFrontmatter>) -> HtmlResponse {
    let mut tpl_ctx = get_common_context();
    tpl_ctx.insert("posts", &posts);

    let rendered = TEMPLATES
        .render("posts_test.html", &tpl_ctx)
        .expect("rendering error");
    (StatusCode::OK, Html(rendered))
}

pub fn render_post(post: &PostFrontmatter, body: &str) -> HtmlResponse {
    let mut tpl_ctx = get_common_context();
    tpl_ctx.insert("post", post);
    tpl_ctx.insert("post_body", body);

    let rendered = TEMPLATES.render("post_test.html", &tpl_ctx).expect("error");
    (StatusCode::OK, Html(rendered))
}

pub fn render_homepage(body: &str) -> HtmlResponse {
    let mut tpl_ctx = get_common_context();
    tpl_ctx.insert("page_body", body);

    let rendered = TEMPLATES.render("homepage.html", &tpl_ctx).expect("error");
    (StatusCode::OK, Html(rendered))
}

pub fn render_generic(title: &str, body: &str) -> Result<HtmlResponse, ShoudevError>{
    let mut tpl_ctx = get_common_context();
    tpl_ctx.insert("page_title", title);
    tpl_ctx.insert("page_body", body);

    let rendered = TEMPLATES.render("generic.html", &tpl_ctx)?;
    Ok((StatusCode::OK, Html(rendered)))
}

pub fn error_404() -> Result<HtmlResponse, ShoudevError> {
    let rendered = TEMPLATES.render("error_pages/404.html", &get_common_context())?;
    Ok((StatusCode::NOT_FOUND, Html(rendered)))
}

pub fn error_500() -> Result<HtmlResponse, ShoudevError> {
    let rendered = TEMPLATES.render("error_pages/500.html", &get_common_context())?;
    Ok((StatusCode::INTERNAL_SERVER_ERROR, Html(rendered)))
}

fn get_common_context() -> Context{
    let mut context = Context::new();
    let badges = BADGES.clone().into_iter().collect::<Vec<_>>();

    context.insert("badges",&badges);
    context
}