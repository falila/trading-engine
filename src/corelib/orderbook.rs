use super::order::{BuyOrSell, Order};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

pub trait OrderBookTrait {
    fn best_buy_price(&self) -> Option<OrderedFloat<f64>>;
    fn best_sell_price(&self) -> Option<OrderedFloat<f64>>;
    fn sell_volume(&self) -> Option<u32>;
    fn buy_volume(&self) -> Option<u32>;
}

pub enum OrderStrategy {
    FIFO, // "First-In-First-Out"
    PTP,  //Price-Time Priority
}

pub struct OrderBook {
    pub buy_orders: HashMap<OrderedFloat<f64>, Vec<Order>>,
    pub sell_orders: HashMap<OrderedFloat<f64>, Vec<Order>>,
    pub orders_matching_strategy: OrderStrategy,
    next_order_id: u64,
}
impl OrderBookTrait for OrderBook {
    fn best_buy_price(&self) -> Option<OrderedFloat<f64>> {
        // Get the maximum price from the buy_orders HashMap
        self.buy_orders.keys().max().cloned()
    }

    fn best_sell_price(&self) -> Option<OrderedFloat<f64>> {
        self.sell_orders.keys().min().cloned()
    }

    fn sell_volume(&self) -> Option<u32> {
        let sell_volume = self
            .sell_orders
            .values()
            .flatten()
            .map(|order| order.quantity)
            .sum();
        Some(sell_volume)
    }

    fn buy_volume(&self) -> Option<u32> {
        let buy_volume = self
            .buy_orders
            .values()
            .flatten()
            .map(|order| order.quantity)
            .sum();
        Some(buy_volume)
    }
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new(),
            next_order_id: 1,
            orders_matching_strategy: OrderStrategy::PTP,
        }
    }

    pub fn add_order(&mut self, order_type: BuyOrSell, price: f64, quantity: u32, timestamp: u64) {
        let id: u64 = self.next_order_id;
        self.next_order_id += 1;

        let order = Order::new(id, quantity, price, timestamp);

        match order_type {
            BuyOrSell::Buy => match self.buy_orders.get_mut(&OrderedFloat(price)) {
                Some(orders) => {
                    orders.push(order);
                }
                None => {
                    self.buy_orders.insert(OrderedFloat(price), vec![order]);
                }
            },
            BuyOrSell::Sell => match self.sell_orders.get_mut(&OrderedFloat(price)) {
                Some(orders) => {
                    orders.push(order);
                }
                None => {
                    self.sell_orders.insert(OrderedFloat(price), vec![order]);
                }
            },
        }
    }
}
