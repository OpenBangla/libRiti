use std::env::var;
use std::path::PathBuf;

pub(crate) const ENV_LAYOUT_FILE: &str = "RITI_LAYOUT_FILE";
pub(crate) const ENV_PHONETIC_DATABASE_DIR: &str = "RITI_PHONETIC_DATABASE_DIR";
pub(crate) const ENV_LAYOUT_FIXED_VOWEL: &str = "RITI_LAYOUT_FIXED_VOWEL";
pub(crate) const ENV_LAYOUT_FIXED_CHANDRA: &str = "RITI_LAYOUT_FIXED_CHANDRA";
pub(crate) const ENV_LAYOUT_FIXED_KAR: &str = "RITI_LAYOUT_FIXED_KAR";

/// Get file path of the selected layout.
pub(crate) fn get_settings_layout_file() -> String {
    var(ENV_LAYOUT_FILE).unwrap()
}

/// Get the base file path of database directory.
pub(crate) fn get_settings_phonetic_database_dir() -> PathBuf {
    var(ENV_PHONETIC_DATABASE_DIR).unwrap().into()
}

/// Get file path of user defined Auto Correct file.
pub(crate) fn get_settings_user_phonetic_autocorrect() -> String {
    let base = var("XDG_DATA_HOME").unwrap_or_else(|_| {
        format!("{}{}", var("HOME").unwrap(), "/.local/share")
    });

    format!("{}{}", base, "/openbangla-keyboard/autocorrect.json")
}

/// Get file path of user defined phonetic candidate selection file.
pub(crate) fn get_settings_user_phonetic_selection_data() -> String {
    let base = var("XDG_DATA_HOME").unwrap_or_else(|_| {
        format!("{}{}", var("HOME").unwrap(), "/.local/share")
    });

    format!("{}{}", base, "/openbangla-keyboard/phonetic-candidate-selection.json")
}

/// Check if the Automatic Vowel Forming of 
/// Fixed Keyboard layout method feature is enabled. 
pub(crate) fn get_settings_fixed_automatic_vowel() -> bool {
    var(ENV_LAYOUT_FIXED_VOWEL).unwrap().parse().unwrap()
}

/// Check if the Automatic Chandra position of 
/// Fixed Keyboard layout method feature is enabled. 
pub(crate) fn get_settings_fixed_automatic_chandra() -> bool {
    var(ENV_LAYOUT_FIXED_CHANDRA).unwrap().parse().unwrap()
}

/// Check if the Traditional Kar Joining of 
/// Fixed Keyboard layout method feature is enabled. 
pub(crate) fn get_settings_fixed_traditional_kar() -> bool {
    var(ENV_LAYOUT_FIXED_KAR).unwrap().parse().unwrap()
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env::set_var;
    use super::*;

    /// Sets default settings for testing Phonetic Method.
    pub(crate) fn set_default_phonetic() {
        set_var(
            ENV_LAYOUT_FILE,
            format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/avrophonetic.json"),
        );
        set_var(ENV_PHONETIC_DATABASE_DIR, format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data"));
    }

    /// Sets default settings for testing Fixed Method.
    pub(crate) fn set_defaults_fixed() {
        set_var(
            ENV_LAYOUT_FILE,
            format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/Probhat.json"),
        );
        set_var(ENV_LAYOUT_FIXED_VOWEL, "true");
        set_var(ENV_LAYOUT_FIXED_CHANDRA, "true");
        set_var(ENV_LAYOUT_FIXED_KAR, "true");
    }

    #[test]
    fn test_get_bools() {
        set_var(ENV_LAYOUT_FIXED_VOWEL, "true");
        set_var(ENV_LAYOUT_FIXED_CHANDRA, "false");
        set_var(ENV_LAYOUT_FIXED_KAR, "true");

        assert!(get_settings_fixed_automatic_vowel());
        assert!(!get_settings_fixed_automatic_chandra());
        assert!(get_settings_fixed_traditional_kar());
    }
}
