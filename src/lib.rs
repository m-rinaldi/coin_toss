#[derive(Copy, Clone, Debug)]
pub struct Coin;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CoinFace {
    Heads,
    Tails,
}

impl Coin {
    pub fn new() -> Coin {
        Coin
    }

    pub fn toss(&self) -> CoinFace {
        let outcome: bool = rand::random();

        use CoinFace::*;

        match outcome {
            true => Heads,
            false => Tails,
        }
    }
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
