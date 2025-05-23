import {ApiResponse} from "./types";

export async function collect_quote(x: { symbol: string, apiKey: string }) {
    const {symbol, apiKey} = x;

    const data: ApiResponse = await fetch(`https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest?symbol=${symbol}`, {
        headers: {
            "x-cmc_pro_api_key": apiKey
        }
    }).then((symbolDataRequest) => symbolDataRequest.json());

    return data;
}