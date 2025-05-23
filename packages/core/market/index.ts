import {ApiResponse} from "./types";

export async function collect_gainers_losers(x: { apiKey: string, limit: number }): Promise<ApiResponse> {
    const { apiKey, limit } = x;

    //
    const data: ApiResponse = await fetch(`https://pro-api.coinmarketcap.com/v1/cryptocurrency/trending/gainers-losers?limit=${limit}`, {
        headers: {
            "x-cmc_pro_api_key": apiKey
        }
    }).then((symbolDataRequest) => symbolDataRequest.json());

    return data;
}