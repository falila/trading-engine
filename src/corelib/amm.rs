use std::collections::{HashMap, HashSet};

use super::token::TokenTicker;

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

        *reserve_a += amount_in;
        let reserve_b = self.token_reserves.get_mut(&token_b)?;
        *reserve_b -= amount_out;

        Some(())
    }
}
