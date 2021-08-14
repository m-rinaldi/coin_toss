use rand::prelude::*;
use rand_xoshiro::SplitMix64;
use rand_core::SeedableRng;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Coin {
    prng: RefCell<SplitMix64>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CoinFace {
    Heads,
    Tails,
}

impl Coin {
    pub fn new() -> Self {
        Coin {
            prng: RefCell::new(SplitMix64::from_entropy()),
        }
    }

    pub fn from_seed(seed: u64) -> Self {
        Coin {
            prng: RefCell::new(SplitMix64::seed_from_u64(seed)),
        }
    }

    pub fn toss(&self) -> CoinFace {
        let outcome;
        {
            let mut prng = self.prng.borrow_mut();
            outcome = prng.gen();
        }

        use CoinFace::*;

        match outcome {
            true => Heads,
            false => Tails,
        }
    }
}

impl IntoIterator for Coin {
    // TODO
}

impl CoinFace {
    pub fn flip(&mut self) {
        use CoinFace::*;
        *self = match *self {
            Heads => Tails,
            Tails => Heads,
        }
    }
}

impl std::ops::Not for CoinFace {
    type Output = Self;
    fn not(mut self) -> Self {
        self.flip();
        self
    }
}

impl std::fmt::Display for CoinFace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let coin_str = match self {
            CoinFace::Heads => "O",
            CoinFace::Tails => "X",
        };
        write!(f, "{}", coin_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use CoinFace::*;

    #[test]
    fn test_toss() {
        let coin = Coin::new();
        let outcome = coin.toss();

        matches!(outcome, Heads | Tails);
    }

    #[test]
    fn test_toss_with_same_seed() {
        let (c1, c2) = {
            let seed = 113;
            (Coin::from_seed(seed), Coin::from_seed(seed))
        };

        let (mut v1, mut v2);
        {
            let create_vec = || Vec::with_capacity(2048);
            v1 = create_vec();
            v2 = create_vec();
        };

        for _ in 0..v1.capacity() {
            v1.push(c1.toss());
            v2.push(c2.toss());
        }

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_flip() {
        use CoinFace::*;
        let mut face = Heads;

        face.flip();
        assert_eq!(face, Tails);

        face.flip();
        assert_eq!(face, Heads);
    }

    #[test]
    fn test_not() {
        use CoinFace::*;
        let face = Heads;

        assert_eq!(!face, Tails);
        assert_eq!(!!face, Heads);
    }

    #[test]
    fn test_distribution() {
        let coin = Coin::new();

        let mut heads_count = 0usize;
        let mut tails_count = 0usize;

        for _ in 0..100_000_000 {
            match coin.toss() {
                Heads => heads_count += 1,
                Tails => tails_count += 1,
            };
        }
 
        let ratio = (heads_count as f64) / (tails_count as f64);
        let epsilon = 0.001;
        assert!(1. - epsilon < ratio);
        assert!(1. + epsilon > ratio);
    }

    #[test]
    fn test_display() {
        let heads_str = format!("{}", Heads);
        assert_eq!("O", heads_str);

        let tails_str = format!("{}", Tails);
        assert_eq!("X", tails_str);
    }
}
