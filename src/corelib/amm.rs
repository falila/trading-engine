use crate::corelib::order::Wallet;
use std::collections::{HashMap, HashSet};

use super::token::{Pair, TokenTicker};

pub struct AMMPool {
    liquidity_pools: HashMap<TokenTicker, u64>,
    total_lp_per_pair: HashMap<Pair, u64>,
    account_lp_tokens: HashMap<Wallet, HashMap<Pair, u64>>,
}

impl AMMPool {
    pub fn new() -> AMMPool {
        AMMPool {
            liquidity_pools: HashMap::new(),
            account_lp_tokens: HashMap::new(),
            total_lp_per_pair: HashMap::new(),
        }
    }

    pub fn add_liquidity(&mut self, token: TokenTicker, amount: u64) {
        *self.liquidity_pools.entry(token).or_insert(0) += amount;
    }

    pub fn add_liquidity_pair(
        &mut self,
        wallet: Wallet,
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
            let total_liquidity_a = *self.liquidity_pools.get(&token_b).unwrap() as f64;
            let share_a = amount_a as f64 / total_liquidity_a as f64;

            let total_liquidity_b = *self.liquidity_pools.get(&token_b).unwrap() as f64;
            let share_b = (amount_b as f64 / total_liquidity_b as f64) as f64;

            // Mint and return LP tokens to the user based on the proportion of liquidity provided
            let lp_tokens_a = (share_a * total_liquidity_a) as u64;
            let lp_tokens_b = (share_b * total_liquidity_b) as u64;

            let pair = Pair {
                ticker_a: token_a,
                ticker_b: token_b,
            };
            let mut wallet_pairs = self
                .account_lp_tokens
                .entry(wallet)
                .or_insert_with(|| HashMap::new());
            for p in wallet_pairs.iter_mut() {
                if *p.0 == pair {
                    wallet_pairs
                        .entry(pair)
                        .and_modify(|qt| *qt += lp_tokens_a + lp_tokens_b);
                    break;
                } else {
                }
            }
            lp_tokens_a + lp_tokens_b
        } else {
            // Reject the operation if the ratio doesn't match within tolerance
            println!("Error: Actual ratio does not match the target ratio within the specified tolerance.");
            0 // Return 0 LP tokens
        }
    }

    pub fn token_swap(
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
        for (token, _) in self.liquidity_pools.iter() {
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
        let reserve_a = *self.liquidity_pools.get(&token_a)?;
        let reserve_b = *self.liquidity_pools.get(&token_b)?;

        // a constant product model (e.g., Uniswap) for AMM swaps
        let new_reserve_a = reserve_a + amount_in;
        let new_reserve_b = reserve_b + amount_in;

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
        let reserve_a = self.liquidity_pools.get_mut(&token_a)?;

        *reserve_a += amount_in;
        let reserve_b = self.liquidity_pools.get_mut(&token_b)?;
        *reserve_b -= amount_out;

        Some(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add_liquidity() {
        let mut amm = AMMPool::new();
        let token_a = TokenTicker::ETH;
        let token_b = TokenTicker::USDT;
        let amount_a = 1000;
        let amount_b = 2000;

        amm.add_liquidity(token_a.clone(), amount_a);
        amm.add_liquidity(token_b.clone(), amount_b);

        assert_eq!(amm.liquidity_pools.get(&token_a), Some(&1000));
        assert_eq!(amm.liquidity_pools.get(&token_b), Some(&2000));
    }

    #[test]
    fn test_add_liquidity_pair() {
        let mut amm = AMMPool::new();
        let wallet = Wallet::new(String::from("walletkeyxzr"));
        let token_a = TokenTicker::ETH;
        let amount_a = 1000;
        let token_b = TokenTicker::USDT;
        let amount_b = 2000;
        let target_ratio = 2.0;
        let tolerance = 0.1; // 10% tolerance

        let lp_tokens = amm.add_liquidity_pair(
            wallet.clone(),
            token_a.clone(),
            amount_a,
            token_b.clone(),
            amount_b,
            target_ratio,
            tolerance,
        );

        assert_eq!(lp_tokens, 3000); // Assuming LP tokens minted correctly
    }

    #[test]
    fn test_token_swap_insufficient_liquidity() {
        // Initialize liquidity pools
        let mut liquidity_pools = HashMap::new();
        liquidity_pools.insert(TokenTicker::ETH.clone(), 1000); // Lower liquidity
        liquidity_pools.insert(TokenTicker::USDT.clone(), 4000);

        let mut amm = AMMPool {
            liquidity_pools,
            total_lp_per_pair: HashMap::new(),
            account_lp_tokens: HashMap::new(),
        };

        let token_in = TokenTicker::ETH;
        let token_out = TokenTicker::USDT;
        let amount_in = 2000; // Higher amount than available liquidity

        let amount_out = amm.token_swap(token_in.clone(), token_out.clone(), amount_in);

        assert_eq!(amount_out, None); // Expecting None as liquidity is insufficient
    }

    #[test]
    fn test_token_swap_successful() {
        // Initialize liquidity pools
        let mut liquidity_pools = HashMap::new();
        liquidity_pools.insert(TokenTicker::ETH.clone(), 2000);
        liquidity_pools.insert(TokenTicker::USDT.clone(), 4000);

        let mut amm = AMMPool {
            liquidity_pools,
            total_lp_per_pair: HashMap::new(),
            account_lp_tokens: HashMap::new(),
        };

        let token_in = TokenTicker::ETH;
        let token_out = TokenTicker::USDT;
        let amount_in = 1000;

        let amount_out = amm.token_swap(token_in.clone(), token_out.clone(), amount_in);

        assert_eq!(amount_out, Some(2000)); // Assuming swap successful
    }

    #[test]
    fn test_token_swap_zero_amount() {
        // Initialize liquidity pools
        let mut liquidity_pools = HashMap::new();
        liquidity_pools.insert(TokenTicker::ETH.clone(), 2000);
        liquidity_pools.insert(TokenTicker::USDT.clone(), 4000);

        let mut amm = AMMPool {
            liquidity_pools,
            total_lp_per_pair: HashMap::new(),
            account_lp_tokens: HashMap::new(),
        };

        let token_in = TokenTicker::ETH;
        let token_out = TokenTicker::USDT;
        let amount_in = 0; // Zero input amount

        let amount_out = amm.token_swap(token_in.clone(), token_out.clone(), amount_in);

        assert_eq!(amount_out, Some(0)); // Expecting zero output amount for zero input amount
    }
}
