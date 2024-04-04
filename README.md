# AMM (Automated Market Maker) and Order Book Trading Engine

A Simple AMM and Order Book Trading Engine in Rust. It provides functionalities for liquidity provision, token swaps, and order matching for trading various tokens.

## Features

### Automated Market Maker (AMM)

The AMM feature allows users to provide liquidity to the pool and swap tokens based on a constant product model, such as Uniswap. It supports the following functionalities:

- Add liquidity for pairs of tokens
- Swap one token for another
- Multi-token swaps for more complex trading scenarios

### Order Book

The Order Book feature provides a traditional trading mechanism based on buy and sell orders. It supports the following functionalities:

- Add buy and sell orders to the order book
- Match buy and sell orders based on price-time priority

## Usage

### Automated Market Maker (AMM)

To use the AMM feature:

1. Create a new AMM pool.
2. Add liquidity for pairs of tokens using the `add_liquidity_pair` function.
3. Swap tokens using the `multi_token_swap` function.

### Order Book

To use the Order Book feature:

1. Create a new Order Book.
2. Add buy and sell orders using the `add_order` function.
3. Match buy and sell orders using the `match_orders` function.
