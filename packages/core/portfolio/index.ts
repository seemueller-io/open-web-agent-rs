import {getSnapshot, types} from "mobx-state-tree";
import {NewsStore} from "../news";


const PortfolioCashModel = types.model("PortfolioCash", {
    amount: types.number,
    currency: types.enumeration("Currency", ["USD", "BTC"]),
});


const PortfolioActionModel = types.model("PortfolioAction", {
    action: types.enumeration("Action", ["buy", "sell", "hold"]),
    symbol: types.string,
    quantity: types.number,
    timestamp: types.Date,
});


export const PortfolioNewsportfolioNewsModel = types.model("PortfolioNews", {
    symbol: types.string,
    date_created: types.string,
    news: types.array(types.model("NewsItem", {
        symbol: types.maybe(types.string),
        date_created: types.maybe(types.string),
        news: types.string,
        timestamp: types.maybe(types.string),
    })),
    timestamp: types.maybe(types.string),
});

export const portfolioQuoteModel = types.model("PortfolioQuote", {
    symbol: types.string,
    quote: types.string,
    date_created: types.string,
});


const PortfolioAssetContextModel = types.model("PortfolioAssetContext", {
    timestamp: types.Date,
    portfolio_snapshot: PortfolioCashModel,
});


const PortfolioAssetModel = types.model("PortfolioAsset", {
    symbol: types.string,
    quantity: types.number,
    recommended_action: types.maybe(
        types.enumeration("RecommendedAction", ["buy", "sell", "hold"])
    ),
    last_taken_action: types.optional(types.enumeration("LastAction", ["buy", "sell", "hold", "none", "never"]), "never"),
    context: PortfolioAssetContextModel,
});


const PortfolioModel = types
    .model("Portfolio", {
        supportedSymbols: types.array(types.string),
        liquidity: PortfolioCashModel,
        actions: types.optional(types.array(PortfolioActionModel), []),
        assets: types.array(PortfolioAssetModel),
        news: types.optional(types.array(NewsStore), []),
        quotes: types.optional(types.array(portfolioQuoteModel), []),
    })
    .actions((self) => ({

        addAction(actionData: {
            action: "buy" | "sell" | "hold";
            symbol: string;
            quantity: number;
        }) {
            self.actions.push({
                ...actionData,
                timestamp: new Date(),
            });
        },
        addNews(newsData: any) {
            self.news.push({
                ...newsData,
                timestamp: new Date(),
            });
        },
        addQuote(quoteData: any) {
            self.quotes
            self.quotes.push({
                ...quoteData,
                timestamp: new Date(),
            });
        },

        updateLiquidity(amount: number) {
            self.liquidity.amount = amount;
        },
    }));

const tokenList = [
    "AAVE", "AVAX", "BAT", "BCH", "BTC",
    "CRV", "DOGE", "DOT", "ETH", "GRT",
    "LINK", "LTC", "MKR", "SHIB", "SUSHI",
    "UNI", "USDC", "USDT", "XTZ", "YFI",
];


const portfolioCash = PortfolioCashModel.create({
    amount: 10000,
    currency: "USD",
});


const portfolioAssets = tokenList.map((token) =>
    PortfolioAssetModel.create({
        symbol: token,
        quantity: 0,
        recommended_action: "hold",
        last_taken_action: undefined,
        context: {
            timestamp: new Date(),
            portfolio_snapshot: getSnapshot(portfolioCash),
        },
    })
);


const portfolioActions = [];
const portfolioNews = [];
const portfolioQuotes = [];


const portfolioInstance = PortfolioModel.create({
    liquidity: portfolioCash,
    actions: portfolioActions,
    assets: portfolioAssets,
    supportedSymbols: tokenList,
    news: portfolioNews,
    quotes: portfolioQuotes
});

export default portfolioInstance;
