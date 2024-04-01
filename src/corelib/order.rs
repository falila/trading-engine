#[derive(Debug, Clone)]
pub enum BuyOrSell {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub quantity: u32,
    pub price: f64,
    pub id: u64,
    pub timestamp: u64,
}

impl Order {
    pub fn new(id: u64, quantity: u32, price: f64, time: u64) -> Order {
        Order {
            quantity: quantity,
            price: price,
            id: id,
            timestamp: time,
        }
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.price != other.price {
            // higher price takes priority
            self.price.partial_cmp(&other.price).unwrap().reverse()
        } else if self.timestamp != other.timestamp {
            // earlier timestamp takes priority
            self.timestamp.cmp(&other.timestamp)
        } else {
            // orer by order id if prices and timestamp are the same
            self.id.cmp(&other.id)
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Order {}
