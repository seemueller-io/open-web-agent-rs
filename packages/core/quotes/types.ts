type Status = {
    timestamp: string;
    error_code: number;
    error_message: string | null;
    elapsed: number;
    credit_count: number;
    notice: string | null;
};

type Tag = {
    slug: string;
    name: string;
    category: string;
};

type Quote = {
    USD: {
        price: number | null;
        volume_24h: number;
        volume_change_24h: number;
        percent_change_1h: number;
        percent_change_24h: number;
        percent_change_7d: number;
        percent_change_30d: number;
        percent_change_60d: number;
        percent_change_90d: number;
        market_cap: number | null;
        market_cap_dominance: number | null;
        fully_diluted_market_cap: number | null;
        tvl: number | null;
        last_updated: string;
    };
};

type Platform = {
    id: number;
    name: string;
    symbol: string;
    slug: string;
    token_address: string;
};

type Cryptocurrency = {
    id: number;
    name: string;
    symbol: string;
    slug: string;
    num_market_pairs: number;
    date_added: string;
    tags: Tag[];
    max_supply: number | null;
    circulating_supply: number | null;
    total_supply: number;
    platform: Platform | null;
    is_active: number;
    infinite_supply: boolean;
    cmc_rank: number | null;
    is_fiat: number;
    self_reported_circulating_supply: number | null;
    self_reported_market_cap: number | null;
    tvl_ratio: number | null;
    last_updated: string;
    quote: Quote;
};

export type ApiResponse = {
    status: Status;
    data: {
        [SYMBOL: string]: Cryptocurrency[];
    };
};