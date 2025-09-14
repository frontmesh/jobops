use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{Header, Footer, Fallback};
use crate::pages::{HomePage, BlogPage, PostPage, TagPage};

#[server]
pub async fn generate_random_number() -> Result<f64, ServerFnError> {
    Ok(js_sys::Math::random())
}

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

// Keep the old HelloWorld component for backward compatibility if needed
#[component]
pub fn HelloWorld() -> impl IntoView {
    provide_meta_context();

    let get_random = create_server_action::<GenerateRandomNumber>();
    let on_click = move |_| get_random.dispatch(GenerateRandomNumber {});
    
    // Create a derived signal for the formatted value
    let formatted_value = move || {
        get_random
            .value()
            .get()
            .and_then(|r| r.ok())
            .map(|n| format!("{:.6}", n))
            .unwrap_or_else(|| "Click to generate".into())
    };

    // Create a derived signal for the pending state
    let is_pending = move || get_random.pending().get();

    view! {
        <>
            <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            
            <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
                <nav class="bg-white shadow-md p-4">
                    <div class="container mx-auto">
                        <h2 class="text-xl font-semibold text-indigo-600">Leptos + Tailwind Demo</h2>
                    </div>
                </nav>
                
                <main class="container mx-auto px-4 py-8">
                    <div class="max-w-2xl mx-auto">
                        <div class="bg-white rounded-lg shadow-xl p-6 space-y-6">
                            <div class="border-b border-gray-200 pb-4">
                                <h1 class="text-3xl font-bold text-gray-900">
                                    "Random Number Generator"
                                </h1>
                                <p class="mt-2 text-gray-600">
                                    "Click the button below to generate a random number using server functions."
                                </p>
                            </div>
                            
                            <div class="flex flex-col items-center space-y-4">
                                <div class="text-center">
                                    <span class="text-2xl font-mono bg-gray-100 rounded-lg px-4 py-2">
                                        {formatted_value}
                                    </span>
                                </div>
                                
                                <button
                                    class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-6 rounded-lg
                                           transform transition duration-200 hover:scale-105 active:scale-95
                                           disabled:opacity-50 disabled:cursor-not-allowed"
                                    class:opacity-50=is_pending
                                    prop:disabled=is_pending
                                    on:click=on_click
                                >
                                    <div class="flex items-center space-x-2">
                                        <span>"Generate Number"</span>
                                        <span class="text-indigo-200">
                                            {move || if is_pending() { "..." } else { "" }}
                                        </span>
                                    </div>
                                </button>
                            </div>

                            <div class="mt-8 bg-gray-50 rounded-md p-4">
                                <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wide">
                                    "Last Generated Value"
                                </h3>
                                <div class="mt-2">
                                    <div class="text-sm text-gray-600 font-mono bg-white px-2 py-1 rounded">
                                        {formatted_value}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </main>
            </div>
        </>
    }
}
