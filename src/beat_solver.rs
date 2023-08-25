use crate::game_types::Suite;
use anyhow::Result;

pub fn beater(suite: Suite) -> Suite {
    use Suite::*;
    match suite {
        Paper => Scissors,
        Rock => Paper,
        Scissors => Rock,
    }
}

pub fn beaters(opponent: &Vec<Suite>, my: &Vec<Suite>) -> Result<Vec<Suite>> {
    // BAD CASE - my reached end before opponent's
    if opponent.len() != my.len() {
        anyhow::bail!("beaters: unequal length");
    }
    let mut ret: Vec<Suite> = Vec::with_capacity(my.len());
    let mut my = my.clone();
    for &op_card in opponent {
        match peek_card(beater(op_card), &my) {
            (Some(suite), my1) => {
                ret.push(suite);
                my = my1;
            }
            (None, _) => {
                // peek same if possible
                match peek_card(op_card, &my) {
                    (Some(suite), my1) => {
                        ret.push(suite);
                        my = my1;
                    }
                    (None, _) => {
                        // add any last (WAS FIRST)
                        ret.push(my[my.len() - 1]);
                        my.remove(0);
                    }
                }
            }
        }
    }
    Ok(ret)
}

/// peek_card peeks card from deck and returns the card and updated deck
/// if no card in deck, then Nothing returned and deck is the same
fn peek_card(suite: Suite, deck: &[Suite]) -> (Option<Suite>, Vec<Suite>) {
    let mut deck1 = deck.to_owned();
    if let Some(ix) = deck.iter().position(|&x| x == suite) {
        deck1.remove(ix);
        return (Some(suite), deck1);
    }
    (None, deck1)
}

#[cfg(test)]
mod test {
    use super::*;
    use Suite::*;

    #[test]
    fn test_beater() {
        assert_eq!(beater(Rock), Paper);
        assert_eq!(beater(Paper), Scissors);
        assert_eq!(beater(Scissors), Rock);
    }

    #[test]
    fn test_peek_card() {
        let v = vec![Rock, Paper, Paper, Rock];
        assert_eq!(peek_card(Rock, &v), (Some(Rock), vec![Paper, Paper, Rock]));
        assert_ne!(peek_card(Rock, &v), (Some(Rock), vec![Rock, Paper, Paper]));
        assert_eq!(peek_card(Scissors, &v), (None, v.clone()));
    }

    #[test]
    fn test_beaters() -> Result<()> {
        let opp = vec![Rock, Paper, Scissors];
        let my = vec![Rock, Paper, Scissors];
        assert_eq!(beaters(&opp, &my)?, vec![Paper, Scissors, Rock]);
        assert_eq!(
            beaters(&opp, &vec![Rock, Rock, Paper])?,
            vec![Paper, Rock, Rock]
        );

        let opp1 = vec![Rock, Paper, Scissors, Paper];
        let my1 = vec![Rock, Paper, Scissors, Paper];
        assert_eq!(beaters(&opp1, &my1)?, vec![Paper, Scissors, Rock, Paper]);

        let opp2 = vec![Rock, Paper, Scissors, Paper];
        let my2 = vec![Rock, Paper, Scissors, Rock];
        assert_eq!(beaters(&opp2, &my2)?, vec![Paper, Scissors, Rock, Rock]);

        let opp3 = vec![Rock, Paper, Scissors, Paper];
        let my3 = vec![Scissors, Paper, Scissors, Rock];
        assert_eq!(beaters(&opp3, &my3)?, vec![Paper, Scissors, Rock, Scissors]);

        Ok(())
    }
}
