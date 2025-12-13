mod content_renderer;
mod lib;
mod routes;
mod templates;

// Expose server modules
pub use content_renderer::read_markdown_files;
pub use routes::{blog, index, page_debug, about_me};
