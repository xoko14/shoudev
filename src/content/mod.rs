use std::fs;

use ignore::{types::TypesBuilder, WalkBuilder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostFrontmatter {
    pub title: String,
    pub description: String,
    pub date_posted: String,
    pub tags: Vec<String>,
    pub author: String,
    pub alias: String
}

pub fn find_posts() -> Result<Vec<(PostFrontmatter, String)>, ()>{
    let mut t = TypesBuilder::new();
    t.add_defaults();
    let toml = match t.select("toml").build(){
        Ok(t) => t,
        Err(_) => return Err(())
    };

    let file_walker = WalkBuilder::new("./content/posts/").types(toml).build();
    let mut frontmatters = Vec::<(PostFrontmatter, String)>::new();
    for frontmatter in file_walker{
        match frontmatter{
            Ok(f) => {
                if f.path().is_dir() {continue;}
                let fm_raw = fs::read_to_string(f.path()).expect("TODO");
                let fm: PostFrontmatter = toml::from_str(&fm_raw).expect("TODO");
                let post_location = format!("{}/post.md", f.path().parent().expect("TODO").to_str().expect("TODO"));
                frontmatters.push((fm, post_location))
            },
            Err(_) => return Err(())
        };
    }

    return Ok(frontmatters);
}
