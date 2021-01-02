use std::path::PathBuf;

pub struct Config {
    layout: String,
    phonetic_database: bool,
    phonetic_include_english: bool,
    database_dir: PathBuf,
    fixed_database: bool,
    fixed_vowel: bool,
    fixed_chandra: bool,
    fixed_kar: bool,
    fixed_old_reph: bool,
    fixed_numpad: bool,
}

impl Config {
    pub fn new_config(
        layout: String,
        phonetic_database: bool,
        phonetic_include_english: bool,
        database_dir: PathBuf,
        fixed_database: bool,
        fixed_vowel: bool,
        fixed_chandra: bool,
        fixed_kar: bool,
        fixed_old_reph: bool,
        fixed_numpad: bool,
    ) -> Config {
        Config {
            layout,
            phonetic_database,
            phonetic_include_english,
            database_dir,
            fixed_database,
            fixed_vowel,
            fixed_chandra,
            fixed_kar,
            fixed_old_reph,
            fixed_numpad,
        }
    }

    pub(crate) fn get_layout_file_path(&self) -> String {
        self.layout.clone()
    }

    pub(crate) fn get_database_path(&self) -> PathBuf {
        self.database_dir.join("dictionary.json")
    }

    pub(crate) fn get_suffix_data_path(&self) -> PathBuf {
        self.database_dir.join("suffix.json")
    }

    pub(crate) fn get_autocorrect_data(&self) -> PathBuf {
        self.database_dir.join("autocorrect.json")
    }

    pub(crate) fn get_phonetic_database(&self) -> bool {
        self.phonetic_database
    }

    pub(crate) fn get_phonetic_include_english(&self) -> bool {
        self.phonetic_include_english
    }
}

pub(crate) fn phonetic_method_defaults() -> Config {
    Config {
        layout: format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/data/avrophonetic.json"
        ),
        database_dir: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data").into(),
        phonetic_database: true,
        phonetic_include_english: true,
        fixed_database: false,
        fixed_vowel: false,
        fixed_chandra: false,
        fixed_kar: false,
        fixed_numpad: false,
        fixed_old_reph: false,
    }
}

pub(crate) fn fixed_method_defaults() -> Config {
    Config {
        layout: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/Probhat.json"),
        database_dir: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data").into(),
        fixed_database: true,
        fixed_vowel: true,
        fixed_chandra: true,
        fixed_kar: true,
        fixed_numpad: true,
        fixed_old_reph: true,
        phonetic_database: false,
        phonetic_include_english: false,
    }
}
