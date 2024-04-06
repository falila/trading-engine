use std::collections::HashMap;

use ordered_float::OrderedFloat;

use super::amm::AMMPool;
use super::token::{Pair, TokenTicker};
use super::{order::Order, orderbook::OrderBook};

pub struct TradeEngine {
    pub order_books: HashMap<TokenTicker, OrderBook>,
    pub amm_pools: HashMap<Pair, AMMPool>,
}

pub trait Amm {
    fn token_swap(
        &mut self,
        token_in: TokenTicker,
        token_out: TokenTicker,
        amount_in: u64,
    ) -> Option<u64>;

    fn add_liquidity_pair(
        &mut self,
        token_a: TokenTicker,
        amount_a: u64,
        token_b: TokenTicker,
        amount_b: u64,
        target_ratio: f64,
        tolerance: f64,
    ) -> u64;
}

impl Amm for TradeEngine {
    fn token_swap(
        &mut self,
        token_in: TokenTicker,
        token_out: TokenTicker,
        amount_in: u64,
    ) -> Option<u64> {
        todo!()
    }

    fn add_liquidity_pair(
        &mut self,
        token_a: TokenTicker,
        amount_a: u64,
        token_b: TokenTicker,
        amount_b: u64,
        target_ratio: f64,
        tolerance: f64,
    ) -> u64 {
        todo!()
    }
}

impl TradeEngine {
    pub fn new() -> TradeEngine {
        TradeEngine {
            order_books: HashMap::new(),
            amm_pools: HashMap::new(),
        }
    }
    pub fn list_new_token(&mut self, token_ticker: TokenTicker) {
        self.order_books
            .entry(token_ticker)
            .or_insert(OrderBook::new());
    }

    pub fn get_token_order_book(&mut self, token_ticker: &TokenTicker) -> Option<&mut OrderBook> {
        self.order_books.get_mut(token_ticker)
    }

    pub fn match_orders(&mut self) -> Vec<(u64, u64, f64, u32)> {
        let mut matched_trades = Vec::new();
        for (_, orderbook) in self.order_books.iter_mut() {
            let buy_prices: Vec<OrderedFloat<f64>> = orderbook.buy_orders.keys().copied().collect();
            let sell_prices: Vec<OrderedFloat<f64>> =
                orderbook.sell_orders.keys().copied().collect();

            let mut buy_iter = buy_prices.iter().copied();
            let mut sell_iter = sell_prices.iter().copied();

            while let (Some(buy_price), Some(sell_price)) = (buy_iter.next(), sell_iter.next()) {
                if buy_price >= sell_price {
                    let buy_orders = orderbook.buy_orders.entry(buy_price).or_insert(Vec::new());
                    let sell_orders = orderbook
                        .sell_orders
                        .entry(sell_price)
                        .or_insert(Vec::new());

                    let buy_order = buy_orders.pop().unwrap();
                    let sell_order = sell_orders.pop().unwrap();

                    let quantity_traded = buy_order.quantity.min(sell_order.quantity);

                    matched_trades.push((
                        buy_order.id,
                        sell_order.id,
                        sell_order.price,
                        quantity_traded,
                    ));

                    if buy_order.quantity > quantity_traded {
                        buy_orders.push(Order {
                            quantity: buy_order.quantity - quantity_traded,
                            ..buy_order
                        });
                    }

                    if sell_order.quantity > quantity_traded {
                        sell_orders.push(Order {
                            quantity: sell_order.quantity - quantity_traded,
                            ..sell_order
                        });
                    }
                } else {
                    break;
                }
            }
        }

        matched_trades
    }
}

#[cfg(test)]
mod test {

    use crate::corelib::token::{Category, CryptoExchange, Market, Token};

    use self::{TokenTicker, TradeEngine};
    use super::super::order::BuyOrSell;
    use super::super::orderbook::OrderBookTrait;
    use super::*;
    use crate::corelib::order::Wallet;
    use chrono::Utc;

    #[test]
    #[ignore]

