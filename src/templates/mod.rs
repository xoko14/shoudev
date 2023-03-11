use axum::{http::StatusCode, response::Html};
use tera::Context;

use crate::{content::PostFrontmatter, TEMPLATES};

type HtmlResponse = (StatusCode, Html<String>);

pub fn render_posts(posts: &Vec<PostFrontmatter>) -> HtmlResponse{
    let mut tpl_ctx = Context::new();
    tpl_ctx.insert("posts", &posts);

    let rendered = TEMPLATES.render("posts_test.html", &tpl_ctx).expect("rendering error");
    (StatusCode::OK, Html(rendered))
}

pub fn render_post(post: &PostFrontmatter, body: &str) -> HtmlResponse{
    let mut tpl_ctx = Context::new();
    tpl_ctx.insert("post", post);
    tpl_ctx.insert("post_body", body);

    let rendered = TEMPLATES.render("post_test.html", &tpl_ctx).expect("error");
    (StatusCode::OK, Html(rendered))
}

pub async fn error_404() -> HtmlResponse{
    let rendered = TEMPLATES.render("error_pages/404.html", &Context::new()).expect("error");
    (StatusCode::NOT_FOUND, Html(rendered))
}