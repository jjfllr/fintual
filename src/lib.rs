// Construct a simple Portfolio class that has a collection of Stocks and a “Profit” method that
// receives 2 dates and returns the profit of the Portfolio between those dates. Assume each Stock
// has a “Price” method that receives a date and returns its price. Bonus Track: make the Profit
// method return the “annualized return” of the portfolio between the given dates.
mod utils;
use crate::utils::stock::*;

use std::collections::HashMap;
use error_chain::error_chain;
error_chain! {
    foreign_links {

    }
}

#[cfg(test)]
mod portfolio_tests {
    use super::*;

    #[test]
    fn create_portfolio() {
        let mut p = Portfolio::new(000);
        match p.add_stock(Symbol::A, 100, 12345) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };

        match p.add_stock(Symbol::C, 200, 23455) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };

        match p.add_stock(Symbol::G, 300, 34512) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };

        assert!(!p.get_stockdata(Symbol::B).is_some());

        let tmp = match p.get_stockdata(Symbol::A) {
            Some(data) => data,
            None => panic!(),
        };
        assert_eq!(100, tmp.get_quantity());
        assert_eq!(12345, tmp.get_purchase_date());
    }

    #[test]
    fn test_profit(){
        let mut p = Portfolio::new(000);
        match p.add_stock(Symbol::A, 100, 12345) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };

        match p.add_stock(Symbol::C, 200, 23455) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };

        match p.add_stock(Symbol::G, 300, 34512) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };

        println!("{:?}", p.profit(11111, 22222));
        println!("{:?}", p.annualized_rate_of_return(11111, 22222));
    }
}

#[derive(Debug, Clone)]
pub struct Portfolio {
    owner: u32,
    portfolio: HashMap<Symbol, StockData>,
}

impl Portfolio {
    pub fn new(owner: u32) -> Self {
        let port: HashMap<Symbol, StockData> = HashMap::new();
        Portfolio {
            owner,
            portfolio: port,
        }
    }

    pub fn add_stock(&mut self, symbol: Symbol, quantity: u32, date: u32) -> Result<()> {
        match self.portfolio.get(&symbol) {
            Some(_) => {return Err("Stock already in Portfolio".into())},
            None => {self.portfolio.insert(symbol, StockData::new(quantity, date));},
        }

        Ok(())
    }

    pub fn modify_stock(&mut self, symbol:Symbol, quantity:u32, date: u32) -> Result<()> {
        match self.portfolio.get(&symbol) {
            Some(_) => {
                self.portfolio.insert(symbol, StockData::new(quantity, date));
            },
            None => {return Err("Stock is not in portfolio".into())},
        }

        Ok(())
    }

    pub fn get_stockdata<'a>(&'a self, symbol: Symbol) -> Option< &'a StockData> {
        return self.portfolio.get(&symbol)
    }

    // we will use the formula
    // Profit = Sum(CSN_i * CSP_i - OSN_i * OSP_i)
    // CSP_i = Current Stock Price of Share i
    // CSN_i = Current Stock Number of Share i
    // OSP_i = Original Stock Price of Share i
    // OSN_i = Original Stock Number of Share i

    // Our current definition assumes that the position of a stock doesnt changes over time
    // ie. CSN_i == OSN_i = SN_i
    // then Profit = Sum( SN_i * (CSP_i - OSP_i))

    pub fn profit(&self, from: u32, to:u32) -> Option<f32> {
        if from > to {
            return None
        }

        let mut total: f32 = 0.0;
        // iterate over portfolio
        for (stock, data) in self.portfolio.iter() {
            let init_date = match from < data.get_purchase_date() {
                // If the stock is purchased after the from date, calculate profit since
                // the stock is in the portfolio
                false => data.get_purchase_date(),
                true => from,
            };

            let init_price = get_price(&stock, init_date);
            let final_price = get_price(&stock, to);

            total = total + ((final_price - init_price) * (data.get_quantity() as f32));

        }

        return Some(total)
    }

    // https://www.investopedia.com/terms/a/annualized-total-return.asp
    // From here we get that
    // Annualized Return = ((1 + Cumulative Return) ^ (365/days held)) - 1
    // https://www.investopedia.com/terms/c/cumulativereturn.asp
    // From here we get that
    // Cummulative Return = (Current Price - Original Price) / (Original Price)
    // or : CR = CP / OP - 1

    pub fn annualized_rate_of_return(&self, from: u32, to: u32) -> Option<f32> {
        if from > to {
            return None
        }

        let mut initial_value = 0.0;
        let mut final_value = 0.0;

        for (stock, data) in self.portfolio.iter() {
            let init_date = match from < data.get_purchase_date() {
                false => data.get_purchase_date(),
                true => from,
            };

            initial_value = initial_value + get_price(&stock, init_date) * (data.get_quantity() as f32);
            final_value = final_value + get_price(&stock, to) * (data.get_quantity() as f32);

        }

        let cum_rtn = final_value/initial_value - 1.0;
        let num_days = ((to - from) as f32) / 86400.0; //86400 seconds in a day

        return Some(f32::powf(1.0 + cum_rtn, 365.0/num_days) - 1.0)
    }

}
