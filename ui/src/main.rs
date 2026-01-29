mod api;
mod models;

use models::{RealBookEntry, SearchResponse};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
fn App() -> Html {
    let search_query = use_state(|| String::new());
    let selected_volume = use_state(|| Option::<u32>::None);
    let search_results = use_state(|| Option::<SearchResponse>::None);
    let selected_entry = use_state(|| Option::<RealBookEntry>::None);
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    // Handle search
    let on_search = {
        let search_query = search_query.clone();
        let selected_volume = selected_volume.clone();
        let search_results = search_results.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let query = (*search_query).clone();
            let volume = *selected_volume;
            let results = search_results.clone();
            let loading = loading.clone();
            let error = error.clone();

            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match api::search(Some(query), volume, None).await {
                    Ok(response) => {
                        results.set(Some(response));
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

    // Handle random
    let on_random = {
        let selected_entry = selected_entry.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let entry = selected_entry.clone();
            let loading = loading.clone();
            let error = error.clone();

            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match api::get_random().await {
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

    // Input handlers
    let on_query_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let on_volume_change = {
        let selected_volume = selected_volume.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = select.value();
            if value.is_empty() {
                selected_volume.set(None);
            } else {
                selected_volume.set(value.parse().ok());
            }
        })
    };

    let on_entry_click = {
        let selected_entry = selected_entry.clone();
        Callback::from(move |entry: RealBookEntry| {
            selected_entry.set(Some(entry));
        })
    };

    html! {
        <div class="container">
            <header>
                <h1>{ "Real Book Search" }</h1>
            </header>

            <div class="search-section">
                <div class="search-controls">
                    <input
                        type="text"
                        placeholder="Search by title..."
                        value={(*search_query).clone()}
                        oninput={on_query_input}
                    />
                    <select onchange={on_volume_change}>
                        <option value="">{ "All Volumes" }</option>
                        <option value="1">{ "Volume 1" }</option>
                        <option value="2">{ "Volume 2" }</option>
                        <option value="3">{ "Volume 3" }</option>
                    </select>
                    <button onclick={on_search} disabled={*loading}>
                        { if *loading { "Searching..." } else { "Search" } }
                    </button>
                    <button onclick={on_random} disabled={*loading}>
                        { "🎲 Random" }
                    </button>
                </div>

                {if let Some(err) = (*error).as_ref() {
                    html! { <div class="error">{ err }</div> }
                } else {
                    html! {}
                }}
            </div>

            <div class="content">
                <div class="results-section">
                    {if let Some(response) = (*search_results).as_ref() {
                        html! {
                            <>
                                <h2>{ format!("Results ({})", response.total) }</h2>
                                <div class="results-list">
                                    {for response.results.iter().map(|entry| {
                                        let entry_clone = entry.clone();
                                        let onclick = {
                                            let callback = on_entry_click.clone();
                                            move |_| callback.emit(entry_clone.clone())
                                        };
                                        html! {
                                            <div class="result-item" {onclick}>
                                                <div class="result-title">{ &entry.title }</div>
                                                <div class="result-meta">
                                                    { format!("Vol. {} | Pages {}", entry.volume, entry.page_range()) }
                                                </div>
                                            </div>
                                        }
                                    })}
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <div class="placeholder">
                                { "Search for a song or click Random to get started" }
                            </div>
                        }
                    }}
                </div>

                <div class="viewer-section">
                    {if let Some(entry) = (*selected_entry).as_ref() {
                        html! {
                            <>
                                <h2>{ &entry.title }</h2>
                                <div class="sheet-images">
                                    {for entry.all_image_urls().iter().map(|url| {
                                        html! {
                                            <img src={url.clone()} alt={format!("Sheet for {}", entry.title)} />
                                        }
                                    })}
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <div class="placeholder">
                                { "Select a song to view the sheet music" }
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
