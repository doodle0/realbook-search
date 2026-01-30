use yew::prelude::*;

/// Props for the SheetImage component
#[derive(Properties, PartialEq)]
pub struct SheetImageProps {
    /// URL of the sheet music image
    pub url: String,

    /// Alt text for the image
    pub alt: String,
}

/// SheetImage component - displays a single sheet music image with its own loading state
///
/// Each image manages its own loading state independently, showing a spinner
/// until the image's onload event fires.
#[function_component(SheetImage)]
pub fn sheet_image(props: &SheetImageProps) -> Html {
    // Track loading state for this specific image
    let loading = use_state(|| true);

    // Reset loading state when URL changes (e.g., when switching between songs)
    {
        let loading = loading.clone();
        let url = props.url.clone();
        use_effect_with(url, move |_| {
            loading.set(true);
            || ()
        });
    }

    // Callback fired when image finishes loading
    let on_load = {
        let loading = loading.clone();
        Callback::from(move |_: Event| {
            loading.set(false);
        })
    };

    html! {
        <article class="sheet-image-container" aria-busy={loading.to_string()}>
            <img
                src={props.url.clone()}
                alt={props.alt.clone()}
                onload={on_load}
                style={if *loading { "display: none;" } else { "" }}
            />
        </article>
    }
}
