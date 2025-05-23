import {QuoteStore} from "@web-agent-rs/core/quotes/models";

const quoteStore = QuoteStore.create({
    apiKey: process.env.CCC_API_KEY
});

export default quoteStore;