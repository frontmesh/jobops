use crate::models::Post;
use leptos::*;
use leptos_router::*;

#[component]
pub fn PostPreview(post: Post) -> impl IntoView {
    view! {
        <li class="border-b border-gray-200 pb-6 last:border-0">
            <A
                href={format!("/blog/{}", post.slug)}
                class="group block"
            >
                <h2 class="text-xl font-semibold text-gray-900 group-hover:text-blue-600 transition-colors">
                    {&post.title}
                </h2>
                <div class="text-sm text-gray-600 mt-2">
                    {&post.date}
                    {post.category.first().map(|cat| view! {
                        <span>" • "{cat}</span>
                    })}
                </div>
            </A>
                {(!post.tags.is_empty()).then(|| view! {
                    <div class="flex flex-wrap gap-2 mt-3">
                        {post.tags.iter().map(|tag|{
                            let tag = tag.to_owned();
                            view! {
                            <A
                                href={format!("/tag/{}", tag)}
                                class="text-sm bg-gray-100 px-2 py-1 rounded hover:bg-gray-200 transition-colors"
                            >
                                {"#"}{tag}
                            </A>
                        }}).collect::<Vec<_>>()}
                    </div>
                })}
        </li>
    }
}
