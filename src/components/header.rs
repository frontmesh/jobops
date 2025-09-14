use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = create_signal(false);
    
    let toggle_mobile_menu = move |_| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };

    view! {
        <header class="relative bg-gray-100 shadow">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center py-6 md:justify-start md:space-x-10">
                    // Logo
                    <div class="flex justify-start lg:w-0 lg:flex-1">
                        <A href="/" class="inline-flex items-center">
                            <span class="text-2xl md:text-3xl tracking-tight leading-10 font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-indigo-600">
                                JobOps
                            </span>
                        </A>
                    </div>
                    
                    // Mobile menu button
                    <div class="-mr-2 -my-2 md:hidden">
                        <button
                            type="button"
                            class="bg-gray-100 rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
                            on:click=toggle_mobile_menu
                        >
                            <span class="sr-only">"Open menu"</span>
                            // Hamburger icon
                            <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>
                    </div>
                    
                    // Desktop navigation
                    <nav class="hidden md:flex space-x-10">
                        <A href="/" class="text-base font-medium text-gray-500 hover:text-gray-900">
                            "Home"
                        </A>
                        <A href="/blog" class="text-base font-medium text-gray-500 hover:text-gray-900">
                            "Blog"
                        </A>
                    </nav>
                    
                    // Desktop action buttons
                    <div class="hidden md:flex items-center justify-end md:flex-1 lg:w-0">
                        <A href="/log-in" class="whitespace-nowrap text-base font-medium text-gray-500 hover:text-gray-900">
                            "Log In"
                        </A>
                        <A href="/post-job" class="ml-8 whitespace-nowrap inline-flex items-center justify-center px-4 py-2 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700">
                            "Post Job"
                        </A>
                    </div>
                </div>
                
                // Mobile menu (visible when mobile_menu_open is true)
                <div class={move || if mobile_menu_open.get() { "md:hidden" } else { "hidden" }}>
                    <div class="px-2 pt-2 pb-3 space-y-1 sm:px-3 border-t border-gray-200">
                        <A href="/" class="block px-3 py-2 text-base font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50 rounded-md">
                            "Home"
                        </A>
                        <A href="/blog" class="block px-3 py-2 text-base font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50 rounded-md">
                            "Blog"
                        </A>
                        <A href="/log-in" class="block px-3 py-2 text-base font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50 rounded-md">
                            "Log In"
                        </A>
                        <A href="/post-job" class="block w-full px-3 py-2 text-center text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700 rounded-md">
                            "Post Job"
                        </A>
                    </div>
                </div>
            </div>
        </header>
    }
}
