use gray_matter::{Matter, engine::YAML};
use log::{error, warn};
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub author: String,
    pub created: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub filename: String,
    pub metadata: Frontmatter,
    pub rendered_html: String,
}

#[derive(Debug, Clone)]
pub struct PageSummary {
    pub filename: String,
    pub metadata: Frontmatter,
}

fn parse_markdown_file(path: &Path) -> Option<Page> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            warn!("Could not read file {:?}. Error: {}", path, e);
            return None;
        }
    };

    let matter = Matter::<YAML>::new();
    match matter.parse::<Frontmatter>(&file_content) {
        Ok(parsed_entity) => {
            if let Some(metadata) = parsed_entity.data {
                let markdown_content = parsed_entity.content;

                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                let parser = Parser::new_ext(&markdown_content, options);

                let mut rendered_html = String::new();
                html::push_html(&mut rendered_html, parser);

                Some(Page {
                    filename: path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string(),
                    metadata,
                    rendered_html,
                })
            } else {
                warn!(
                    "Skipping file {:?} due to missing or invalid frontmatter.",
                    path
                );
                None
            }
        }
        Err(e) => {
            error!("Skipping file {:?} due to a parsing error: {}", path, e);
            None
        }
    }
}

fn parse_markdown_summary(path: &Path) -> Option<PageSummary> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            warn!("Could not read file {:?}. Error: {}", path, e);
            return None;
        }
    };

    let matter = Matter::<YAML>::new();
    match matter.parse::<Frontmatter>(&file_content) {
        Ok(parsed_entity) => {
            if let Some(metadata) = parsed_entity.data {
                Some(PageSummary {
                    filename: path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string(),
                    metadata,
                })
            } else {
                warn!(
                    "Skipping file {:?} due to missing or invalid frontmatter.",
                    path
                );
                None
            }
        }
        Err(e) => {
            error!("Skipping file {:?} due to a parsing error: {}", path, e);
            None
        }
    }
}

pub fn read_markdown_files(dir: &Path) -> io::Result<Vec<PageSummary>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let pages = fs::read_dir(dir)?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|e| e == "md"))
        .filter_map(|path| parse_markdown_summary(&path))
        .collect();

    Ok(pages)
}

pub fn read_markdown_file(dir: &Path, filename: &str) -> Option<Page> {
    // Security: Prevent directory traversal
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        warn!("Potential path traversal attempt blocked: {}", filename);
        return None;
    }

    let path = dir.join(format!("{}.md", filename));
    if path.exists() && path.is_file() {
        parse_markdown_file(&path)
    } else {
        None
    }
}
