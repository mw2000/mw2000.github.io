#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
use markdown::*; 
use manganis::*;
use std::fs;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}


#[component]
fn Blog(id: i32) -> Element {
    // Read the Markdown file
    let markdown_input = include_str!("../posts/1.md");
    let markdown_options = Options {
        parse: ParseOptions {
            constructs: Constructs { 
                frontmatter: true, 
                ..Constructs::gfm()
            },
            ..ParseOptions::gfm()
        },
        compile: CompileOptions::gfm()
    };

    // Write to a new String buffer.
    let post = markdown::to_html_with_options(markdown_input, &markdown_options).expect("Failed to parse markdown");
    let gfm = Constructs::gfm();

    // Parse the Markdown into HTML
    // Render the HTML content=
    // Rendering?
    rsx! {
        div { dangerous_inner_html: "{post}" }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            h1 { "Mihir Wadekar's website" }
            div {
                ul {
                    li {
                        Link {
                            to: Route::Blog {
                                id: 1
                            },
                            "Binius: highly efficient proofs over binary fields"
                        }
                    }
                    li {
                        Link {
                            to: Route::Blog {
                                id: 2
                            },
                            "Ask security questions"
                        }
                    }
                    li {
                        Link {
                            to: Route::Blog {
                                id: 3
                            },
                            "Some ways to use ZK-SNARKs for privacy"
                        }
                    }
                }
            }
        }
    }
}
