use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement};

/// Props for the SearchInput component
///
/// In Yew, Props are how we pass data from parent to child components.
/// The #[derive(Properties, PartialEq)] attributes are required for Yew props.
///
/// PartialEq is used by Yew to determine if the component needs to re-render
/// when props change (if props are equal, no re-render is needed).
#[derive(Properties, PartialEq)]
pub struct SearchInputProps {
    /// Current search query text
    pub query: String,

    /// Currently selected volume (None means "All Volumes")
    pub selected_volume: Option<u32>,

    /// Whether the Random button is loading
    pub random_loading: bool,

    /// Error message to display (None means no error)
    pub error: Option<String>,

    /// Callback fired when the query input changes
    /// Takes the new query string as a parameter
    pub on_query_change: Callback<String>,

    /// Callback fired when the volume select changes
    /// Takes the new volume (or None for "All Volumes") as a parameter
    pub on_volume_change: Callback<Option<u32>>,

    /// Callback fired when the Random button is clicked
    pub on_random: Callback<()>,

    /// Callback for arrow key navigation (up/down)
    pub on_navigate: Callback<String>,

    /// Callback for Enter key (to view selected result)
    pub on_enter: Callback<()>,
}

/// SearchInput component - handles search query, volume filter, and action buttons
///
/// This is a "controlled component" - it doesn't manage its own state.
/// Instead, it receives its current values via props and notifies the parent
/// of changes via callbacks. This is the React pattern of "lifting state up".
#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProps) -> Html {
    // Create a ref to the input element so we can focus it
    let input_ref = use_node_ref();

    // Auto-focus the input when the component mounts
    {
        let input_ref = input_ref.clone();
        use_effect_with((), move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
            || ()
        });
    }

    // Create event handlers that convert DOM events to our callback types

    // Handle text input changes
    // This now triggers live search automatically
    let on_input = {
        let callback = props.on_query_change.clone();
        Callback::from(move |e: InputEvent| {
            // Get the input element from the event
            let input: HtmlInputElement = e.target_unchecked_into();
            // Extract the value and pass it to the parent callback
            callback.emit(input.value());
        })
    };

    // Handle keyboard events in the input field
    // Arrow keys navigate results, Enter views selected result
    let on_keydown = {
        let navigate = props.on_navigate.clone();
        let enter = props.on_enter.clone();
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            match key.as_str() {
                "ArrowUp" => {
                    e.prevent_default(); // Prevent cursor from moving in input
                    navigate.emit("up".to_string());
                }
                "ArrowDown" => {
                    e.prevent_default(); // Prevent cursor from moving in input
                    navigate.emit("down".to_string());
                }
                "Enter" => {
                    e.prevent_default(); // Prevent form submission
                    enter.emit(());
                }
                _ => {}
            }
        })
    };

    // Handle volume select changes
    let on_change = {
        let callback = props.on_volume_change.clone();
        Callback::from(move |e: Event| {
            // Get the select element from the event
            let select: HtmlSelectElement = e.target_unchecked_into();
            let value = select.value();

            // Convert the string value to Option<u32>
            // Empty string means "All Volumes" (None)
            let volume = if value.is_empty() {
                None
            } else {
                value.parse().ok()
            };

            callback.emit(volume);
        })
    };

    // Handle Random button click
    let on_random_click = {
        let callback = props.on_random.clone();
        Callback::from(move |_| {
            callback.emit(());
        })
    };

    html! {
        // Pico CSS automatically styles <section> elements nicely with padding/margins
        <section>
            <div class="search-controls">
                // Text input for search query
                // The "value" prop makes this a controlled input
                // Search happens automatically as you type
                // Arrow keys and Enter work even when focused in this input
                <input
                    ref={input_ref}
                    type="text"
                    placeholder="Type to search... (↑↓ navigate, Enter to view)"
                    value={props.query.clone()}
                    oninput={on_input}
                    onkeydown={on_keydown}
                />

                // Volume filter dropdown
                <select onchange={on_change}>
                    <option value="" selected={props.selected_volume.is_none()}>
                        { "All Volumes" }
                    </option>
                    <option value="1" selected={props.selected_volume == Some(1)}>
                        { "Volume 1" }
                    </option>
                    <option value="2" selected={props.selected_volume == Some(2)}>
                        { "Volume 2" }
                    </option>
                    <option value="3" selected={props.selected_volume == Some(3)}>
                        { "Volume 3" }
                    </option>
                </select>

                // Random button - disabled during loading
                <button
                    onclick={on_random_click}
                    disabled={props.random_loading}
                    aria-busy={props.random_loading.to_string()}
                >
                    { "🎲 Random" }
                </button>
            </div>

            // Display error message if present
            // Pico CSS styles <mark> elements for emphasis/alerts
            {if let Some(error_msg) = &props.error {
                html! {
                    <mark style="background-color: var(--pico-del-color); padding: var(--pico-spacing);">
                        { error_msg }
                    </mark>
                }
            } else {
                html! {}
            }}
        </section>
    }
}
