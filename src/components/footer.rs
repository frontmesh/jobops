use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-100">
            <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div class="text-center text-gray-400">
                    <p>{"© 2025 "}<span class="tracking-tight leading-10 font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-indigo-600">JobOps </span>{". All rights reserved."}</p>
                </div>
            </div>
        </footer>
    }
}
