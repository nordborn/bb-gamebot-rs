use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Suite {
    Rock,
    Scissors,
    Paper,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub suite: Suite,
    pub power: isize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    #[serde(alias="aiCards")]
    pub ai_cards: Vec<Card>,
    #[serde(alias="humanCards")]
    pub human_cards: Vec<Card>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_card_from_json() -> Result<(), Box<dyn Error>> {
        let s = r#"{"id":"123", "suite":"Paper", "power":10}"#;
        let c: Card = serde_json::from_str(s)?;
        assert_eq!(c.id, "123");
        assert_eq!(c.suite, Suite::Paper);
        Ok(())
    }
}
