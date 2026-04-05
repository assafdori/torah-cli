pub mod bolls;
pub mod types;

use crate::data::kjv;
use types::{Chapter, SearchResult, Verse};

pub struct Resolver;

impl Resolver {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_verse(
        &self,
        book: &str,
        chapter: u32,
        verse: u32,
        translation: &str,
    ) -> Result<Verse, String> {
        if !translation.eq_ignore_ascii_case("TORAH") {
            return Err(format!(
                "Unsupported translation '{}'. This build supports TORAH only.",
                translation
            ));
        }

        kjv::get_verse(book, chapter, verse).ok_or_else(|| "Verse not found".to_string())
    }

    pub async fn get_chapter(
        &self,
        book: &str,
        chapter: u32,
        translation: &str,
    ) -> Result<Chapter, String> {
        if !translation.eq_ignore_ascii_case("TORAH") {
            return Err(format!(
                "Unsupported translation '{}'. This build supports TORAH only.",
                translation
            ));
        }

        kjv::get_chapter(book, chapter).ok_or_else(|| "Chapter not found".to_string())
    }

    pub async fn get_verse_range(
        &self,
        book: &str,
        chapter: u32,
        verse_start: u32,
        verse_end: u32,
        translation: &str,
    ) -> Result<Vec<Verse>, String> {
        if !translation.eq_ignore_ascii_case("TORAH") {
            return Err(format!(
                "Unsupported translation '{}'. This build supports TORAH only.",
                translation
            ));
        }

        let verses = kjv::get_verse_range(book, chapter, verse_start, verse_end);
        if verses.is_empty() {
            return Err("Verse range not found".to_string());
        }
        Ok(verses)
    }

    pub async fn search(
        &self,
        query: &str,
        translation: &str,
    ) -> Result<Vec<SearchResult>, String> {
        if !translation.eq_ignore_ascii_case("TORAH") {
            return Err(format!(
                "Unsupported translation '{}'. This build supports TORAH only.",
                translation
            ));
        }

        Ok(kjv::search(query))
    }

    pub async fn get_book_names(&self, _translation: &str) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    pub async fn get_random_verse(&self, translation: &str) -> Result<Verse, String> {
        if !translation.eq_ignore_ascii_case("TORAH") {
            return Err(format!(
                "Unsupported translation '{}'. This build supports TORAH only.",
                translation
            ));
        }

        Ok(kjv::random_verse())
    }
}
