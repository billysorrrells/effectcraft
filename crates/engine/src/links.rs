//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "Billo Motion";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://billysorrells.com/billotech/free/";
pub const WEBSITE: &str = "https://billysorrells.com/billotech/";
pub const APP_PAGE: &str = "https://billysorrells.com/billotech/#toolkit";
pub const GITHUB: &str = "https://billysorrells.com/billotech/free/";
pub const ISSUES: &str = "mailto:bookbillysorrells@gmail.com?subject=Billo%20Motion%20feedback";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("Billo Photo", "photocraft"),
    ("Billo Vector", "vectorcraft"),
    ("Billo Cut", "filmcraft"),
    ("Billo Light", "lightcraft"),
    ("Billo PDF", "pdfcraft"),
    ("Billo Layout", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://billysorrells.com/billotech/")
}
