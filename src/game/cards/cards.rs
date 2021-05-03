use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlackCard {
    pub text: String,
    pub pick: u8,
}

impl Display for BlackCard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\" (Pick {})", self.text, self.pick)
    }
}

impl Clone for BlackCard {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            pick: self.pick,
        }
    }
}

impl PartialEq for BlackCard {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.pick == other.pick
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhiteCard {
    pub text: String,
}

impl Display for WhiteCard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.text)
    }
}

impl Clone for WhiteCard {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
        }
    }
}

impl PartialEq for WhiteCard {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub fn load_from_json(json_str: &str) -> Result<(Vec<BlackCard>, Vec<WhiteCard>), &'static str> {
    let card_data_json: Value = serde_json::from_str(json_str).unwrap();
    let card_data = match card_data_json {
        Value::Object(card_objects) => Ok(card_objects),
        _ => Err("Failed to parse cards JSON."),
    }
    .unwrap();

    let black_card_data = card_data["blackCards"].clone();
    let black_cards: Vec<BlackCard> = serde_json::from_value(black_card_data).unwrap();

    let white_card_data = card_data["whiteCards"].clone();
    let white_card_texts: Vec<String> = serde_json::from_value(white_card_data).unwrap();
    let white_cards = white_card_texts
        .iter()
        .map(|txt| WhiteCard {
            text: txt.to_owned(),
        })
        .collect();

    Ok((black_cards, white_cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_json() {
        let sample_json = r#"
        {
            "blackCards": [
                {
                    "text": "Black card 1",
                    "pick": 1
                },
                {
                    "text": "Black card 2",
                    "pick": 1
                }
            ],
            "whiteCards": [
                "White card text 1",
                "White card text 2",
                "White card text 3"
            ]
        }
        "#;

        let expected_black_cards = vec![
            BlackCard {
                text: "Black card 1".to_owned(),
                pick: 1,
            },
            BlackCard {
                text: "Black card 2".to_owned(),
                pick: 1,
            },
        ];

        let expected_white_cards = vec![
            WhiteCard {
                text: "White card text 1".to_owned(),
            },
            WhiteCard {
                text: "White card text 2".to_owned(),
            },
            WhiteCard {
                text: "White card text 3".to_owned(),
            },
        ];

        let (black_cards, white_cards) = load_from_json(sample_json).unwrap();
        assert_eq!(black_cards, expected_black_cards);
        assert_eq!(white_cards, expected_white_cards);
    }
}
