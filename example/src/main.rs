use leptos::*;
use leptos_abort::create_abort;
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;

fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (query, set_query) = create_signal(cx, "".to_string());

    let abort = create_abort(cx);

    let result = create_resource(
        cx,
        move || query(),
        move |query| async move {
            // Creates a new AbortSignal that can be used for use in RequestInit
            // This signal will automatically be aborted when a new run of the fetcher calls the abort for a new signal
            // or when the context gets disposed
            let signal = abort.signal();

            // Hit dummy API, give the signal via the RequestInit parameter
            let _ = JsFuture::from(window().fetch_with_str_and_init(
                "https://hub.dummyapis.com/delay?seconds=1",
                &RequestInit::new().signal(Some(&signal)),
            ))
            .await;

            // Give back the original parameter as this is just an example
            query
        },
    );

    view! { cx,
        <input
            on:input=move |e| {
            set_query(event_target_value(&e))
            }
        />
            {move || result.read(cx)}
    }
}
