use serde::{Deserialize, Deserializer, Serialize};

/// Custom deserializer for title field that accepts both strings and numbers
fn deserialize_title<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct TitleVisitor;

    impl<'de> Visitor<'de> for TitleVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or number")
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(TitleVisitor)
}

/// Represents a single entry in the Real Book
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RealBookEntry {
    /// Song title
    #[serde(deserialize_with = "deserialize_title")]
    pub title: String,
    /// Volume number (1, 2, or 3)
    pub volume: u32,
    /// Starting page number
    pub page_s: u32,
    /// Ending page number
    pub page_e: u32,
}

impl RealBookEntry {
    /// Generate the image URL for a specific page in this entry
    pub fn image_url(&self, page: u32) -> String {
        format!(
            "https://wypn9z41ir5bzmgjjalyna.on.drv.tw/realbook/rendered/{}.jpeg",
            self.volume * 1000 + page
        )
    }

    /// Get all image URLs for this entry (from page_s to page_e)
    pub fn all_image_urls(&self) -> Vec<String> {
        (self.page_s..=self.page_e)
            .map(|page| self.image_url(page))
            .collect()
    }

    /// Check if this entry matches a search query (case-insensitive)
    pub fn matches(&self, query: &str) -> bool {
        self.title.to_lowercase().contains(&query.to_lowercase())
    }
}

/// Search query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search text (searches in title)
    pub query: Option<String>,
    /// Filter by volume
    pub volume: Option<u32>,
    /// Filter by page number (checks if page is within page_s..=page_e)
    pub page: Option<u32>,
}

/// Search results response
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    /// Matching entries
    pub results: Vec<RealBookEntry>,
    /// Total number of results
    pub total: usize,
}

/// Volume information
#[derive(Debug, Serialize)]
pub struct VolumeInfo {
    pub volume: u32,
    pub count: usize,
}