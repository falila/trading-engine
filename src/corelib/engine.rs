use std::collections::HashMap;

use super::orderbook::OrderBook;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Market {
    AfricaMarket(AfricaExchange),
    OtherMarket(CryptoExchange),
}
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum AfricaExchange {
    NajaEx,
    MorrockEx,
    WariEx,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum CryptoExchange {
    UpBit,
    KuCoin,
    OKx,
    ByBit,
    CoinDCX,
    Binance,
    Coinbase,
    Kraken,
}
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Category {
    AI,
    Defi,
    Memes,
    Infatrusture,
    DAO,
    Gaming,
    Metaverse,
    Social,
    Oracle,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum TokenTicker {
    BTC(String),
    ETH,
    USDT,
    SOL,
    BNB,
    XRP,
    USDC,
    Doge,
    ADA,
    AVA,
    DOT,
    BCH,
    LINK,
    TRON,
    ICP,
    LTC,
    UNI,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Token {
    ticker: TokenTicker,
    category: Category,
    market: Market,
}

impl Token {
    pub fn new(ticker: TokenTicker, category: Category, market: Market) -> Token {
        Token {
            ticker,
            category,
            market,
        }
    }
}

pub struct MatchingEngine {
    pub orderbooks: HashMap<TokenTicker, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }
    pub fn list_new_token(&mut self, token_ticker: TokenTicker) {
        self.orderbooks
            .entry(token_ticker)
            .or_insert(OrderBook::new());
    }

    pub fn get_token_orderbook(&mut self, token_ticker: &TokenTicker) -> Option<&mut OrderBook> {
        self.orderbooks.get_mut(token_ticker)
    }
}

#[cfg(test)]
mod test {

    use chrono::Utc;
    use self::{Category, Market, MatchingEngine, Token, TokenTicker};
    use super::super::order::BuyOrSell;
    use super::super::orderbook::OrderBookTrait;
    use super::*;

    #[test]
    fn test_token_listing() {
        let mut engine = MatchingEngine::new();
        let new_token = Token::new(
            TokenTicker::BTC(String::from("Bitcoin")),
            Category::Infatrusture,
            Market::OtherMarket(CryptoExchange::Binance),
        );
        engine.list_new_token(new_token.ticker.clone());
        assert_eq!(engine.orderbooks.len(), 1);
        match engine.get_token_orderbook(&new_token.ticker) {
            Some(order_book) => {
                // create buy orders
                order_book.add_order(
                    BuyOrSell::Buy,
                    35.0,
                    690,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Buy,
                    20.0,
                    685,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Buy,
                    15.0,
                    690,
                    Utc::now().timestamp().try_into().unwrap(),
                );

                order_book.add_order(
                    BuyOrSell::Sell,
                    10.0,
                    700,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Sell,
                    25.0,
                    705,
                    Utc::now().timestamp().try_into().unwrap(),
                );
                order_book.add_order(
                    BuyOrSell::Sell,
                    30.0,
                    700,
                    Utc::now().timestamp().try_into().unwrap(),
                );
            }
            None => panic!("Ticker not found"),
        };
        assert_eq!(
            engine
                .get_token_orderbook(&new_token.ticker)
                .unwrap()
                .buy_volume()
                .unwrap(),
            685+690+690
        );
        assert_eq!(
            engine
                .get_token_orderbook(&new_token.ticker)
                .unwrap()
                .sell_volume()
                .unwrap(),
            2*700+705
        );
    }
}
