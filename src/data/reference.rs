use crate::data::books::{self, BookInfo};

/// A parsed Torah reference.
#[derive(Debug, Clone)]
pub struct TorahReference {
    pub book: &'static BookInfo,
    pub chapter: u32,
    pub verse_start: Option<u32>,
    pub verse_end: Option<u32>,
}

impl TorahReference {
    pub fn display(&self) -> String {
        match (self.verse_start, self.verse_end) {
            (Some(start), Some(end)) if start != end => {
                format!("{} {}:{}-{}", self.book.name, self.chapter, start, end)
            }
            (Some(start), _) => {
                format!("{} {}:{}", self.book.name, self.chapter, start)
            }
            _ => {
                format!("{} {}", self.book.name, self.chapter)
            }
        }
    }
}

/// Parse a Torah reference string into a structured reference.
///
/// Supports formats:
/// - "Genesis 1:1"      -> book=Genesis, chapter=1, verse=1
/// - "Exodus 3:1-5"     -> book=Exodus, chapter=3, verses=1-5
/// - "Genesis 1"        -> book=Genesis, chapter=1 (whole chapter)
/// - "Deut 6:4-9"       -> book=Deuteronomy, chapter=6, verses=4-9
/// - "gen1:1"           -> book=Genesis, chapter=1, verse=1 (no space)
pub fn parse(input: &str) -> Result<TorahReference, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("Empty reference".to_string());
    }

    let (book_str, rest) = split_book_and_location(input)?;

    let book =
        books::normalize_book(&book_str).ok_or_else(|| format!("Unknown book: '{}'", book_str))?;

    if rest.is_empty() {
        return Ok(TorahReference {
            book,
            chapter: 1,
            verse_start: None,
            verse_end: None,
        });
    }

    let (chapter, verse_start, verse_end) = parse_chapter_verse(&rest)?;

    if chapter == 0 || chapter > book.chapters {
        return Err(format!(
            "{} has {} chapters, but chapter {} was requested",
            book.name, book.chapters, chapter
        ));
    }

    Ok(TorahReference {
        book,
        chapter,
        verse_start,
        verse_end,
    })
}

/// Split input into (book_name, chapter_verse_rest).
fn split_book_and_location(input: &str) -> Result<(String, String), String> {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    let mut i = 0;

    // Keep this defensive parsing logic even though Torah books are not numbered.
    if i < len && chars[i].is_ascii_digit() && chars[i] != '0' {
        i += 1;
        if i < len && (chars[i].is_alphabetic() || chars[i] == ' ') {
            if chars[i] == ' ' {
                i += 1;
            }
        } else {
            return Ok(("".to_string(), input.to_string()));
        }
    }

    let book_start = 0;
    while i < len && (chars[i].is_alphabetic() || chars[i] == ' ' || chars[i] == '.') {
        i += 1;
        if i < len && chars[i - 1] == ' ' && chars[i].is_ascii_digit() {
            i -= 1;
            break;
        }
    }

    let book_str = input[book_start..i].trim().to_string();
    let rest = input[i..].trim().to_string();

    if book_str.is_empty() {
        return Err("No book name found".to_string());
    }

    Ok((book_str, rest))
}

/// Parse "3:16", "3:16-18", "3" into (chapter, verse_start, verse_end).
fn parse_chapter_verse(input: &str) -> Result<(u32, Option<u32>, Option<u32>), String> {
    let input = input.trim();

    if let Some((chapter_str, verse_part)) = input.split_once(':') {
        let chapter: u32 = chapter_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid chapter number: '{}'", chapter_str))?;

        if let Some((start_str, end_str)) = verse_part.split_once('-') {
            let start: u32 = start_str
                .trim()
                .parse()
                .map_err(|_| format!("Invalid verse number: '{}'", start_str))?;
            let end: u32 = end_str
                .trim()
                .parse()
                .map_err(|_| format!("Invalid verse number: '{}'", end_str))?;
            Ok((chapter, Some(start), Some(end)))
        } else {
            let verse: u32 = verse_part
                .trim()
                .parse()
                .map_err(|_| format!("Invalid verse number: '{}'", verse_part))?;
            Ok((chapter, Some(verse), Some(verse)))
        }
    } else {
        let chapter: u32 = input
            .parse()
            .map_err(|_| format!("Invalid chapter number: '{}'", input))?;
        Ok((chapter, None, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_verse() {
        let r = parse("Exodus 3:14").unwrap();
        assert_eq!(r.book.name, "Exodus");
        assert_eq!(r.chapter, 3);
        assert_eq!(r.verse_start, Some(14));
        assert_eq!(r.verse_end, Some(14));
    }

    #[test]
    fn test_verse_range() {
        let r = parse("Deut 6:4-9").unwrap();
        assert_eq!(r.book.name, "Deuteronomy");
        assert_eq!(r.chapter, 6);
        assert_eq!(r.verse_start, Some(4));
        assert_eq!(r.verse_end, Some(9));
    }

    #[test]
    fn test_whole_chapter() {
        let r = parse("Genesis 1").unwrap();
        assert_eq!(r.book.name, "Genesis");
        assert_eq!(r.chapter, 1);
        assert_eq!(r.verse_start, None);
    }

    #[test]
    fn test_hebrew_name_alias() {
        let r = parse("Bamidbar 6").unwrap();
        assert_eq!(r.book.name, "Numbers");
        assert_eq!(r.chapter, 6);
    }

    #[test]
    fn test_abbreviation() {
        let r = parse("Lev 19:18").unwrap();
        assert_eq!(r.book.name, "Leviticus");
        assert_eq!(r.chapter, 19);
        assert_eq!(r.verse_start, Some(18));
    }

    #[test]
    fn test_compact_input() {
        let r = parse("gen1:1").unwrap();
        assert_eq!(r.book.name, "Genesis");
        assert_eq!(r.chapter, 1);
        assert_eq!(r.verse_start, Some(1));
    }

    #[test]
    fn test_display() {
        let r = parse("Exodus 3:14").unwrap();
        assert_eq!(r.display(), "Exodus 3:14");

        let r = parse("Genesis 1").unwrap();
        assert_eq!(r.display(), "Genesis 1");

        let r = parse("Deut 6:4-9").unwrap();
        assert_eq!(r.display(), "Deuteronomy 6:4-9");
    }

    #[test]
    fn test_invalid_chapter() {
        assert!(parse("Genesis 51").is_err());
    }

    #[test]
    fn test_invalid_book() {
        assert!(parse("Notabook 1:1").is_err());
    }
}
