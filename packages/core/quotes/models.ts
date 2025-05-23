import {types, flow, Instance} from "mobx-state-tree";
import {collect_quote} from './index';


const QuoteData = types.optional(types.frozen(), {})

export const QuoteStore = types
    .model("QuoteStore", {
        apiKey: types.string,
        quotes: types.map(QuoteData),
    })
    .views(self => ({
        getQuote(symbol) {
            return self.quotes.get(symbol);
        },

        hasQuote(symbol) {
            return self.quotes.has(symbol);
        }
    }))
    .actions(self => {

        const extractUsefulData = (data, symbol) => {
            return data.data[symbol].map(qd => ({
                symbol: qd.symbol,
                slug: qd.slug,
                tags: qd.tags,
                id: qd.id,
                ...qd.quote.USD
            })).at(0);
        };

        const fetchQuote = flow(function* (symbol) {
            try {
                const data = yield collect_quote({symbol, apiKey: self.apiKey});
                const usefulData = extractUsefulData(data, symbol);


                self.quotes.set(symbol, usefulData);

                return usefulData;
            } catch (error) {
                console.error(`An error occurred fetching the quote for symbol: ${symbol}`, error);
                throw error;
            }
        });

        const fetchQuotes = flow(function* (symbols) {
            const results = {};

            for (const symbol of symbols) {

                if (self.quotes.has(symbol)) {
                    results[symbol] = self.quotes.get(symbol);
                } else {

                    const data = yield fetchQuote(symbol);
                    results[symbol] = extractUsefulData(data, symbol);
                }
            }

            return results;
        });

        const clearCache = () => {
            self.quotes.clear();
        };

        return {
            fetchQuote,
            fetchQuotes,
            clearCache
        };
    });


export type QuoteManagerType = Instance<typeof QuoteStore>;