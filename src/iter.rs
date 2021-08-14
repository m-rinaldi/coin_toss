use super::{Coin, CoinFace};

pub struct CoinIter;

impl Iterator for CoinIter {
    type Item = CoinFace;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl IntoIterator for Coin {
    type Item = CoinFace;
    type IntoIter = CoinIter;

    fn into_iter(self) -> Self::IntoIter {
        CoinIter {}
    }
}