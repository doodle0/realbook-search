use serde::{Deserialize, Serialize};

/// Represents a single entry in the Real Book
/// Must match backend model exactly for deserialization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RealBookEntry {
    pub title: String,
    pub volume: u32,
    pub page_s: u32,
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

    /// Get page range as a display string
    pub fn page_range(&self) -> String {
        if self.page_s == self.page_e {
            format!("{}", self.page_s)
        } else {
            format!("{}-{}", self.page_s, self.page_e)
        }
    }
}

/// Search results response from API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    pub results: Vec<RealBookEntry>,
    pub total: usize,
}

/// Volume information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolumeInfo {
    pub volume: u32,
    pub count: usize,
}
