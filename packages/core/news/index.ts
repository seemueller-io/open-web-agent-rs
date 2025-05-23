import {types, Instance} from 'mobx-state-tree';
import {runInAction} from "mobx";


const Article = types.model('Article', {
    title: types.string,
    content: types.string,
    url: types.maybe(types.string),
    source: types.maybe(types.string),
    pubDate: types.maybe(types.string),

    summary: types.maybe(types.string),
    description: types.maybe(types.string),
    authorsByline: types.maybe(types.string),
    shortSummary: types.maybe(types.string),
    labels: types.maybe(types.frozen()),
    imageUrl: types.maybe(types.string),
    score: types.maybe(types.number),
});


export const NewsStore = types
    .model('NewsStore', {
        symbolsNews: types.map(types.array(Article)),
        isLoading: types.boolean,
        error: types.maybe(types.string),
        apiKey: types.string,
    })
    .actions((self) => ({

        addNews(symbol: string, articles: any[]) {
            if (!self.symbolsNews.has(symbol)) {
                self.symbolsNews.set(symbol, []);
            }


            const mappedArticles = articles.map((article) => Article.create({
                title: article.title || 'No Title',
                content: article.content || 'No Content',
                url: article.url,
                source: article.domain,
                pubDate: article.pubDate,
                summary: article.summary,
                description: article.description,
                authorsByline: article.authorsByline,
                shortSummary: article.shortSummary,
                labels: article.labels,
                imageUrl: article.imageUrl,
                score: article.score,


            }));
            self.symbolsNews.get(symbol)!.push(...mappedArticles);
            self.isLoading = false;
        },

        clearNews(symbol: string) {
            if (self.symbolsNews.has(symbol)) {
                self.symbolsNews.set(symbol, []);
            }
        },

        setLoading(loading: boolean) {
            self.isLoading = loading;
        },

        setError(message: string) {
            self.error = message;
            self.isLoading = false;
        },

        async fetchNewsForSymbol(symbol: string, limit: number, sort: "date" | "relevance") {
            self.setLoading(true);
            self.setError(undefined);

            try {
                await runInAction(async () => {
                    const newsData = await collect_news({symbol, apiKey: self.apiKey, limit, sort});
                    if (newsData && newsData.articles) {
                        self.addNews(symbol, newsData.articles);
                    } else {
                        self.setError("Failed to fetch news or invalid response format.");
                    }
                })


            } catch (err: any) {
                console.error('Error fetching news:', err);
                self.setError(err.message || "Failed to fetch news.");
            }
        },
    }))
    .views((self) => ({

        getNewsForSymbol(symbol: string) {

            return self.symbolsNews.get(symbol) || [];
        },

        getAllSymbols() {
            return Array.from(self.symbolsNews.keys());
        },

        hasNewsForSymbol(symbol: string) {
            return self.symbolsNews.has(symbol) && self.symbolsNews.get(symbol)!.length > 0;
        },
    }));

export type INewsStore = Instance<typeof NewsStore>;


export const createNewsStore = (apikey, perigon) => NewsStore.create({
    symbolsNews: {},
    isLoading: false,
    error: undefined,
    apiKey: apikey,
});

/* @collect_news return value structure
{
news: {
status: 200,
numResults: 4080,
articles: [
  [Object], [Object],
  [Object], [Object],
  [Object], [Object],
  [Object], [Object],
  [Object], [Object]
]
}
}
 */
export async function collect_news(x: { symbol: string, apiKey: string, limit: number, sort: "date" | "relevance" }) {


    const {symbol, apiKey, limit, sort} = x;

    const symbolNameMap = {
        "BTC": "Bitcoin",
        "ETH": "Ethereum",
        "XRP": "Ripple",
        "LTC": "Litecoin",
        "ADA": "Cardano",
        "DOGE": "Dogecoin",
        "BNB": "Binance Coin",
        "DOT": "Polkadot",
        "SOL": "Solana",
        "AVAX": "Avalanche"
    };


    const cryptocurrencyName = symbolNameMap[symbol] ?? symbol;


    const rawContentQuery = "scandal OR \"corporate misconduct*\" OR fraud OR \"financial irregularities*\" OR lawsuit OR \"legal action*\" OR bankruptcy OR \"financial distress*\" OR \"data breach\" OR \"security vulnerability*\" OR \"environmental impact\" OR \"ecological damage*\" OR \"labor dispute\" OR \"worker rights*\" OR \"product failure\" OR \"quality issue*\" OR \"ethical concern\" OR \"moral dilemma*\" OR \"health risk\" OR \"safety hazard*\" OR \"regulatory violation\" OR \"compliance issue*\" OR \"market manipulation\" OR \"trading irregularity*\" OR \"public relations crisis\" OR \"reputation damage*\" OR \"political controversy\" OR \"government intervention*\" OR \"consumer complaint\" OR \"customer dissatisfaction*\" OR \"supply chain disruption\" OR \"logistics problem*\" OR \"intellectual property dispute\" OR \"patent infringement*\"";
    const contentQuery = encodeURIComponent(rawContentQuery);


    const rawTitleQuery = `${cryptocurrencyName} OR ${symbol} OR "${cryptocurrencyName} price" OR "${cryptocurrencyName} market" OR "${cryptocurrencyName} news"`;
    const titleQuery = encodeURIComponent(rawTitleQuery);

    try {
        const result = await allNews({
            q: contentQuery,
            title: titleQuery,
            size: limit,
            sortBy: sort,
            apiKey: apiKey
        });


        return result.data;
    } catch (err) {
        console.error('Error fetching news:', err);
        throw err;
    }
}
