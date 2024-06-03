use crate::components::App;
use console_error_panic_hook::set_once;
use crossword::Crossword;
use leptos::{mount_to_body, view};

mod article;
#[allow(non_snake_case)]
mod components;
mod crossword;

fn main() {
    set_once();
    mount_to_body(move || view! { <App/> });
}
