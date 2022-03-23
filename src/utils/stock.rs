use rand::{Rng, SeedableRng, rngs::StdRng};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Symbol {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
}

pub fn get_price(symbol: &Symbol, date: u32) -> f32 {
    let mut rng1 = StdRng::seed_from_u64(date as u64);
    let mut rng2 = StdRng::seed_from_u64(*symbol as u64);
    return rng1.gen::<f32>() / rng2.gen::<f32>()

}

#[derive(Debug, Clone, Copy)]
pub struct StockData {
    quantity: u32,
    purchase_date: u32 // date in EPOCH
}

impl StockData {
    pub fn new(quantity: u32, purchase_date: u32) -> Self {
        StockData {
            quantity,
            purchase_date,
        }
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn get_purchase_date(&self) -> u32 {
        self.purchase_date
    }
}

#[cfg(test)]
mod rng_test {
    use super::*;
    #[test]
    fn generator_eq() {
        assert!((get_price(&Symbol::A, 12345) - get_price(&Symbol::A, 12345)).abs() <= f32::EPSILON);
    }

    #[test]
    #[should_panic]
    fn generator_neq() {
        assert!((get_price(&Symbol::A, 12345) - get_price(&Symbol::B, 12345)).abs() <= f32::EPSILON);
    }
}
