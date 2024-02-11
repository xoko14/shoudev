use std::fs;

use ignore::{types::TypesBuilder, WalkBuilder};
use serde::{Serialize, Deserialize};
use yaml_front_matter::{Document, YamlFrontMatter};

pub mod badges;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostFrontmatter {
    pub title: String,
    pub description: String,
    pub date_posted: String,
    pub tags: Vec<String>,
    pub author: String,
    pub alias: String
}

impl Default for PostFrontmatter {
    fn default() -> Self {
        Self { title: Default::default(), description: Default::default(), date_posted: Default::default(), tags: Default::default(), author: Default::default(), alias: Default::default() }
    }
}

pub fn find_posts() -> Result<Vec<(PostFrontmatter, String)>, ()>{
    let mut t = TypesBuilder::new();
    t.add_defaults();
    let md = match t.select("markdown").build(){
        Ok(t) => t,
        Err(_) => return Err(())
    };

    let file_walker = WalkBuilder::new("./content/posts/").types(md).build();
    let mut frontmatters = Vec::<(PostFrontmatter, String)>::new();
    for frontmatter in file_walker{
        match frontmatter{
            Ok(f) => {
                if f.path().is_dir() {continue;}
                let fm_raw = fs::read_to_string(f.path()).expect("TODO");
                let fm: Document<PostFrontmatter> = YamlFrontMatter::parse(&fm_raw).unwrap();
                let post_location = f.path().to_str().expect("TODO").to_owned();
                let mut frontmatter = fm.metadata;
                frontmatter.alias = f.file_name().to_str().expect("TODO").split(".").next().expect("TODO").to_owned();
                frontmatters.push((frontmatter, post_location))
            },
            Err(_) => return Err(())
        };
    }

    return Ok(frontmatters);
}
