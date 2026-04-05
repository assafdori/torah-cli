/// The 5 books of the Torah (Pentateuch) with common abbreviations.
#[derive(Debug, Clone)]
pub struct BookInfo {
    pub name: &'static str,
    pub abbrevs: &'static [&'static str],
    pub id: u32,
    pub chapters: u32,
}

pub static BOOKS: &[BookInfo] = &[
    BookInfo {
        name: "Genesis",
        abbrevs: &["gen", "ge", "gn", "bere", "bereshit"],
        id: 1,
        chapters: 50,
    },
    BookInfo {
        name: "Exodus",
        abbrevs: &["exo", "ex", "exod", "shem", "shemot"],
        id: 2,
        chapters: 40,
    },
    BookInfo {
        name: "Leviticus",
        abbrevs: &["lev", "le", "lv", "vay", "vayikra"],
        id: 3,
        chapters: 27,
    },
    BookInfo {
        name: "Numbers",
        abbrevs: &["num", "nu", "nm", "nb", "bam", "bamidbar"],
        id: 4,
        chapters: 36,
    },
    BookInfo {
        name: "Deuteronomy",
        abbrevs: &["deu", "de", "dt", "deut", "dev", "devarim"],
        id: 5,
        chapters: 34,
    },
];

/// Normalize a book name input to the canonical book name.
/// Handles full names, abbreviations, case insensitivity, and numbered books.
pub fn normalize_book(input: &str) -> Option<&'static BookInfo> {
    let input = input.trim().to_lowercase();
    let input = input.replace('.', "");

    // Try exact full name match first
    for book in BOOKS {
        if book.name.to_lowercase() == input {
            return Some(book);
        }
    }

    // Try abbreviation match
    for book in BOOKS {
        for abbrev in book.abbrevs {
            if *abbrev == input {
                return Some(book);
            }
        }
    }

    // Try prefix match on full name (e.g., "gene" -> "Genesis")
    BOOKS
        .iter()
        .find(|book| input.len() >= 3 && book.name.to_lowercase().starts_with(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_name() {
        assert_eq!(normalize_book("Genesis").unwrap().name, "Genesis");
        assert_eq!(normalize_book("deuteronomy").unwrap().name, "Deuteronomy");
    }

    #[test]
    fn test_abbreviation() {
        assert_eq!(normalize_book("gen").unwrap().name, "Genesis");
        assert_eq!(normalize_book("shemot").unwrap().name, "Exodus");
        assert_eq!(normalize_book("vay").unwrap().name, "Leviticus");
        assert_eq!(normalize_book("deut").unwrap().name, "Deuteronomy");
    }

    #[test]
    fn test_prefix_match() {
        assert_eq!(normalize_book("gene").unwrap().name, "Genesis");
        assert_eq!(normalize_book("deut").unwrap().name, "Deuteronomy");
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(normalize_book("BAMIDBAR").unwrap().name, "Numbers");
        assert_eq!(normalize_book("GEN").unwrap().name, "Genesis");
    }

    #[test]
    fn test_invalid() {
        assert!(normalize_book("notabook").is_none());
        assert!(normalize_book("xyz").is_none());
    }
}
