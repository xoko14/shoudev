use std::fs;


#[derive(knuffel::Decode, Clone, serde::Serialize)]
pub struct Badges{
    #[knuffel(children(name="badge"))]
    badges: Vec<Badge>
}

#[derive(knuffel::Decode, Clone, serde::Serialize)]
pub struct Badge{
    #[knuffel(property)]
    pub file: String,
    #[knuffel(property)]
    pub href: Option<String>
}


impl IntoIterator for Badges{
    type Item = Badge;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.badges.into_iter()
    }
}

pub fn get_badge_data() -> Badges{
    let badges_kdl = include_str!("../../content/badges.kdl");

    knuffel::parse("badges.kdl", badges_kdl).unwrap()
}