use super::post_preview::PostPreview;
use crate::models::Post;
use leptos::*;

#[component]
pub fn PostList(posts: Vec<Post>) -> impl IntoView {
    view! {
        <ul class="space-y-6">
            {posts.into_iter().map(|post| view! {
                <PostPreview post=post/>
            }).collect::<Vec<_>>()}
        </ul>
    }
}
