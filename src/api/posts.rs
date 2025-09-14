#![allow(unused)]
use crate::models::Post;
use leptos::server;
use markdown::to_html;
use pulldown_cmark::{html, Options, Parser};

use leptos::ServerFnError;
use std::path::PathBuf;


const POSTS_DIR: &str = "content/blog";

pub fn extract_date(path: &PathBuf) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .map(|name| name.split('-').take(3).collect::<Vec<&str>>().join("-"))
        .unwrap_or_default()
}

pub fn extract_between_markers(content: &str, marker: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = content.split(marker).collect();
    parts.get(1..3).map(|p| {
        let frontmatter = p[0].trim().to_string();
        let markdown_content = p[1].trim();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown_content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        (frontmatter, html_output)
    })
}

fn extract_aliases(frontmatter: &str) -> Vec<String> {
    frontmatter
        .lines()
        .find_map(|line| parse_frontmatter_line(line, "aliases"))
        .unwrap_or_default()
}

fn get_slug_from_aliases(aliases: &[String]) -> String {
    aliases
        .first()
        .map(|alias| {
            alias
                .trim_start_matches('/')
                .trim_end_matches('/')
                .to_string()
        })
        .unwrap_or_default()
}

pub fn parse_frontmatter_line(line: &str, prefix: &str) -> Option<Vec<String>> {
    line.contains(&format!("{} = [", prefix))
        .then(|| {
            line.split('[').nth(1).map(|s| {
                s.trim_end_matches(']')
                    .split(',')
                    // .map(|s| s.trim_matches('"').trim().to_string())
                    .map(|s| s.trim().trim_matches('"').trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
        })
        .flatten()
}

pub fn extract_title(frontmatter: &str) -> Option<String> {
    frontmatter
        .lines()
        .find(|line| line.starts_with("title = "))
        .map(|line| {
            line.trim_start_matches("title = ")
                .trim_matches('"')
                .to_string()
        })
}

fn parse_post_content(content: &str) -> Option<Post> {
    let (frontmatter, content) = extract_between_markers(content, "+++")?;

    let title = extract_title(&frontmatter)?;
    let tags = frontmatter
        .lines()
        .find_map(|line| parse_frontmatter_line(line, "tags"))
        .unwrap_or_default();
    let category = frontmatter
        .lines()
        .find_map(|line| parse_frontmatter_line(line, "category"))
        .unwrap_or_default();
    let aliases = extract_aliases(&frontmatter);
    let slug = get_slug_from_aliases(&aliases);

    // If no alias is found, create slug from title
    let slug = if slug.is_empty() {
        title.to_lowercase().replace(" ", "-")
    } else {
        slug
    };

    Some(Post {
        title,
        content,
        date: String::new(),
        tags,
        category,
        slug,
        image: None,
    })
}

#[cfg(feature = "ssr")]
#[server(GetPosts)]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    // In a Cloudflare Workers environment, we cannot access the file system.
    // You would typically:
    // 1. Embed posts at compile time using include_str! macro
    // 2. Fetch posts from a remote API or storage service
    // 3. Use Cloudflare KV storage
    
    // For now, returning an empty vector to make compilation work
    // TODO: Implement proper post loading mechanism for Cloudflare Workers
    Ok(Vec::new())
}

#[cfg(not(feature = "ssr"))]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    Err(ServerFnError::ServerError("get_posts is only available in SSR mode".to_string()))
}

#[server(GetPost)]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError> {
    get_post_sync(slug)
}

#[cfg(feature = "ssr")]
pub fn get_post_sync(slug: String) -> Result<Option<Post>, ServerFnError> {
    // In a Cloudflare Workers environment, we cannot access the file system.
    // You would typically:
    // 1. Embed posts at compile time using include_str! macro
    // 2. Fetch posts from a remote API or storage service
    // 3. Use Cloudflare KV storage
    
    // For now, returning None to make compilation work
    // TODO: Implement proper post loading mechanism for Cloudflare Workers
    let _ = slug; // Suppress unused variable warning
    Ok(None)
}

#[cfg(not(feature = "ssr"))]
pub fn get_post_sync(_slug: String) -> Result<Option<Post>, ServerFnError> {
    Err(ServerFnError::ServerError("get_post_sync is only available in SSR mode".to_string()))
}
