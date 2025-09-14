mod footer;
mod header;
mod meta_tags;

pub mod post;

mod fallback;
pub use fallback::Fallback;

pub use footer::Footer;
pub use header::Header;
pub use meta_tags::MetaTags;
