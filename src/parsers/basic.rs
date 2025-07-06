use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

const CORRUPTION: &[char] = &[
    '|', ':', '/', '?', '&', 'ᴗ', 'x', '◕', '.', '@', '#', '✿', '$', '%', '^', '*',
];

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

    fn corrupt_replace(&mut self, input: String, strength: u32) -> String {
        let mut res = String::new();

        for c in input.chars() {
            match self.rand.random::<u32>() % strength {
                0 => res.push(*CORRUPTION.choose(&mut self.rand).unwrap()),
                _ => res.push(c),
            }
        }

        res
    }

    fn corrupt_insert(&mut self, input: String, len: u32) -> String {
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
