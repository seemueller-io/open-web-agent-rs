import state from "./_state/index.js";
import {getSnapshot} from "mobx-state-tree";
import {collect_gainers_losers} from "@web-agent-rs/core/market";

def("QUERY", env.vars.user_input);


defTool(
    "get_quote",
    "Fetch quote for symbol",
    {
        "symbol": {
            type: "string",
            default: "BTC"
        }
    },
    async (args) => {
        const { symbol } = args;
        await state.quotes.fetchQuote(symbol);
        

        const quote = await state.quotes.getQuote(symbol);
        
        return JSON.stringify(quote)
    }
);

defTool(
    "get_news",
    "Fetches news for symbol",
    {
        "symbol": {
            type: "string",
            default: "BTC"
        }
    },
    async (args) => {
        const { symbol } = args;
        await state.news.fetchNewsForSymbol(symbol, 5, "date");
        
        const news = await state.news.getNewsForSymbol(symbol).map(i => getSnapshot(i));
        
        return news
    }
);


defTool(
    "get_market",
    "Fetches trending symbols of market",
    {
        "limit": {
            type: "number",
            default: "25"
        }
    },
    async (args) => {

        const { limit } = args;

        
        const marketOverviewRequest = await collect_gainers_losers({apiKey: process.env.CCC_API_KEY, limit: parseInt(limit) })
        
        return marketOverviewRequest.data.map(item => ({
            symbol: item.symbol,
            name: item.name,
            
            
            change_1h: item.quote.USD.percent_change_1h,
            price: item.quote.USD.price,
            volume_24h: item.quote.USD.volume_24h
        }))
    }
);



$`You are a market data assistant specializing in financial analysis. Respond to QUERIES with accurate, clear, and concise information relevant to professionals in the finance sector. Use available tools efficiently to gather and present quantitative data.`;


