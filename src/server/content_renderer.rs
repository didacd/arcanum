use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Path;

// --- Your Structs (Frontmatter, Page) remain the same ---
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
    pub path: String,
    pub filename: String,
    pub metadata: Frontmatter,
    pub content: String,
    pub rendered_html: String,
}
// ---------------------------------------------------------

pub fn read_markdown_files(dir: &Path) -> io::Result<Vec<Page>> {
    let mut pages = Vec::new();
    let matter = Matter::<YAML>::new();

    // This part remains the same. If the directory itself can't be read,
    // it's a fatal error and we should return an Err.
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |e| e == "md") {
            // Read the content of a single file. An I/O error here is also fatal.
            let file_content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Warning: Could not read file {:?}. Error: {}", path, e);
                    continue; // Skip to the next file
                }
            };

            // we use a `match` to handle success and failure for this file individually.
            match matter.parse::<Frontmatter>(&file_content) {
                Ok(parsed_entity) => {
                    // Parsing was successful. Now check if frontmatter was deserialized.
                    if let Some(metadata) = parsed_entity.data {
                        // Success! We have a valid page.
                        let markdown_content = parsed_entity.content;

                        let mut options = Options::empty();
                        options.insert(Options::ENABLE_STRIKETHROUGH);
                        let parser = Parser::new_ext(&markdown_content, options);

                        let mut rendered_html = String::new();
                        html::push_html(&mut rendered_html, parser);

                        let page = Page {
                            path: path.to_str().unwrap_or("").to_string(),
                            filename: path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("")
                                .to_string(),
                            metadata,
                            content: markdown_content.to_string(),
                            rendered_html,
                        };
                        // Add the valid page to our list.
                        pages.push(page);
                    } else {
                        // The file parsed, but the frontmatter was empty or invalid.
                        eprintln!(
                            "Warning: Skipping file {:?} due to missing or invalid frontmatter.",
                            path
                        );
                    }
                }
                Err(e) => {
                    // The file itself could not be parsed by gray-matter (e.g., malformed YAML).
                    eprintln!(
                        "Warning: Skipping file {:?} due to a parsing error: {}",
                        path, e
                    );
                }
            }
            // The loop continues to the next file regardless of the outcome.
        }
    }
    // The function now always returns Ok with a list of the pages that succeeded.
    Ok(pages)
}
