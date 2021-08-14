use super::{Coin, CoinFace};

pub struct CoinIter(Coin);

impl Iterator for CoinIter {
    type Item = CoinFace;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.toss())
    }
}

impl IntoIterator for Coin {
    type Item = CoinFace;
    type IntoIter = CoinIter;

    fn into_iter(self) -> Self::IntoIter {
        CoinIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let coin = Coin::new();
        let iter = coin.into_iter();
        let times = 50_000;
        assert_eq!(times, iter.take(times).count());
    }
}