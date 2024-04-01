pub mod corelib;

#[cfg(test)]
mod test {

    use chrono::Utc;

    //use crate::corelib::order;

    use super::*;
    use corelib::{
        order::BuyOrSell,
        orderbook::{OrderBook, OrderBookTrait},
    };
    use ordered_float::OrderedFloat;

    #[test]
    fn test_add_order() {
        let mut order_book = OrderBook::new();
        //create sell orders
        order_book.add_order(
            BuyOrSell::Sell,
            99.9,
            33,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Sell,
            99.9,
            100,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Sell,
            20.0,
            10,
            Utc::now().timestamp().try_into().unwrap(),
        );

        // create buy orders

        order_book.add_order(
            BuyOrSell::Buy,
            37.0,
            66,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Buy,
            30.0,
            87,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Buy,
            50.0,
            90,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Buy,
            50.0,
            94,
            Utc::now().timestamp().try_into().unwrap(),
        );

        assert_eq!(order_book.sell_orders.len(), 2);
        assert_eq!(order_book.buy_orders.len(), 3);

        assert_eq!(
            order_book
                .sell_orders
                .get(&OrderedFloat(99.9))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            order_book
                .sell_orders
                .get(&OrderedFloat(20.0))
                .unwrap()
                .len(),
            1
        );

        assert_eq!(
            order_book
                .buy_orders
                .get(&OrderedFloat(37.0))
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            order_book
                .buy_orders
                .get(&OrderedFloat(30.0))
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            order_book
                .buy_orders
                .get(&OrderedFloat(50.0))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn test_prices_and_volumes() {
        // Initialze the new order_book
        let mut order_book = OrderBook::new();

        // Create some buy orders.
        order_book.add_order(
            BuyOrSell::Buy,
            300.0,
            641,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Buy,
            370.0,
            87,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Buy,
            500.0,
            900,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Buy,
            27.0,
            784,
            Utc::now().timestamp().try_into().unwrap(),
        );

        // Create some sell orders.
        order_book.add_order(
            BuyOrSell::Sell,
            200.0,
            200,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Sell,
            99.0,
            100,
            Utc::now().timestamp().try_into().unwrap(),
        );
        order_book.add_order(
            BuyOrSell::Sell,
            20.0,
            10,
            Utc::now().timestamp().try_into().unwrap(),
        );

        assert_eq!(order_book.best_buy_price().unwrap(), OrderedFloat(500.0));
        assert_eq!(order_book.best_sell_price().unwrap(), OrderedFloat(20.0));

        assert_eq!(order_book.buy_volume().unwrap(), 641 + 87 + 900 + 784);
        assert_eq!(order_book.sell_volume().unwrap(), 200 + 100 + 10);
    }
}
