// Import modules
mod api;
mod models;
mod components;
mod utils;

// Import types we need
use models::{RealBookEntry, SearchResponse};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use gloo_events::EventListener;

// Import all our components
use components::{Header, SearchInput, ResultsList, SheetViewer};

/// Main App component
///
/// This is the root component that manages all application state using Yew's
/// hook-based state management. It orchestrates child components and handles
/// all API interactions and keyboard navigation.
///
/// State managed by this component:
/// - search_query: Current search text
/// - selected_volume: Volume filter (or None for "All")
/// - search_results: Results from the last search
/// - selected_entry: Entry selected for viewing sheet music
/// - search_loading: Whether a search API call is in progress
/// - random_loading: Whether a random entry API call is in progress
/// - error: Error message displayed in SearchInput
/// - selected_index: Index of keyboard-selected result
#[function_component(App)]
fn app() -> Html {
    // Initialize state using the use_state hook
    // use_state returns a handle that acts like both a value and a setter
    let search_query = use_state(|| String::new());
    let selected_volume = use_state(|| Option::<u32>::None);
    let search_results = use_state(|| Option::<SearchResponse>::None);
    let selected_entry = use_state(|| Option::<RealBookEntry>::None);
    let search_loading = use_state(|| false);
    let random_loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    // Track which result is currently selected via keyboard navigation
    let selected_index = use_state(|| Option::<usize>::None);

    // Callback: Handle when user types in the search box
    // This triggers live search and clears the sheet viewer
    let on_query_change = {
        let search_query = search_query.clone();
        let selected_entry = selected_entry.clone();
        Callback::from(move |new_query: String| {
            search_query.set(new_query);
            // Clear sheet viewer when typing - user must press Enter to view
            selected_entry.set(None);
        })
    };

    // Callback: Handle when user changes the volume dropdown
    let on_volume_change = {
        let selected_volume = selected_volume.clone();
        Callback::from(move |new_volume: Option<u32>| {
            selected_volume.set(new_volume);
        })
    };

    // Callback: Handle when user clicks the Random button
    let on_random = {
        let selected_entry = selected_entry.clone();
        let random_loading = random_loading.clone();
        let error = error.clone();

        Callback::from(move |_: ()| {
            let entry = selected_entry.clone();
            let loading = random_loading.clone();
            let error = error.clone();

            // Show loading spinner before clearing entry to avoid placeholder flash
            loading.set(true);
            entry.set(None);
            error.set(None);

            spawn_local(async move {
                // Ensure spinner shows for at least 300ms for better UX
                let min_duration = gloo_timers::future::TimeoutFuture::new(300);

                let result = api::get_random().await;

                // Wait for minimum duration
                min_duration.await;

                match result {
                    Ok(random_entry) => {
                        entry.set(Some(random_entry));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Callback: Handle when user clicks on a search result
    let on_entry_click = {
        let selected_entry = selected_entry.clone();
        Callback::from(move |entry: RealBookEntry| {
            selected_entry.set(Some(entry));
        })
    };

    // Callback: Handle arrow key navigation from input field
    let on_navigate = {
        let selected_index = selected_index.clone();
        let search_results = search_results.clone();
        Callback::from(move |direction: String| {
            if let Some(response) = (*search_results).as_ref() {
                let total = response.results.len();
                if total > 0 {
                    let new_index = if direction == "down" {
                        utils::next_result_index(*selected_index, total)
                    } else {
                        utils::prev_result_index(*selected_index, total)
                    };
                    selected_index.set(Some(new_index));
                }
            }
        })
    };

    // Callback: Handle Enter key from input field
    let on_enter = {
        let selected_entry = selected_entry.clone();
        let selected_index = selected_index.clone();
        let search_results = search_results.clone();
        Callback::from(move |_: ()| {
            if let Some(response) = (*search_results).as_ref() {
                if let Some(idx) = *selected_index {
                    if idx < response.results.len() {
                        selected_entry.set(Some(response.results[idx].clone()));
                    }
                }
            }
        })
    };

    // Live search: trigger search whenever query or volume changes
    {
        let search_query = search_query.clone();
        let selected_volume = selected_volume.clone();
        let search_results = search_results.clone();
        let search_loading = search_loading.clone();
        let error = error.clone();
        let selected_index = selected_index.clone();

        use_effect_with(((*search_query).clone(), *selected_volume), move |(query, volume)| {
            let query = query.clone();
            let volume = *volume;
            let results = search_results.clone();
            let loading = search_loading.clone();
            let error = error.clone();
            let selected_index = selected_index.clone();

            // Only search if query is not empty
            if !query.is_empty() {
                loading.set(true);
                error.set(None);

                spawn_local(async move {
                    match api::search(Some(query), volume, None).await {
                        Ok(response) => {
                            results.set(Some(response.clone()));
                            // Auto-highlight first result if results exist
                            if !response.results.is_empty() {
                                selected_index.set(Some(0));
                            } else {
                                selected_index.set(None);
                            }
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(e.message));
                            selected_index.set(None);
                        }
                    }
                    loading.set(false);
                });
            } else {
                // Clear results if query is empty
                results.set(None);
                selected_index.set(None);
            }

            || ()
        });
    }

    // Set up global keyboard shortcuts for when input is not focused
    // Arrow keys and Enter work both in the input field and globally
    {
        let selected_entry_clone = selected_entry.clone();
        let selected_index_clone = selected_index.clone();

        use_effect_with(
            ((*search_results).clone(), *selected_index),
            move |(results, sel_idx)| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let current_results = results.clone();
            let current_index = *sel_idx;

            let listener = EventListener::new(&document, "keydown", move |event| {
                let keyboard_event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();

                // Skip if user is typing in input/textarea
                // (these shortcuts are handled by the input's onkeydown)
                if let Some(target) = keyboard_event.target() {
                    if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                        let tag_name = element.tag_name().to_lowercase();
                        if tag_name == "input" || tag_name == "textarea" {
                            return;
                        }
                    }
                }

                // Arrow Down -> Navigate to next result
                if keyboard_event.key() == "ArrowDown" {
                    if let Some(response) = &current_results {
                        let total = response.results.len();
                        if total > 0 {
                            keyboard_event.prevent_default();
                            let next_index = utils::next_result_index(current_index, total);
                            selected_index_clone.set(Some(next_index));
                        }
                    }
                }
                // Arrow Up -> Navigate to previous result
                else if keyboard_event.key() == "ArrowUp" {
                    if let Some(response) = &current_results {
                        let total = response.results.len();
                        if total > 0 {
                            keyboard_event.prevent_default();
                            let prev_index = utils::prev_result_index(current_index, total);
                            selected_index_clone.set(Some(prev_index));
                        }
                    }
                }
                // Enter -> View the currently selected result
                else if keyboard_event.key() == "Enter" {
                    if let Some(response) = &current_results {
                        if let Some(idx) = current_index {
                            if idx < response.results.len() {
                                keyboard_event.prevent_default();
                                // Set the selected entry to view its sheet music
                                selected_entry_clone.set(Some(response.results[idx].clone()));
                            }
                        }
                    }
                }
            });

            // Return cleanup function - the listener is dropped when this runs
            // This happens when the component unmounts
            move || drop(listener)
        });
    }

    // Render the UI
    // The html! macro lets us write JSX-like syntax
    html! {
        // Pico CSS styles <main> as the main container
        <main class="container">
            // Header component (stateless, no props needed)
            <Header />

            // SearchInput component (controlled component with callbacks)
            // Search happens automatically as user types
            <SearchInput
                query={(*search_query).clone()}
                selected_volume={*selected_volume}
                random_loading={*random_loading}
                error={(*error).clone()}
                on_query_change={on_query_change}
                on_volume_change={on_volume_change}
                on_random={on_random}
                on_navigate={on_navigate}
                on_enter={on_enter}
            />

            // Content grid: results on left, viewer on right (responsive)
            <div class="content-grid">
                // ResultsList component - shows loading spinner while searching
                // selected_index tracks which result is highlighted via keyboard navigation
                <ResultsList
                    results={(*search_results).clone()}
                    loading={*search_loading}
                    selected_index={*selected_index}
                    on_entry_click={on_entry_click}
                />

                // SheetViewer component - displays selected sheet music
                <SheetViewer
                    entry={(*selected_entry).clone()}
                    loading={*random_loading}
                />
            </div>
        </main>
    }
}

/// Entry point of the application
///
/// This function is called when the WASM module loads.
/// It creates a Yew renderer for the App component and mounts it to the <body>.
fn main() {
    yew::Renderer::<App>::new().render();
}
