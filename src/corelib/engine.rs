use std::collections::{HashMap, HashSet};

use ordered_float::OrderedFloat;

use super::{order::Order, orderbook::OrderBook};

use num_traits::float::Float;
use num_traits::identities::Zero;

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
    BTC,
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
    pub amm_pools: HashMap<TokenTicker, AMMPool>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
            amm_pools: HashMap::new(),
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

    /// Matches buy and sell orders from all order books.
    ///
    /// This function iterates over all order books, compares buy and sell prices, and matches them based on price-time priority.
    /// It returns a vector of matched trades, each represented as a tuple containing the IDs of the buy and sell orders,
    /// the price at which the sell order was executed, and the quantity traded.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing matched trades, where each tuple contains:
    ///
    /// 1. The ID of the buy order.
    /// 2. The ID of the sell order.
    /// 3. The price at which the sell order was executed.
    /// 4. The quantity traded.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// # fn main() {
    /// #    let mut engine = MatchingEngine::new();
    /// #    // Add orders to the engine
    /// #    let _ = engine.match_orders();
    /// # }
    pub fn match_orders(&mut self) -> Vec<(u64, u64, f64, u32)> {
        let mut matched_trades = Vec::new();
        for (_, orderbook) in self.orderbooks.iter_mut() {
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

    pub fn add_liquidity(&mut self, token_ticker: TokenTicker, amount: u64) {
        if let Some(amm_pool) = self.amm_pools.get_mut(&token_ticker) {
            amm_pool.add_liquidity(token_ticker, amount);
        }
    }

    pub fn swap(
        &mut self,
        token_in: TokenTicker,
        token_out: TokenTicker,
        amount_in: u64,
    ) -> Option<u64> {
        if let Some(amm_pool) = self.amm_pools.get_mut(&token_in) {
            amm_pool.swap(token_out, amount_in)
        } else {
            None
        }
    }
}

pub struct AMMPool {
    token_reserves: HashMap<TokenTicker, u64>,
    base_reserve: HashMap<TokenTicker, u64>,
    lp_providers: HashSet<u64>,
}

impl AMMPool {
    pub fn new() -> AMMPool {
        AMMPool {
            lp_providers: HashSet::new(),
            token_reserves: HashMap::new(),
            base_reserve: HashMap::new(),
        }
    }

    pub fn add_liquidity(&mut self, token: TokenTicker, amount: u64) {
        *self.token_reserves.entry(token).or_insert(0) += amount;
        *self.base_reserve.entry(token.clone()).or_insert(0) += amount;
    }

    pub fn add_liquidity_pair(
        &mut self,
        token_a: TokenTicker,
        amount_a: u64,
        token_b: TokenTicker,
        amount_b: u64,
        target_ratio: f64,
        tolerance: f64,
    ) -> u64 {
        // Calculate the ratio of the amounts being added
        let actual_ratio = amount_a as f64 / amount_b as f64;

        // Check if the actual ratio matches the target ratio within the specified tolerance
        if (actual_ratio - target_ratio).abs() <= tolerance {
            // Add liquidity for both tokens
            self.add_liquidity(token_a.clone(), amount_a);
            self.add_liquidity(token_b.clone(), amount_b);

            // Calculate LP tokens to mint based on the shares of the new pair
            let total_liquidity = self.token_reserves.values().sum::<u64>() as f64;
            let share_a = amount_a as f64 / total_liquidity;
            let share_b = amount_b as f64 / total_liquidity;

            // Mint and return LP tokens to the user based on the proportion of liquidity provided
            let lp_tokens = (share_a * total_liquidity) as u64;
            lp_tokens
        } else {
            // Reject the operation if the ratio doesn't match within tolerance
            println!("Error: Actual ratio does not match the target ratio within the specified tolerance.");
            0 // Return 0 LP tokens
        }
    }

    pub fn swap(&mut self, token_out: TokenTicker, amount_in: u64) -> Option<u64> {
        // Let's assume a constant product model (e.g., Uniswap) for AMM swaps
        let new_token_reserve =
            *self.token_reserves.entry(token_out.clone()).or_insert(0) + amount_in;
        let new_base_reserve = *self.base_reserve.get(&token_out)?;

        // Calculate the output amount using the constant product formula
        let amount_out = match token_out {
            TokenTicker::BTC | TokenTicker::ETH => {
                let numerator = new_base_reserve * new_token_reserve;
                let denominator = (self.token_reserves[&token_out] as u128).sqrt() as u64;
                (numerator / denominator) as u64
            }
            TokenTicker::USDT => {
                // USDT is a stablecoin with a 1:1 peg to the base token (e.g., USD)
                // Therefore, output amount is equal to the input amount
                amount_in
            }
            TokenTicker::UNI => {
                // UNI is a governance token with a 0.3% fee on swaps
                // Therefore, output amount is calculated after deducting the fee
                let fee = (amount_in as f64 * 0.003) as u64;
                amount_in - fee
            }
            _ => {
                // For tokens with complex swap mechanics, additional logic would be needed
                // Return None for unsupported token types
                return None;
            }
        };

        // Update reserves
        self.token_reserves.insert(token_out, new_token_reserve);
        self.base_reserve
            .insert(token_out, new_base_reserve - amount_in);

        Some(amount_out)
    }

    /// Performs a multi-token swap between two tokens in the AMM pool.
    ///
    /// This function calculates the optimal path for the swap by finding the token pair with the highest output amount based on the constant product formula.
    /// It then iterates through the optimal path, swapping one token for another, and updates the reserves accordingly.
    ///
    /// # Arguments
    ///
    /// * `token_in` - The token to swap from.
    /// * `token_out` - The token to swap to.
    /// * `amount_in` - The amount of the input token to swap.
    ///
    /// # Returns
    ///
    /// The amount of the output token received after the swap, if successful. Returns `None` if the swap cannot be performed or the optimal path is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() {
    /// #    let mut pool = AMMPool::new();
    /// #    pool.add_liquidity(TokenTicker::ETH, 1000);
    /// #    pool.add_liquidity(TokenTicker::USDT, 5000);
    /// #    let _ = pool.multi_token_swap(TokenTicker::ETH, TokenTicker::USDT, 100);
    /// # }
    pub fn multi_token_swap(
        &mut self,
        token_in: TokenTicker,
        token_out: TokenTicker,
        amount_in: u64,
    ) -> Option<u64> {
        // Perform the multi-token swap
        // Find the path with the highest output amount for the given token pair
        let mut max_output_amount = 0;
        let mut optimal_path: Vec<TokenTicker> = Vec::new();

        // Iterate over all tokens in the pool
        for (token, _) in self.token_reserves.iter() {
            if token != &token_in && token != &token_out {
                // Calculate the output amount for the current path
                let output_amount =
                    self.calculate_output_amount(token_in.clone(), token.clone(), amount_in)?;

                // Update optimal path if output amount is higher
                if output_amount > max_output_amount {
                    max_output_amount = output_amount;
                    optimal_path = vec![token_in.clone(), token.clone(), token_out.clone()];
                }
            }
        }

        // Perform the swap using the optimal path
        let mut amount_in_remaining = amount_in;
        let mut current_token = token_in;
        for i in 0..optimal_path.len() - 1 {
            let token_a = optimal_path[i].clone();
            let token_b = optimal_path[i + 1].clone();

            let amount_out = self.calculate_output_amount(
                token_a.clone(),
                token_b.clone(),
                amount_in_remaining,
            )?;

            // Update reserves for token_a and token_b
            self.update_reserves(
                token_a.clone(),
                token_b.clone(),
                amount_in_remaining,
                amount_out,
            )?;

            // Update remaining input amount
            amount_in_remaining = amount_out;

            // Update current token for the next iteration
            current_token = token_b;
        }

        Some(amount_in_remaining)
    }

    fn calculate_output_amount(
        &self,
        token_a: TokenTicker,
        token_b: TokenTicker,
        amount_in: u64,
    ) -> Option<u64> {
        let reserve_a = *self.token_reserves.get(&token_a)?;
        let reserve_b = *self.token_reserves.get(&token_b)?;

        // a constant product model (e.g., Uniswap) for AMM swaps
        let new_reserve_a = reserve_a + amount_in;
        let new_reserve_b = *self.base_reserve.get(&token_b)?;

        let numerator = new_reserve_b * reserve_a;
        let denominator = new_reserve_a;

        Some((numerator / denominator) as u64)
    }

    // Update the reserves for swapping token_a for token_b
    fn update_reserves(
        &mut self,
        token_a: TokenTicker,
        token_b: TokenTicker,
        amount_in: u64,
        amount_out: u64,
    ) -> Option<()> {
        let reserve_a = self.token_reserves.get_mut(&token_a)?;
        let reserve_b = self.token_reserves.get_mut(&token_b)?;

        *reserve_a += amount_in;
        *reserve_b -= amount_out;

        Some(())
    }
}

#[cfg(test)]
mod test {

    use self::{Category, Market, MatchingEngine, Token, TokenTicker};
    use super::super::order::BuyOrSell;
    use super::super::orderbook::OrderBookTrait;
    use super::*;
    use chrono::Utc;

    #[test]
    #[ignore]

    fn test_token_listing() {
        // Test listing of tokens
        let mut engine_1 = MatchingEngine::new();
        let new_token = Token::new(
            TokenTicker::BTC,
            Category::Infatrusture,
            Market::OtherMarket(CryptoExchange::Binance),
        );
        engine_1.list_new_token(new_token.ticker.clone());
        assert_eq!(engine_1.orderbooks.len(), 1);
        match engine_1.get_token_orderbook(&new_token.ticker) {
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
                .get_token_orderbook(&new_token.ticker)
                .unwrap()
                .buy_volume()
                .unwrap(),
            685 + 690 + 690
        );
        assert_eq!(
            engine_1
                .get_token_orderbook(&new_token.ticker)
                .unwrap()
                .sell_volume()
                .unwrap(),
            2 * 700 + 705
        );
    }

    #[test]
    fn match_orders() {
        // Test maching of orders
        let mut engine = MatchingEngine::new();
        let new_token = Token::new(
            TokenTicker::DOT,
            Category::Infatrusture,
            Market::OtherMarket(CryptoExchange::Coinbase),
        );
        engine.list_new_token(new_token.ticker.clone());
        assert_eq!(engine.orderbooks.len(), 1);
        match engine.get_token_orderbook(&new_token.ticker) {
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
                .get_token_orderbook(&new_token.ticker)
                .unwrap()
                .buy_volume()
                .unwrap(),
            20
        );
        assert_eq!(
            engine
                .get_token_orderbook(&new_token.ticker)
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

        // Add liquidity pair with matching ratio
        let lp_tokens =
            pool.add_liquidity_pair(TokenTicker::ETH, 1000, TokenTicker::USDT, 5000, 2.0, 0.1);
        assert_eq!(lp_tokens, 10); // Assuming total liquidity is 10000 and each token contributes equally

        // Add liquidity pair with mismatched ratio (should fail)
        let lp_tokens_fail =
            pool.add_liquidity_pair(TokenTicker::ETH, 1000, TokenTicker::USDT, 4000, 2.0, 0.1);
        assert_eq!(lp_tokens_fail, 0); // Should return 0 LP tokens due to ratio mismatch
    }

    #[test]
    fn test_swap() {
        let mut pool = AMMPool::new();
        pool.add_liquidity_pair(TokenTicker::ETH, 1000, TokenTicker::USDT, 5000, 2.0, 0.1);

        // Swap ETH for USDT
        let amount_out = pool.multi_token_swap(TokenTicker::ETH, TokenTicker::USDT, 100);
        assert_eq!(amount_out, Some(200)); // Assuming a constant product model with 1:2 ratio

        // Swap USDT for ETH
        let amount_out = pool.multi_token_swap(TokenTicker::USDT, TokenTicker::ETH, 1000);
        assert_eq!(amount_out, Some(50)); // Assuming a constant product model with 1:2 ratio
    }
}
