#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

// App utama
#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            // Header
            <header class="bg-gradient-to-r from-blue-600 to-teal-400 text-white p-4 shadow-lg">
                <h1 class="text-4xl font-extrabold">{"YewChat 💬"}</h1>
            </header>
            // Main chat area
            <main class="flex-1 p-4 bg-white shadow-inner">
                <div id="chat-window" class="space-y-3">
                    // di sini nanti loop pesan chat
                </div>
            </main>
            // Footer dengan input & tombol
            <footer class="bg-white p-4 shadow">
                <div class="flex max-w-2xl mx-auto">
                    <input
                        type="text"
                        placeholder="Type your message..."
                        class="flex-1 border border-gray-300 rounded-l-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-teal-400"
                    />
                    <button class="bg-teal-500 text-white px-6 py-2 rounded-r-lg hover:bg-teal-600 transition">
                        {"Send"}
                    </button>
                </div>
            </footer>
        </div>
    }
}

// entry‐point wasm
#[wasm_bindgen(start)]
pub fn run_app() {
    // optional: init logger
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}