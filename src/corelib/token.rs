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
