use std::ops::Index;

use rand::prelude::*;
use rand::seq::IndexedRandom;

const CORRUPTION: &[char] = &[
    '|', ':', '/', '?', '&', 'ᴗ', 'x', '◕', '.', '@', '#', '✿', '$', '%', '^', '*',
];

#[derive(Debug)]
pub struct Aegyorruptor;

impl Aegyorruptor {
    pub fn parse(&self, input: String) -> String {
        let mut res = String::new();

        let mut cc = input.chars();

        while let Some(c) = cc.next() {
            res = match c {
                'b' => format!("૮{}ა", res),
                'c' => self.corrupt_insert(res, cc.next().unwrap_or('0').to_digit(10).unwrap()),
                'C' => self.corrupt_replace(res, cc.next().unwrap_or('0').to_digit(10).unwrap()),
                'd' => res + "▿",
                'e' => format!("◕{}&✿", res),
                'f' => format!("{}✿", res),
                'h' => format!("♡{}♡", res),
                'n' => format!("'.'.{}'.'.", res),
                'o' => res + ".",
                'p' => format!("({})", res),
                's' => res + "ᴗ",
                't' => format!("₍{}₎", res),
                'u' => format!("◡{}◡✿", res),
                _ => res,
            }
        }

        res
    }

    fn corrupt_replace(&self, input: String, strength: u32) -> String {
        let mut rng = rand::rng();

        let mut res = String::new();

        for c in input.chars() {
            match rng.random::<u32>() % strength {
                0 => res.push(*CORRUPTION.choose(&mut rng).unwrap()),
                _ => res.push(c),
            }
        }

        res
    }

    fn corrupt_insert(&self, input: String, len: u32) -> String {
        let mut rng = rand::rng();

        let mut res = input.clone();

        let ch_idx: Vec<usize> = input.char_indices().map(|(i, _)| i).collect();

        let pos = *ch_idx.index(rng.random::<u32>() as usize % res.chars().count());

        for _ in 0..len {
            res.insert(pos, *CORRUPTION.choose(&mut rng).unwrap());
        }

        res
    }
}
