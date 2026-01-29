use yew::prelude::*;

const API_BASE_URL: &str = "http://localhost:8080/api";

#[component]
fn Counter() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

#[component]
fn ApiTest() -> Html {
    html! {
        <div>
            // <button {onclick}>{ "API" }</button>
            <img src={ format!("{}/rickroll", API_BASE_URL) }/>
        </div>
    }
}

#[component]
fn App() -> Html {
    html! {
        <>
            <Counter/>
            <ApiTest/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
