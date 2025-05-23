import {NewsStore} from "@web-agent-rs/core/news";
import {Instance} from "mobx-state-tree";

const newsStore = NewsStore.create({
    isLoading: false,
    apiKey: process.env.PERIGON_API_KEY
});

export type NewsStore = Instance<typeof newsStore>;

export default newsStore;