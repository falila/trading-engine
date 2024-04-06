#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Market {
    AfricaMarket(AfricaExchange),
    OtherMarket(CryptoExchange),
    USMarket(USExchange),
}
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum AfricaExchange {
    NajaEx,
    MorrockEx,
    WariEx,
    GCoin,
    XMGCoin,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum CryptoExchange {
    UpBit,
    KuCoin,
    OKx,
    ByBit,
    CoinDCX,
    Binance,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum USExchange {
    BinanceUS,
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
    XUSD,
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
    FIL,
    ROOT,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Pair {
    pub ticker_a: TokenTicker,
    pub ticker_b: TokenTicker,
}

impl Pair {
    pub fn new(ticker_a: TokenTicker, ticker_b: TokenTicker) -> Pair {
        Pair { ticker_a, ticker_b }
    }
}
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Token {
    pub ticker: TokenTicker,
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
