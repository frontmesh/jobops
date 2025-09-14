use leptos::*;
use leptos_router::*;
use crate::components::MetaTags;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <MetaTags 
            title="JobOps - Find Your Next Career Opportunity".to_string()
            description="Discover amazing job opportunities and connect with top employers. Start your career journey today.".to_string()
            url="/".to_string()
            image="https://example.com/jobops-home.jpg".to_string()
        />
        
        // Hero Section
        <section class="bg-gradient-to-br from-blue-50 to-indigo-100 py-20">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h1 class="text-4xl md:text-6xl font-bold text-gray-900 mb-6">
                        "Find Your Dream "
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-indigo-600">
                            "Job Today"
                        </span>
                    </h1>
                    <p class="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
                        "Connect with top employers, explore exciting opportunities, and take the next step in your career journey."
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <A 
                            href="/jobs" 
                            class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                        >
                            "Browse Jobs"
                        </A>
                        <A 
                            href="/post-job" 
                            class="bg-white hover:bg-gray-50 text-indigo-600 border-2 border-indigo-600 font-semibold py-3 px-8 rounded-lg transition-colors"
                        >
                            "Post a Job"
                        </A>
                    </div>
                </div>
            </div>
        </section>
        
        // Features Section
        <section class="py-20 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-16">
                    <h2 class="text-3xl font-bold text-gray-900 mb-4">
                        "Why Choose JobOps?"
                    </h2>
                    <p class="text-lg text-gray-600">
                        "We make job searching and hiring simple, efficient, and effective."
                    </p>
                </div>
                
                <div class="grid md:grid-cols-3 gap-8">
                    <div class="text-center p-6">
                        <div class="w-16 h-16 bg-indigo-100 rounded-full flex items-center justify-center mx-auto mb-4">
                            <span class="text-2xl">"🎯"</span>
                        </div>
                        <h3 class="text-xl font-semibold mb-2">"Targeted Matching"</h3>
                        <p class="text-gray-600">
                            "Our advanced algorithm matches you with jobs that fit your skills and preferences."
                        </p>
                    </div>
                    
                    <div class="text-center p-6">
                        <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                            <span class="text-2xl">"⚡"</span>
                        </div>
                        <h3 class="text-xl font-semibold mb-2">"Fast Applications"</h3>
                        <p class="text-gray-600">
                            "Apply to multiple jobs with one click. No more repetitive form filling."
                        </p>
                    </div>
                    
                    <div class="text-center p-6">
                        <div class="w-16 h-16 bg-purple-100 rounded-full flex items-center justify-center mx-auto mb-4">
                            <span class="text-2xl">"🤝"</span>
                        </div>
                        <h3 class="text-xl font-semibold mb-2">"Direct Connection"</h3>
                        <p class="text-gray-600">
                            "Connect directly with hiring managers and get faster responses."
                        </p>
                    </div>
                </div>
            </div>
        </section>
        
        // CTA Section
        <section class="py-20 bg-gray-50">
            <div class="max-w-4xl mx-auto text-center px-4 sm:px-6 lg:px-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-4">
                    "Ready to Get Started?"
                </h2>
                <p class="text-lg text-gray-600 mb-8">
                    "Join thousands of professionals who have found their dream jobs through JobOps."
                </p>
                <A 
                    href="/signup" 
                    class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-3 px-8 rounded-lg transition-colors inline-block"
                >
                    "Sign Up Now"
                </A>
            </div>
        </section>
    }
}
