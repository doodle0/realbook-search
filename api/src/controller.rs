use std::path::Path;
use std::sync::Arc;
use rocket::{fs::NamedFile, State, serde::json::Json};
use crate::models::{RealBookEntry, SearchResponse, VolumeInfo};
use std::collections::HashMap;

#[get("/")]
pub fn index() -> &'static str {
    "This is the API root address."
}

#[get("/rickroll")]
pub async fn rickroll() -> Option<NamedFile> {
    println!("{:?}", Path::new("resources/rickroll.gif").canonicalize());
    NamedFile::open(Path::new("api/resources/rickroll.gif")).await.ok()
}

/// Search endpoint with optional filters
/// Query parameters:
/// - query: text search in title (case-insensitive, partial match)
/// - volume: filter by volume number (1, 2, or 3)
/// - page: filter by page number (returns entries containing this page)
#[get("/search?<query>&<volume>&<page>")]
pub fn search(
    data: &State<Arc<Vec<RealBookEntry>>>,
    query: Option<String>,
    volume: Option<u32>,
    page: Option<u32>,
) -> Json<SearchResponse> {
    let mut results: Vec<RealBookEntry> = data.iter().cloned().collect();

    // Filter by text query
    if let Some(q) = query {
        if !q.is_empty() {
            results.retain(|entry| entry.matches(&q));
        }
    }

    // Filter by volume
    if let Some(vol) = volume {
        results.retain(|entry| entry.volume == vol);
    }

    // Filter by page (entry must contain this page)
    if let Some(p) = page {
        results.retain(|entry| entry.page_s <= p && p <= entry.page_e);
    }

    let total = results.len();

    Json(SearchResponse { results, total })
}

/// List all volumes with entry counts
#[get("/volumes")]
pub fn volumes(data: &State<Arc<Vec<RealBookEntry>>>) -> Json<Vec<VolumeInfo>> {
    let mut volume_counts: HashMap<u32, usize> = HashMap::new();

    for entry in data.iter() {
        *volume_counts.entry(entry.volume).or_insert(0) += 1;
    }

    let mut volumes: Vec<VolumeInfo> = volume_counts
        .into_iter()
        .map(|(volume, count)| VolumeInfo { volume, count })
        .collect();

    volumes.sort_by_key(|v| v.volume);

    Json(volumes)
}

/// Get a random Real Book entry
#[get("/random")]
pub fn random(data: &State<Arc<Vec<RealBookEntry>>>) -> Json<RealBookEntry> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let entry = data.choose(&mut rng).unwrap().clone();
    Json(entry)
}
