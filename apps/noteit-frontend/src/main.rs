use dioxus::prelude::*;

// https://tailwindcss.com/ 
fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    h1 { "lol" }
  }
}
