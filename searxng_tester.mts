import { SearxngService, type SearxngServiceConfig, type SearxngSearchResult } from 'searxng';

const config: SearxngServiceConfig = {
    baseURL: 'https://search-engine-gsio.fly.dev',
    defaultSearchParams: {
        format: 'json',
        lang: 'auto',
    },
    defaultRequestHeaders: {
        'Content-Type': 'application/json',
    },
};

const searxngService = new SearxngService(config);

async function performSearch(query) {
    try {
        const results = await searxngService.search(query);
        console.log(results);
        return results;
    } catch (error) {
        console.error('Search failed:', error);
    }
}


const results = await performSearch('dogs');

console.log(JSON.stringify(results));