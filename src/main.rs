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
            class="button rounded"
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <ProgressBar progress=count />
        <ProgressBar progress=Signal::derive(double_count) />
    }
}

#[component]
fn ProgressBar(
    /// The maximum value of the progress bar
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
        <br />
    }
}
