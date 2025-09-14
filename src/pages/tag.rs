use crate::api::posts::get_posts;
use crate::components::post::PostList;
use leptos::*;
use leptos_router::*;

#[component]
pub fn TagPage() -> impl IntoView {
    let params = use_params_map();
    let tag = move || params.with(|params| params.get("tag").cloned().unwrap_or_default());
    let posts = create_resource(|| (), |_| async move { get_posts().await });

    view! {
        <div class="max-w-3xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-8">
                {"Posts tagged with "}
                <span class="text-blue-600">{"#"}{move || tag()}</span>
            </h1>

            <Suspense
                fallback=move || view! {
                    <div class="animate-pulse">
                        <div class="h-4 bg-gray-200 rounded w-3/4 mb-4"></div>
                        <div class="h-4 bg-gray-200 rounded w-2/4 mb-4"></div>
                    </div>
                }
            >
                {move || posts.get().map(|result| match result {
                    Ok(all_posts) => {
                        // Filter posts by tag
                        let filtered_posts = all_posts.into_iter()
                            .filter(|post| post.tags.contains(&tag()))
                            .collect::<Vec<_>>();

                        if filtered_posts.is_empty() {
                            view! {
                                <p class="text-gray-600">{"No posts found with this tag."}</p>
                            }.into_view()
                        } else {
                            view! {
                                <PostList posts=filtered_posts/>
                            }.into_view()
                        }
                    }
                    Err(e) => view! {
                        <div class="bg-red-50 text-red-600 p-4 rounded">
                            <p>"Error loading posts: " {e.to_string()}</p>
                        </div>
                    }.into_view()
                })}
            </Suspense>
        </div>
    }
}
