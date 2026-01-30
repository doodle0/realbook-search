use yew::prelude::*;
use crate::models::RealBookEntry;
use crate::components::SheetImage;

/// Props for the SheetViewer component
#[derive(Properties, PartialEq)]
pub struct SheetViewerProps {
    /// The currently selected entry to display sheet music for
    pub entry: Option<RealBookEntry>,

    /// Whether data is currently loading (shows Pico CSS spinner via aria-busy)
    pub loading: bool,
}

/// SheetViewer component - displays sheet music images for the selected song
#[function_component(SheetViewer)]
pub fn sheet_viewer(props: &SheetViewerProps) -> Html {
    html! {
        // aria-busy shows Pico CSS's built-in loading spinner
        <article aria-busy={props.loading.to_string()}>
            {
                if let Some(entry) = &props.entry {
                    html! {
                        <>
                            <header>
                                <h2>{ &entry.title }</h2>
                                <p>
                                    { format!("Volume {} | Pages {}", entry.volume, entry.page_range()) }
                                </p>
                            </header>

                            <div class="sheet-images">
                                {
                                    for entry.all_image_urls().iter().map(|url| {
                                        html! {
                                            <SheetImage
                                                url={url.clone()}
                                                alt={format!("Sheet music for {}", entry.title)}
                                            />
                                        }
                                    })
                                }
                            </div>
                        </>
                    }
                } else {
                    html! { <></> }
                }
            }
        </article>
    }
}
