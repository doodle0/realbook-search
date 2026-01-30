use yew::prelude::*;

/// Header component - displays the application title
///
/// This is a simple "presentational" component with no props or state.
/// In Yew, we use the #[function_component] macro to define components as functions.
///
/// The function returns Html, which is Yew's virtual DOM representation.
/// We use the html! macro to write JSX-like syntax that compiles to Html.
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        // Pico CSS automatically styles <header> elements nicely
        <header>
            <h1>{ "Real Book Search" }</h1>
            <p>{ "Find jazz standards by title, volume, or page number" }</p>
            <p>
                <small>
                    { "Keyboard shortcuts: " }
                    <kbd>{ "↑↓" }</kbd>{ " navigate, " }
                    <kbd>{ "Enter" }</kbd>{ " view selected" }
                </small>
            </p>
        </header>
    }
}
