import {describe, expect, it} from 'vitest';
import {collect_news, createNewsStore, NewsStore} from './index';


const testApiKey = '';

describe('NewsStore', () => {
    it('should create a NewsStore instance', () => {
        const store = createNewsStore(testApiKey);
        expect(store).toBeDefined();
        expect(store.isLoading).toBe(false);
        expect(store.error).toBeUndefined();
        expect(store.getAllSymbols()).toEqual([]);
    });

    it('should add news articles for a symbol', () => {
        const store = createNewsStore(testApiKey);
        const articles = [
            { title: 'Article 1', content: 'Content 1', url: 'http://example.com/1', source: 'Source 1', publishedAt: '2025-01-01' },
            { title: 'Article 2', content: 'Content 2', url: 'http://example.com/2', source: 'Source 2', publishedAt: '2025-01-02' }
        ];

        store.addNews('BTC', articles);

        expect(store.getNewsForSymbol('BTC')).toHaveLength(2);
        expect(store.getNewsForSymbol('BTC')[0].title).toBe('Article 1');
        expect(store.getNewsForSymbol('BTC')[1].title).toBe('Article 2');
        expect(store.hasNewsForSymbol('BTC')).toBe(true);
    });

    it('should clear news articles for a symbol', () => {
        const store = createNewsStore(testApiKey);
        const articles = [
            { title: 'Article 1', content: 'Content 1', url: 'http://example.com/1', source: 'Source 1', publishedAt: '2025-01-01' }
        ];

        store.addNews('BTC', articles);
        store.clearNews('BTC');

        expect(store.getNewsForSymbol('BTC')).toHaveLength(0);
        expect(store.hasNewsForSymbol('BTC')).toBe(false);
    });

    it('should handle fetchNewsForSymbol successfully', async () => {
        const store = createNewsStore(testApiKey);

        await store.fetchNewsForSymbol('BTC', 10, 'date');

        const storeNews = store.getNewsForSymbol('BTC');

        console.log(storeNews);

        expect(storeNews).toHaveLength(10);
        expect(store.getNewsForSymbol('BTC')[0].title).toBeTypeOf("string");
        expect(store.isLoading).toBe(false);
        expect(store.error).toBeUndefined();
    });




    it('should throw an error for invalid symbol in collect_news', async () => {
        await expect(collect_news({ symbol: 'INVALID', apiKey: testApiKey, limit: 10, sort: 'date' }))
            .rejects.toThrow('Invalid symbol: INVALID. Must be one of BTC, ETH, XRP, LTC, ADA, DOGE, BNB, DOT, SOL, AVAX.');
    });

    it('should fetch news using collect_news', async () => {

        const result = await collect_news({ symbol: 'BTC', apiKey: testApiKey, limit: 1, sort: 'date' });

        expect(result).toBeDefined();
        expect(result.status).toBe(200);
        expect(result.articles).toBeDefined();
    });
});