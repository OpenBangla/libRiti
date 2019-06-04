use crate::context::{MODIFIER_ALT, MODIFIER_CTRL, MODIFIER_SHIFT};

/// Some utility functions which we implement on the `char` type.
pub(crate) trait Utility {
    /// Checks the char for a vowel character.
    fn is_vowel(&self) -> bool;
    /// Checks the char for a kar character.
    fn is_kar(&self) -> bool;
    /// Checks the char for a pure consonant character.
    fn is_pure_consonant(&self) -> bool;
}

impl Utility for char {
    /// Checks the char for a vowel character.
    fn is_vowel(&self) -> bool {
        "\u{0985}\u{0986}\u{0987}\u{0988}\u{0989}\u{098A}\u{098B}\u{098F}\u{0990}\u{0993}\u{0994}\u{098C}\u{09E1}\u{09BE}\u{09BF}\u{09C0}\u{09C1}\u{09C2}\u{09C3}\u{09C7}\u{09C8}\u{09CB}\u{09CC}".contains(*self)
    }

    /// Checks the char for a kar character.
    fn is_kar(&self) -> bool {
        "\u{09BE}\u{09BF}\u{09C0}\u{09C1}\u{09C2}\u{09C3}\u{09C7}\u{09C8}\u{09CB}\u{09CC}\u{09C4}"
            .contains(*self)
    }

    /// Checks the char for a pure consonant character.
    fn is_pure_consonant(&self) -> bool {
        "\u{0995}\u{0996}\u{0997}\u{0998}\u{0999}\u{099A}\u{099B}\u{099C}\u{099D}\u{099E}\u{099F}\u{09A0}\u{09A1}\u{09A2}\u{09A3}\u{09A4}\u{09A5}\u{09A6}\u{09A7}\u{09A8}\u{09AA}\u{09AB}\u{09AC}\u{09AD}\u{09AE}\u{09AF}\u{09B0}\u{09B2}\u{09B6}\u{09B7}\u{09B8}\u{09B9}\u{09CE}\u{09DC}\u{09DD}\u{09DF}".contains(*self)
    }
}

/// Tuple of modifier keys.
///
/// First  is Shift, second is Ctrl and third is Alt.
pub(crate) type Modifiers = (bool, bool, bool);

/// Returns boolean tuples of the modifiers from the bit masked integer `modifier`.
/// First  is Shift, second is Ctrl and third is Alt.
pub(crate) fn get_modifiers(modifier: u8) -> Modifiers {
    let shift = (modifier & MODIFIER_SHIFT) == MODIFIER_SHIFT;
    let ctrl = (modifier & MODIFIER_CTRL) == MODIFIER_CTRL;
    let alt = (modifier & MODIFIER_ALT) == MODIFIER_ALT;

    (shift, ctrl, alt)
}

#[macro_export]
/// A helper macro for initializing HashMap.
/// Originally from the `maplit` crate but customized for use
/// with HashMap associated with `FxHasher`.
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = std::collections::HashMap::with_capacity_and_hasher(_cap, std::hash::BuildHasherDefault::<rustc_hash::FxHasher>::default());
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[cfg(test)]
mod test {
    use super::get_modifiers;
    use super::Utility;
    use crate::context::{MODIFIER_ALT, MODIFIER_CTRL, MODIFIER_SHIFT};
    
    #[test]
    fn test_utilities() {
        assert!('আ'.is_vowel());
        assert!(!'ক'.is_vowel());
        assert!('া'.is_kar());
        assert!(!'আ'.is_kar());
        assert!('ক'.is_pure_consonant());
    }

    #[test]
    fn test_get_modifiers() {
        assert_eq!(get_modifiers(MODIFIER_SHIFT), (true, false, false));
        assert_eq!(get_modifiers(MODIFIER_CTRL), (false, true, false));
        assert_eq!(get_modifiers(MODIFIER_ALT), (false, false, true));
        assert_eq!(
            get_modifiers(MODIFIER_SHIFT | MODIFIER_CTRL | MODIFIER_ALT),
            (true, true, true)
        );
    }
}
