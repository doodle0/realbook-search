use yew::prelude::*;
use crate::models::{RealBookEntry, SearchResponse};

/// Props for the ResultsList component
#[derive(Properties, PartialEq)]
pub struct ResultsListProps {
    /// The search response containing results and total count
    /// None means no search has been performed yet
    pub results: Option<SearchResponse>,

    /// Whether data is currently loading
    pub loading: bool,

    /// Index of the currently selected result (via keyboard navigation)
    /// None means no result is selected via keyboard
    pub selected_index: Option<usize>,

    /// Callback fired when a user clicks on a result
    /// Passes the clicked entry to the parent component
    pub on_entry_click: Callback<RealBookEntry>,
}

/// ResultsList component - displays search results or a placeholder message
///
/// This component demonstrates conditional rendering in Yew:
/// - If there are results, display them in a scrollable list
/// - If there are no results, display a helpful placeholder message
#[function_component(ResultsList)]
pub fn results_list(props: &ResultsListProps) -> Html {
    // Auto-scroll selected item into view when selection changes
    {
        let selected_index = props.selected_index;
        use_effect_with(selected_index, move |sel_idx| {
            if let Some(idx) = sel_idx
                && let Some(document) = web_sys::window().and_then(|w| w.document())
                && let Some(element) = document.query_selector(&format!(".result-item[data-index='{}']", idx)).ok().flatten() {
                // Use "nearest" behavior - only scrolls if element is not visible
                // This works smoothly for both up and down navigation
                let options = web_sys::ScrollIntoViewOptions::new();
                options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                let _ = element.scroll_into_view_with_scroll_into_view_options(&options);
            }
            || ()
        });
    }

    html! {
        // Pico CSS styles <article> with aria-busy showing built-in loading spinner
        <article aria-busy={props.loading.to_string()}>
            {
                if let Some(response) = &props.results {
                    // We have results - display them
                    html! {
                        <>
                            <header>
                                <h2>{ format!("Results ({})", response.total) }</h2>
                            </header>

                            <div class="results-list">
                                {
                                    // Iterate over results and create a div for each
                                    // enumerate() gives us the index along with each entry
                                    for response.results.iter().enumerate().map(|(index, entry)| {
                                        // Clone the entry so we can move it into the closure
                                        let entry_clone = entry.clone();

                                        // Check if this is the currently selected result (via keyboard)
                                        let is_selected = props.selected_index == Some(index);

                                        // Add 'selected' class if this result is highlighted
                                        let class = if is_selected {
                                            "result-item selected"
                                        } else {
                                            "result-item"
                                        };

                                        // Create a click handler for this specific result
                                        let on_click = {
                                            let callback = props.on_entry_click.clone();
                                            let entry = entry_clone.clone();
                                            // The move keyword captures entry by value
                                            Callback::from(move |_| {
                                                callback.emit(entry.clone());
                                            })
                                        };

                                        html! {
                                            <div {class} onclick={on_click} data-index={index.to_string()}>
                                                // Title in bold
                                                <div class="result-title">
                                                    { &entry.title }
                                                </div>

                                                // Volume and page info in smaller, muted text
                                                <div class="result-meta">
                                                    { format!("Vol. {} | Pages {}", entry.volume, entry.page_range()) }
                                                </div>
                                            </div>
                                        }
                                    })
                                }
                            </div>
                        </>
                    }
                } else {
                    // No results yet - show placeholder
                    html! {
                        <div class="placeholder">
                            <p>{ "Search for a song or click Random to get started" }</p>
                        </div>
                    }
                }
            }
        </article>
    }
}
