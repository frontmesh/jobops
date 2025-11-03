use crate::api::posts::get_posts;
use crate::components::post::PostList;
use crate::components::MetaTags;
use leptos::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = create_local_resource(
        || (), 
        |_| async move { get_posts().await }
    );

    view! {
        <MetaTags 
            title="Blog Posts - Rust Board".to_string()
            description="Read our latest blog posts about job opportunities and career development".to_string()
            url="/blog".to_string()
            image="https://example.com/default-image.jpg".to_string()
        />
        <section class="max-w-3xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-8">"Blog Posts"</h1>
            <Suspense
                fallback=move || view! {
                    <div class="animate-pulse">
                        <div class="h-4 bg-gray-200 rounded w-3/4 mb-4"></div>
                        <div class="h-4 bg-gray-200 rounded w-2/4 mb-4"></div>
                    </div>
                }
            >
                {move || posts.get().map(|result| match result {
                    Ok(posts) => view! { <PostList posts=posts/> }.into_view(),
                    Err(e) => view! {
                        <div class="bg-red-50 text-red-600 p-4 rounded">
                            <p>"Error loading posts: " {e.to_string()}</p>
                        </div>
                    }.into_view()
                })}
            </Suspense>
        </section>
    }
}
