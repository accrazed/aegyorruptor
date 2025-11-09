use crate::corruption::CORRUPTION;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

#[derive(Debug)]
pub struct Aegyorruptor {
    rand: ThreadRng,
}

impl Aegyorruptor {
    pub fn new() -> Self {
        Aegyorruptor { rand: rand::rng() }
    }

    pub fn parse(&mut self, input: String) -> String {
        let mut res = String::new();
        let mut cc = input.chars();

        while let Some(c) = cc.next() {
            res = match c {
                'b' => format!("૮{}ა", res),
                'c' => self.corrupt_ins(res, cc.next().unwrap_or('0').to_digit(10).unwrap_or(1)),
                'C' => self.corrupt_repl(res, cc.next().unwrap_or('0').to_digit(10).unwrap_or(1)),
                'd' => format!("{}▿", res),
                'e' => format!("◕{}&", res),
                'f' => format!("{}✿", res),
                'F' => format!("˶{}˶", res),
                'h' => format!("♡{}♡", res),
                'i' => format!("{}※。*°☆", res),
                'n' => format!("'.'.{}'.'.", res),
                'o' => format!("{}.", res),
                'p' => format!("({})", res),
                's' => format!("{}ᴗ", res),
                'S' => format!("{}︿", res),
                't' => format!("₍{}₎", res),
                'u' => format!("◡{}◡", res),
                'w' => format!("{}﹏", res),
                'x' => format!("ﾒ{}ﾒ", res),
                _ => format!("{}{}", res, c),
            }
        }

        res
    }

    // corrupt_replace randomly replaces emoticon characters in-place with corrupted bytes
    fn corrupt_repl(&mut self, input: String, strength: u32) -> String {
        let mut res = String::new();

        for c in input.chars() {
            match self.rand.random_range(..strength) {
                0 => res.push(*CORRUPTION.choose(&mut self.rand).unwrap()),
                _ => res.push(c),
            }
        }

        res
    }

    // corrupt_insert inserts a block of corrupted bytes into a random index of the emoticon
    fn corrupt_ins(&mut self, input: String, len: u32) -> String {
        let mut res = input.clone();

        let pos = input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.rand.random_range(..res.chars().count()))
            .unwrap();

        for _ in 0..len {
            res.insert(pos, *CORRUPTION.choose(&mut self.rand).unwrap());
        }

        res
    }
}
