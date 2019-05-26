use serde_json::Value;
use std::env::var;
use std::fs::read_to_string;

use crate::context::Method;
use crate::fixed::chars::*;
use crate::fixed::parser::LayoutParser;
use crate::keycodes::*;
use crate::suggestion::Suggestion;
use crate::utility::get_modifiers;
use crate::utility::Utility;
use crate::ENV_LAYOUT;

const MARKS: &str = "`~!@#$%^+*-_=+\\|\"/;:,./?><()[]{}";

pub(crate) struct FixedMethod {
    buffer: String,
    handled: bool,
    parser: LayoutParser,
    layout: String,
}

impl Method for FixedMethod {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion {
        let modifier = get_modifiers(modifier);
        let (_, ctrl, alt) = modifier;
        // Don't catch Ctrl or Alt without AltGr combination.
        if (ctrl && !alt) || (!ctrl && alt) {
            // Handle edge cases
            if key == VC_SHIFT || key == VC_ALT {
                if !self.buffer.is_empty() {
                    self.handled = true;
                    return self.create_suggestion();
                } else {
                    self.handled = false;
                    return Suggestion::empty();
                }
            } else {
                self.handled = false;
                return Suggestion::empty();
            }
        }

        if key == VC_SHIFT || key == VC_CONTROL || key == VC_ALT {
            if !self.buffer.is_empty() {
                self.handled = true;
                return self.create_suggestion();
            } else {
                self.handled = false;
                return Suggestion::empty();
            }
        } else if key == VC_BACKSPACE {
            if !self.buffer.is_empty() {
                // Remove the last character from buffer.
                self.internal_backspace();
                self.handled = true;

                if !self.buffer.is_empty() {
                    return self.create_suggestion();
                } else {
                    // The buffer is now empty, so return empty suggestion.
                    return Suggestion::empty();
                }
            } else {
                self.handled = false;
                return Suggestion::empty();
            }
        }

        if let Some(value) = self.parser.get_char_for_key(key, modifier.into()) {
            self.process_key_value(&value);
            self.handled = true;
        } else {
            self.handled = false;
            return Suggestion::empty();
        }

        self.create_suggestion()
    }

    fn handle_special_key(&mut self, _key: u16) -> u8 {
        0
    }

    fn key_handled(&self) -> bool {
        self.handled
    }

    fn update_engine(&mut self) {
        let layout = var(ENV_LAYOUT).unwrap();

        // Check if the layout was changed.
        if self.layout != layout {
            let file = serde_json::from_str::<Value>(&read_to_string(&layout).unwrap()).unwrap();
            let parser = LayoutParser::new(file.get("layout").unwrap());
            self.parser = parser;
        }
    }
}

impl FixedMethod {
    /// Creates a new instance of `FixedMethod` with the layout which
    /// is set in the `RITI_LAYOUT_FILE` environment variable.
    pub(crate) fn new() -> Self {
        let layout = var(ENV_LAYOUT).unwrap();
        let file = serde_json::from_str::<Value>(&read_to_string(&layout).unwrap()).unwrap();
        let parser = LayoutParser::new(file.get("layout").unwrap());

        FixedMethod {
            buffer: String::new(),
            handled: false,
            parser,
            layout,
        }
    }

    fn create_suggestion(&self) -> Suggestion {
        Suggestion::new("".to_string(), vec![self.buffer.clone()])
    }

