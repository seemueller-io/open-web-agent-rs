import {describe, it} from 'vitest';
import {QuoteStore} from "./models";

describe('QuoteStore', () => {
    it('should get data for symbols using the quoteManager', async () => {
        const testApiKey = '';
        const quoteManager = QuoteStore.create({
            apiKey: testApiKey,
        });

        const symbol = 'BTC';

        const data = await quoteManager.fetchQuote(symbol);

        console.log(JSON.stringify(data));
    });
});