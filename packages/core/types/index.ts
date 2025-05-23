export type CryptoDataResponse = {
    status: {
        timestamp: string;
        error_code: number;
        error_message: string | null;
        elapsed: number;
        credit_count: number;
        notice: string | null;
    };
    data: {
        [key: string]: CryptoAsset[];
    };
};

export type CryptoAsset = {
    id: number;
    name: string;
    symbol: string;
    slug: string;
    num_market_pairs: number;
    date_added: string;
    tags: CryptoTag[];
    max_supply: number | null;
    circulating_supply: number;
    total_supply: number;
    platform: CryptoPlatform | null;
    is_active: number;
    infinite_supply: boolean;
    cmc_rank: number;
    is_fiat: number;
    self_reported_circulating_supply: number | null;
    self_reported_market_cap: number | null;
    tvl_ratio: number;
    last_updated: string;
    quote: {
        [currency: string]: CryptoQuote;
    };
};

export type CryptoTag = {
    slug: string;
    name: string;
    category: string;
};

export type CryptoPlatform = {
    id: number;
    name: string;
    symbol: string;
    slug: string;
    token_address: string;
};

export type CryptoQuote = {
    price: number;
    volume_24h: number;
    volume_change_24h: number;
    percent_change_1h: number;
    percent_change_24h: number;
    percent_change_7d: number;
    percent_change_30d: number;
    percent_change_60d: number;
    percent_change_90d: number;
    market_cap: number;
    market_cap_dominance: number;
    fully_diluted_market_cap: number;
    tvl: number;
    last_updated: string;
};