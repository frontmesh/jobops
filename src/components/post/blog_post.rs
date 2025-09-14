use crate::models::Post;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    let tag_list = post.tags.clone();

    view! {
        <Meta property="og:title" content={post.title.clone()}/>
        // <Meta property="og:description" content={post.excerpt.clone().unwrap_or_default()}/>
        <Meta property="og:site_name" content="benw.is"/>
        <Meta property="og:locale" content="en-us"/>
        <Meta property="og:type" content="article"/>
        // <Meta property="og:url" content={post.excerpt.clone().unwrap_or_default()}/>
        <Meta name="twitter:title" content={post.title.clone()}/>
        <Meta name="twitter:site" content="@iambenwis"/>
        <Title text={post.title.clone()} />
        <Meta name="twitter:card" content="summary"/>
        // <Meta name="twitter:image" content="https://benwis.imgix.net/ben_catcarbon.jpeg"/>
        // <Meta name="twitter:description" content={post.excerpt.clone().unwrap_or_default()}/>
        // <Meta name="description" content={post.excerpt.clone().unwrap_or_default()}/>

        <article class="max-w-3xl mx-auto px-4 py-8">
            <h2 class="text-2xl font-bold mb-4">{post.title}</h2>

            <div class="text-sm text-gray-600 mb-8">
                <span class="mr-4">{post.date}</span>
                {post.category.first().map(|cat| view! {
                    <span class="mr-4">{"Category: "}{cat}</span>
                })}
                {move || {
                    (!tag_list.is_empty()).then(|| view! {
                        <div class="flex flex-wrap gap-2 mt-2">
                            {tag_list.iter().map(|tag| {
                                let tag = tag.to_string();
                                view! {
                                    <A
                                        href={format!("/tag/{}", tag)}
                                        class="text-sm bg-gray-100 px-2 py-1 rounded hover:bg-gray-200 transition-colors"
                                    >
                                        {"#"}{tag}
                                    </A>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    })
                }}
            </div>

            <div
                class="prose prose-lg max-w-none"
                inner_html={post.content}
            />
        </article>
    }
}
