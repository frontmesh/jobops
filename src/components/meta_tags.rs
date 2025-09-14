use leptos::*;
use leptos_meta::*;

#[component]
pub fn MetaTags(title: String, description: String, image: String, url: String) -> impl IntoView {
    view! {
        <Title text=title.clone() />
        <Meta name="description" content=description.clone() />
        <Meta name="og:title" content=title />
        <Meta name="og:description" content=description />
        <Meta name="og:image" content=image />
        <Meta name="og:url" content=url />
    }
}
