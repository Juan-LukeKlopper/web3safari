#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod components;

fn main() {
        // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let mut logged_in = true;
    
    if logged_in { cx.render(rsx! { FullPage {} }) }
        else { cx.render(rsx! {Tester {} })} }
// create a component that renders a div with the text "Hello, world!"
#[component]
pub fn FullPage(cx: Scope) -> Element {
    cx.render(rsx! {
        body {
            class: "bg-gray-100 mx-auto max-h-screen max-w-screen",
            header {
                class: "bg-gradient-to-b from-archway to-indigo-700 text-cosmiclatte text-center py-4",
                h1 {
                    class: "text-3xl font-semibold text-cosmiclatte",
                    "Web3 Safari"
                }
                p {
                    class: "text-sm text-cosmiclatte",
                    "Bringing web3 to Africa thanks to the Archway foundation."
                }
            }
            nav {
                class: "bg-indigo-800 text-cosmiclatte py-2 flex justify-evenly",
                a {
                    href: "#home",
                    class: "hover:text-archway",
                    "Home"
                }
                a {
                    href: "#about",
                    class: "hover:text-archway",
                    "About"
                }
                a {
                    href: "#services",
                    class: "hover:text-archway",
                    "Services"
                }
                a {
                    href: "#contact",
                    class: "hover:text-archway",
                    "Contact"
                }
            }
            section {
                id: "home",
                class: "p-2 bg-gradient-to-r from-archway to-indigo-700 text-cosmiclatte",
                div {
                    class: " h-full w-full rounded-md bg-gray-800 ",
                    h2 {
                        class: "text-2xl font-semibold",
                        "Welcome to Our Company"
                    }
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vel felis eu nulla ultricies consectetur. Fusce interdum semper dolor, eget tempus dui efficitur vel."
                    }
                }
            }
            section {
                id: "about",
                class: "p-4 bg-gray-300",
                h2 {
                    class: "text-2xl font-semibold",
                    "About us"
                }
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vel felis eu nulla ultricies consectetur. Fusce interdum semper dolor, eget tempus dui efficitur vel."
                }
            }
            div {
                class: "w-100 bg-cover bg-wave"
            }
            section {
                id: "services",
                class: "p-4 bg-gray-400",
                h2 {
                    class: "text-2xl font-semibold",
                    "Our Services:"
                }
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vel felis eu nulla ultricies consectetur. Fusce interdum semper dolor, eget tempus dui efficitur vel."
                }
            }
            div {
                class: "w-100 bg-cover bg-wave"
            }
            section {
                id: "contact",
                class: "p-4 bg-gray-500",
                h2 {
                    class: "text-2xl font-semibold",
                    "Contact us"
                }
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vel felis eu nulla ultricies consectetur. Fusce interdum semper dolor, eget tempus dui efficitur vel."
                }
            }
            div {
                class: "w-100 bg-cover bg-wave"
            }
            footer {
                class: "bg-indigo-700 text-cosmiclatte text-center py-4",
                " ©2024 Web3 Safari. All right reserved"
            }
            crate::components::sec::Sec {
                sec_id: "Test",
                header_text: "Hello",
                p_text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vel felis eu nulla ultricies consectetur. Fusce interdum semper dolor, eget tempus dui efficitur vel.",
                outer_style: "p-2 bg-gradient-to-r from-archway to-indigo-700 text-cosmiclatte",
                inner_style: " h-full w-full rounded-md bg-gray-800 ", 
            }
        }
    })
}

#[component]
pub fn Tester(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to the dioxus blog!" }
    }
}

