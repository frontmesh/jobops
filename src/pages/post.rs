use crate::api::posts::get_post;
use crate::components::post::BlogPost;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
    pub slug: Option<String>,
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();

    let post = create_local_resource(
        move || {
            params.with(|p| match p.as_ref() {
                Ok(params) => params.slug.clone().unwrap_or_default(),
                Err(_) => String::new(),
            })
        },
        move |slug| async { get_post(slug).await },
    );

    view! {
        <section class="max-w-3xl mx-auto px-4 py-8">
            <Suspense fallback=move || view! {
                <div class="animate-pulse">
                    <div class="h-8 bg-gray-200 rounded w-3/4 mb-4"></div>
                    <div class="h-4 bg-gray-200 rounded w-1/4 mb-4"></div>
                    <div class="h-4 bg-gray-200 rounded w-2/4 mb-4"></div>
                </div>
            }>
                {move || {
                    post.get().map(|result| match result {
                        Ok(Some(post)) => view! { <BlogPost post=post /> }.into_view(),
                        Ok(None) => view! { <h1>"Post not found"</h1> }.into_view(),
                        Err(e) => view! { <p>"Error: " {e.to_string()}</p> }.into_view()
                    }).unwrap_or_else(|| view! { <div>"Loading..."</div> }.into_view())
                }}
            </Suspense>
        </section>
    }
}