    /// Processes the `value` of the pressed key and updates the method's
    /// internal buffer which will be used when creating suggestion.
    fn process_key_value(&mut self, value: &str) {
        let rmc = self.buffer.chars().last().unwrap(); // Right most character

        // Zo fola insertion
        if value == "\u{09CD}\u{09AF}" {
            if rmc == B_R {
                self.buffer = format!("{}{}{}", self.buffer, ZWJ, value);
                return;
            } else {
                self.buffer.push_str(value);
                return;
            }
        }

        // Reph insertion
        if value == "\u{09B0}\u{09CD}" {
            self.insert_reph();
            return;
        }

        if let Some(character) = value.chars().nth(0) {
            // Kar insertion
            if character.is_kar() {
                // Automatic Vowel Forming
                if self.buffer.is_empty() || rmc.is_vowel() || MARKS.contains(rmc) {
                    match character {
                        B_AA_KAR => self.buffer.push(B_AA),
                        B_I_KAR => self.buffer.push(B_I),
                        B_II_KAR => self.buffer.push(B_II),
                        B_U_KAR => self.buffer.push(B_U),
                        B_UU_KAR => self.buffer.push(B_UU),
                        B_RRI_KAR => self.buffer.push(B_RRI),
                        B_E_KAR => self.buffer.push(B_E),
                        B_OI_KAR => self.buffer.push(B_OI),
                        B_O_KAR => self.buffer.push(B_O),
                        B_OU_KAR => self.buffer.push(B_OU),
                        _ => unreachable!(),
                    }
                    return;
                } else if rmc == B_CHANDRA {
                    // Automatic Fix of Chandra Position
                    self.internal_backspace();
                    self.buffer = format!("{}{}{}", self.buffer, character, B_CHANDRA);
                    return;
                } else if rmc == B_HASANTA {
                    // Vowel making with Hasanta + Kar
                    match character {
                        B_AA_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_AA);
                        }
                        B_I_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_I);
                        }
                        B_II_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_II);
                        }
                        B_U_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_U);
                        }
                        B_UU_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_UU);
                        }
                        B_RRI_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_RRI);
                        }
                        B_E_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_E);
                        }
                        B_OI_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_OI);
                        }
                        B_O_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_O);
                        }
                        B_OU_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_OU);
                        }
                        _ => unreachable!(),
                    }
                    return;
                } else if rmc.is_pure_consonant() {
                    // Traditional Kar Joining
                    // In UNICODE it is known as "Blocking Bengali Consonant-Vowel Ligature"
                    self.buffer = format!("{}{}{}", self.buffer, ZWNJ, character);
                    return;
                } else {
                    self.buffer.push(character);
                    return;
                }
            }

            // Hasanta
            if character == B_HASANTA && rmc == B_HASANTA {
                self.buffer.push(ZWNJ);
                return;
            }
        }

        self.buffer.push_str(value);
    }

    /// Checks if the Reph is moveable by the Reph insertion algorithm.
    ///
    /// `rmc`: Right most character.
    ///
    /// `len`: length of the text.
    fn is_reph_moveable(&self, rmc: char, len: usize) -> bool {
        if rmc.is_pure_consonant() {
            return true;
        } else if len > 2 {
            if rmc.is_vowel()
                && self
                    .buffer
                    .chars()
                    .skip(len - 2)
                    .nth(0)
                    .unwrap()
                    .is_pure_consonant()
            {
                return true;
            } else if rmc == B_CHANDRA {
                if self
                    .buffer
                    .chars()
                    .skip(len - 2)
                    .nth(0)
                    .unwrap()
                    .is_pure_consonant()
                {
                    return true;
                } else if len > 3
                    && self.buffer.chars().skip(len - 2).nth(0).unwrap().is_vowel()
                    && self
                        .buffer
                        .chars()
                        .skip(len - 3)
                        .nth(0)
                        .unwrap()
                        .is_pure_consonant()
                {
                    return true;
                }
            }
        }

        false
    }

    fn insert_reph(&mut self) {
        let rmc = self.buffer.chars().last().unwrap();
        let len = self.buffer.chars().count();
        let reph_moveable = self.is_reph_moveable(rmc, len);

        let mut encountered_constant = false;
        let mut encountered_vowel = false;
        let mut encountered_hasanta = false;
        let mut encountered_chandra = false;

        if reph_moveable {
            let cache = self.buffer.clone();
            let mut step = 0usize;

            for i in 1..=len {
                let character = cache.chars().skip(i).nth(0).unwrap();

                if character.is_pure_consonant() {
                    if encountered_constant && !encountered_hasanta {
                        break;
                    }
                    encountered_constant = true;
                    encountered_hasanta = false; // reset
                    step += 1;
                    continue;
                } else if character == B_HASANTA {
                    encountered_hasanta = true;
                    step += 1;
                    continue;
                } else if character.is_vowel() {
                    if encountered_vowel {
                        break;
                    }

                    if i == 1 || encountered_chandra {
                        encountered_vowel = true;
                        step += 1;
                        continue;
                    }

                    break;
                } else if character == B_CHANDRA {
                    if i == 1 {
                        encountered_chandra = true;
                        step += 1;
                        continue;
                    }
                    break;
                }
            }

            let temp: String = cache.chars().skip(step).collect();
            self.internal_backspace_step(step);
            self.buffer = format!("{}{}{}{}", self.buffer, B_R, B_HASANTA, temp);
        } else {
            self.buffer = format!("{}{}{}", self.buffer, B_R, B_HASANTA);
        }
    }

    /// Removes the last character from the buffer.
    fn internal_backspace(&mut self) {
        let len = self.buffer.chars().count() - 1;
        self.buffer = self.buffer.chars().take(len).collect();
    }

    /// Removes the last `n` character from the buffer.
    fn internal_backspace_step(&mut self, n: usize) {
        let len = self.buffer.chars().count() - n;
        self.buffer = self.buffer.chars().take(len).collect();
    }
}

// Implement Default trait on FixedMethod for testing convenience.
impl Default for FixedMethod {
    fn default() -> Self {
        FixedMethod::new()
    }
}

#[cfg(test)]
mod tests {

    use super::FixedMethod;
    use crate::context::Method;

    use crate::keycodes::VC_BACKSPACE;
    use crate::ENV_LAYOUT;
    use std::env::set_var;
    #[test]
    fn test_backspace() {
        set_var(
            ENV_LAYOUT,
            format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/Probhat.json"),
        );

        let mut method = FixedMethod {
            buffer: "আমি".to_string(),
            ..Default::default()
        };

        assert!(!method.get_suggestion(VC_BACKSPACE, 0).is_empty()); // আম
        assert!(!method.get_suggestion(VC_BACKSPACE, 0).is_empty()); // আ
        assert!(method.get_suggestion(VC_BACKSPACE, 0).is_empty()); // Empty
    }
}
