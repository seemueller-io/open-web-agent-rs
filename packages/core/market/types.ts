type Quote = {
    price: number;
    volume_24h: number;
    percent_change_1h: number;
    percent_change_24h: number;
    percent_change_7d: number;
    market_cap: number;
    last_updated: string;
};

type Platform = null;

type Tag = string;

type Data = {
    id: number;
    name: string;
    symbol: string;
    slug: string;
    cmc_rank?: number;
    num_market_pairs: number;
    circulating_supply: number;
    total_supply: number;
    max_supply: number;
    last_updated: string;
    date_added: string;
    tags: Tag[];
    platform: Platform;
    quote: {
        USD: Quote;
        BTC?: Quote;
        ETH?: Quote;
    };
};

type Status = {
    timestamp: string;
    error_code: number;
    error_message: string | null;
    elapsed: number;
    credit_count: number;
};

export type ApiResponse = {
    data: Data[];
    status: Status;
};

