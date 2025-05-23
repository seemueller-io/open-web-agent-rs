import {types} from "mobx-state-tree";
import {QuoteStore} from "@web-agent-rs/core/quotes/models";
import {NewsStore} from "@web-agent-rs/core/news";

import newsStore from "./news";
import quoteStore from "./quotes";


const StateModel = types.model("State", {
    symbols: types.array(types.string),
    quotes: QuoteStore,
    news: NewsStore,
});


const state = StateModel.create({
    quotes: quoteStore,
    news: newsStore,
});

export default state;