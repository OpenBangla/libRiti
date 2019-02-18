use regex::Regex;
use serde_json;
use rustc_hash::FxHashMap;
use crate::phonetic::regex::PhoneticRegex;

pub(crate) struct Database {
    regex: PhoneticRegex,
    table: FxHashMap<String, Vec<String>>,
    suffix: FxHashMap<String, String>,
    autocorrect: FxHashMap<String, String>,
}

impl Database {
    pub(crate) fn new() -> Database {
        Database {
            regex: PhoneticRegex::new(),
            table: serde_json::from_str(include_str!("dictionary.json")).unwrap(),
            suffix: serde_json::from_str(include_str!("suffix.json")).unwrap(),
            autocorrect: serde_json::from_str(include_str!("autocorrect.json")).unwrap(),
        }
    }

    /// Find words from the dictionary with given hint and a Regex
    pub(crate) fn find(&self, word: &str) -> Vec<String> {
        let rgx = Regex::new(&self.regex.parse(word)).unwrap();

        let table = match &word[0..1] {
            "a" => vec!["a", "aa", "e", "oi", "o", "nya", "y"],
            "b" => vec!["b", "bh"],
            "c" => vec!["c", "ch", "k"],
            "d" => vec!["d", "dh", "dd", "ddh"],
            "e" => vec!["i", "ii", "e", "y"],
            "f" => vec!["ph"],
            "g" => vec!["g", "gh", "j"],
            "h" => vec!["h"],
            "i" => vec!["i", "ii", "y"],
            "j" => vec!["j", "jh", "z"],
            "k" => vec!["k", "kh"],
            "l" => vec!["l"],
            "m" => vec!["h", "m"],
            "n" => vec!["n", "nya", "nga", "nn"],
            "o" => vec!["a", "u", "uu", "oi", "o", "ou", "y"],
            "p" => vec!["p", "ph"],
            "q" => vec!["k"],
            "r" => vec!["rri", "h", "r", "rr", "rrh"],
            "s" => vec!["s", "sh", "ss"],
            "t" => vec!["t", "th", "tt", "tth", "khandatta"],
            "u" => vec!["u", "uu", "y"],
            "v" => vec!["bh"],
            "w" => vec!["o"],
            "x" => vec!["e", "k"],
            "y" => vec!["i", "y"],
            "z" => vec!["h", "j", "jh", "z"],
            _ => vec![],
        };

        table
            .iter()
            .flat_map(|&item| {
                self.table[item].iter().filter_map(|i| {
                    if rgx.is_match(i) {
                        Some(i.to_owned())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub(crate) fn find_suffix(&self, string: &str) -> String {
        self.suffix.get(string).unwrap_or(&String::new()).to_owned()
    }

    /// Get the phonetically corrected string from auto-correct dictionary. 
    pub(crate) fn get_corrected(&self, string: &str) -> String {
        self.autocorrect.get(string).unwrap_or(&String::new()).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    #[test]
    fn test_database() {
        let db = Database::new();
        assert_eq!(
            db.find("a"),
            [
                "অ্যা",
                "অ্যাঁ",
                "আ",
                "আঃ",
                "া",
                "এ",
            ]
        );
    }

    #[test]
    fn test_suffix() {
        let db = Database::new();
        assert_eq!(db.find_suffix("gulo"), "গুলো");
        assert_eq!(db.find_suffix("er"), "ের");
        assert_eq!(db.find_suffix("h"), "");
    }

    #[test]
    fn test_autocorrect() {
        let db = Database::new();
        assert_eq!(db.get_corrected("academy"), "oZakaDemi");
        assert_eq!(db.get_corrected("\\nai\\"), "");
    }
}