    fn test_token_listing() {
        // Test listing of tokens
        let mut engine_1 = TradeEngine::new();
        let new_token = Token::new(
            TokenTicker::BTC,
            Category::Infatrusture,
            Market::OtherMarket(CryptoExchange::Binance),
        );
        engine_1.list_new_token(new_token.ticker.clone());
        assert_eq!(engine_1.order_books.len(), 1);
        match engine_1.get_token_order_book(&new_token.ticker) {
            Some(order_book) => {
                // create buy orders
                order_book.add_order(
                    BuyOrSell::Buy,
                    31.0,
                    690,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Buy,
                    21.0,
                    685,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Buy,
                    21.0,
                    690,
                    Utc::now().timestamp().try_into().unwrap(),
                );

                order_book.add_order(
                    BuyOrSell::Sell,
                    20.0,
                    700,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Sell,
                    10.0,
                    705,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Sell,
                    43.0,
                    700,
                    Utc::now().timestamp().try_into().unwrap(),
                );
            }
            None => panic!("Ticker not found"),
        };
        assert_eq!(
            engine_1
                .get_token_order_book(&new_token.ticker)
                .unwrap()
                .buy_volume()
                .unwrap(),
            685 + 690 + 690
        );
        assert_eq!(
            engine_1
                .get_token_order_book(&new_token.ticker)
                .unwrap()
                .sell_volume()
                .unwrap(),
            2 * 700 + 705
        );
    }

    #[test]
    fn match_orders() {
        // Test maching of orders
        let mut engine = TradeEngine::new();
        let new_token = Token::new(
            TokenTicker::DOT,
            Category::Infatrusture,
            Market::OtherMarket(CryptoExchange::Coinbase),
        );
        engine.list_new_token(new_token.ticker.clone());
        assert_eq!(engine.order_books.len(), 1);
        match engine.get_token_order_book(&new_token.ticker) {
            Some(order_book) => {
                // create buy orders
                order_book.add_order(
                    BuyOrSell::Buy,
                    30.0,
                    5,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Buy,
                    41.0,
                    5,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Buy,
                    10.0,
                    10,
                    Utc::now().timestamp().try_into().unwrap(),
                );

                order_book.add_order(
                    BuyOrSell::Sell,
                    40.0,
                    10,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Sell,
                    40.0,
                    5,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Sell,
                    40.0,
                    5,
                    Utc::now().timestamp().try_into().unwrap(),
                );
            }
            None => panic!("Ticker not found"),
        };

        assert_eq!(
            engine
                .get_token_order_book(&new_token.ticker)
                .unwrap()
                .buy_volume()
                .unwrap(),
            20
        );
        assert_eq!(
            engine
                .get_token_order_book(&new_token.ticker)
                .unwrap()
                .sell_volume()
                .unwrap(),
            20
        );
        let orders_traded = engine.match_orders();
        println!("{:?}", orders_traded);
        assert_eq!(orders_traded.len(), 1);
    }

    #[test]
    fn test_add_liquidity_pair() {
        let mut pool = AMMPool::new();
        let wallet = Wallet::new(String::from("testskskdk"));

        // Add liquidity pair with matching ratio
        let lp_tokens = pool.add_liquidity_pair(
            wallet.clone(),
            TokenTicker::ETH,
            1000,
            TokenTicker::USDT,
            5000,
            2.0,
            0.1,
        );
        assert_eq!(lp_tokens, 10); // Assuming total liquidity is 10000 and each token contributes equally

        // Add liquidity pair with mismatched ratio (should fail)
        let lp_tokens_fail = pool.add_liquidity_pair(
            wallet.clone(),
            TokenTicker::ETH,
            1000,
            TokenTicker::USDT,
            4000,
            2.0,
            0.1,
        );
        assert_eq!(lp_tokens_fail, 0); // Should return 0 LP tokens due to ratio mismatch
    }

    #[test]
    fn test_swap() {
        let mut pool = AMMPool::new();
        let wallet = Wallet::new(String::from("walletkeyxz"));
        pool.add_liquidity_pair(
            wallet.clone(),
            TokenTicker::ETH,
            1000,
            TokenTicker::USDT,
            5000,
            2.0,
            0.1,
        );

        // Swap ETH for USDT
        let amount_out = pool.token_swap(TokenTicker::ETH, TokenTicker::USDT, 100);
        assert_eq!(amount_out, Some(200)); // Assuming a constant product model with 1:2 ratio

        // Swap USDT for ETH
        let amount_out = pool.token_swap(TokenTicker::USDT, TokenTicker::ETH, 1000);
        assert_eq!(amount_out, Some(50)); // Assuming a constant product model with 1:2 ratio
    }
}
