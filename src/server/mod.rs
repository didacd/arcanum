mod content_renderer;
mod lib;
mod routes;
mod templates;

// Expose server modules
pub use routes::blog::{blog_post, index, about_me};
pub use routes::api::{debug_posts, get_post_json};
