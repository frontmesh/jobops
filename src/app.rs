use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{Header, Footer, Fallback};
use crate::pages::{HomePage, BlogPage, PostPage, TagPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link
            rel="preconnect"
            href="https://fonts.googleapis.com"
        />
        <Link
            rel="preconnect"
            href="https://fonts.gstatic.com"
            crossorigin="anonymous"
        />
        <Link
            href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700;800&display=swap"
            rel="stylesheet"
        />
        
        <Router>
            <div class="min-h-screen bg-gray-50 flex flex-col font-poppins">
                <Header/>
                <main class="flex-1">
                    <Routes>
                        <Route path="/" view=HomePage />
                        <Route path="/blog" view=BlogPage />
                        <Route path="/blog/:slug" view=PostPage />
                        <Route path="/tag/:tag" view=TagPage />
                        <Route path="/*any" view=Fallback />
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
