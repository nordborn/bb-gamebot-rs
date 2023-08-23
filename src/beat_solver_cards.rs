use crate::game_types::{Card, Suite};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn card_suites(cc: &Vec<Card>) -> Vec<Suite> {
    cc.iter().map(|c| c.suite).collect()
}

pub fn shuffle_respecting_power(cc: &Vec<Card>) -> Vec<Card> {
    let mut ret: Vec<Card> = Vec::new();
    let mut rare8: Vec<Card> = Vec::new();
    let mut rare4: Vec<Card> = Vec::new();
    let mut rare2: Vec<Card> = Vec::new();
    let mut common: Vec<Card> = Vec::new();
    for c in cc {
        match c.power {
            1 => common.push(c.clone()),
            2 => rare2.push(c.clone()),
            4 => rare4.push(c.clone()),
            8 => rare8.push(c.clone()),
            _ => rare2.push(c.clone()),
        }
    }
    let mut rng = thread_rng();
    rare8.shuffle(&mut rng);
    rare4.shuffle(&mut rng);
    rare2.shuffle(&mut rng);
    common.shuffle(&mut rng);
    ret.append(&mut rare8);
    ret.append(&mut rare4);
    ret.append(&mut rare2);
    ret.append(&mut common);
    ret
}

#[allow(dead_code)]
pub fn shuffle_respecting_power_hm(cc: &Vec<Card>) -> Vec<Card> {
    let mut rng = thread_rng();
    let mut ret: Vec<Card> = Vec::new();
    let mut rhm: RefCell<HashMap<isize, Vec<Card>>> = RefCell::new(HashMap::new());
    // fill hashmap <k: power, v: [Card]>
    for c in cc {
        rhm.get_mut()
            .entry(c.power.clone())
            .and_modify(|v| v.push(c.clone()))
            .or_insert(vec![c.clone()]);
    }
    // shuffle each v: [Card]
    for k in rhm.clone().get_mut().keys() {
        if let Some(v) = rhm.get_mut().get_mut(k) {
            v.shuffle(&mut rng);
        }
    }
    // desc keys == power
    let mut rhm_cloned = rhm.clone();
    let mut keys: Vec<&isize> = rhm_cloned.get_mut().keys().collect();
    keys.sort_by(|&a, &b| b.cmp(a)); // reverse

    // fill ret vector from higher to lower powers
    for k in keys {
        if let Some(v) = rhm.get_mut().get_mut(k) {
            ret.append(v);
        }
    }
    println!("{:?}", ret);
    ret
}

pub fn lowest_power_card(cc: &Vec<Card>, s: Suite) -> Option<Card> {
    let mut ret: Option<Card> = None;
    for c in cc.iter() {
        if c.suite != s {
            continue;
        }
        // found smallest - ok, interrupt
        if c.power == 1 {
            ret = Some(c.clone());
            break;
        }
        match &ret {
            // 1st card - use this
            None => ret = Some(c.clone()),
            // found smaller, use this
            Some(r) => {
                if c.power < r.power {
                    ret = Some(c.clone())
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use Suite::*;

    #[test]
    fn test_shuffle() {
        let mut rng = thread_rng();
        let mut v = vec![1_i32, 2, 3, 4, 5, 6];
        v.shuffle(&mut rng);
        assert_ne!(v, vec![1, 2, 3, 4, 5, 6]);
        dbg!(v);
    }

    #[test]
    fn test_shuffle_respecting_power() {
        let cc = vec![
            Card {
                id: "1".into(),
                suite: Paper,
                power: 1,
            },
            Card {
                id: "2".into(),
                suite: Paper,
                power: 2,
            },
            Card {
                id: "3".into(),
                suite: Scissors,
                power: 1,
            },
            Card {
                id: "4".into(),
                suite: Rock,
                power: 8,
            },
        ];
        let cc1 = shuffle_respecting_power(&cc);
        assert_eq!(
            cc1[0],
            Card {
                id: "4".into(),
                suite: Rock,
                power: 8
            }
        );
        assert_ne!(cc1, cc);
        dbg!(cc1);
    }

    #[test]
    fn test_shuffle_respecting_power_hm() {
        let cc = vec![
            Card {
                id: "4".into(),
                suite: Paper,
                power: 5,
            },
            Card {
                id: "2".into(),
                suite: Paper,
                power: 2,
            },
            Card {
                id: "3".into(),
                suite: Paper,
                power: 2,
            },
            Card {
                id: "4".into(),
                suite: Paper,
                power: 2,
            },
            Card {
                id: "1".into(),
                suite: Paper,
                power: 1,
            },
        ];
        let cc1 = shuffle_respecting_power_hm(&cc);
        assert_eq!(cc1[1].power, 2);
        assert_ne!(cc1, cc);
    }

    #[test]
    fn test_lowest_power_card() {
        let cc = vec![
            Card {
                id: "4".into(),
                suite: Paper,
                power: 5,
            },
            Card {
                id: "2".into(),
                suite: Paper,
                power: 2,
            },
            Card {
                id: "4".into(),
                suite: Paper,
                power: 2,
            },
            Card {
                id: "1".into(),
                suite: Paper,
                power: 1,
            },
        ];
        assert_eq!(
            lowest_power_card(&cc, Paper),
            Some(Card {
                id: "1".into(),
                suite: Paper,
                power: 1
            })
        );
        assert_eq!(lowest_power_card(&cc, Scissors), None);
    }
}
