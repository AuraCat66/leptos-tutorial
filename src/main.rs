use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters.map(
        |count| {
            view! {
                <li>
                    <button on:click=move |_| count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! { <ul>{counter_buttons}</ul> }
}
