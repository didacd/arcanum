use crate::config::SiteConfig;
use askama::Template;

use super::content_renderer::{Frontmatter, Page};

// base.html takes extension from either of these two:

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub site: &'a SiteConfig,
    pub homepage_html: String,
    pub posts: Vec<Page>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub site: &'a SiteConfig,
    pub meta_data: Frontmatter,
    pub post: String,
    pub posts: Vec<Page>,
}
