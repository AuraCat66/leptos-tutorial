use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    let html = "<p>This html will be injected.</p>";

    view! {
        <div inner_html=html />
        <button
            on:click=move |_| {
                *set_count.write() += 10;
            }
            style="position: absolute"
            style:left=move || format!("{}px", count.get() + 175)
            style:max-width="400px"
            style=("--columns", move || count.get().to_string())
        >
            "Click to move"
        </button>
        <progress max="50" value=double_count />
        <p>"Double Count: " {double_count}</p>
    }
}
