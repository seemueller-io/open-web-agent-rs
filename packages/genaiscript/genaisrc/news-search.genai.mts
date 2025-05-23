import {SearxngClient} from "@agentic/searxng";
import "./tools/searxng.genai.mjs"

script({
    title: "news_search_agent",
    tools: ["searxng"],
    maxToolCalls: 2,
    cache: false,
});

def("USER_INPUT", env.vars.user_input);
def("TODAY", new Date().toISOString().split("T")[0]);
def("LINK_FORMAT", "[Link](url)");


$`You are an assistant searching for news using complex queries to pinpoint results.

 
- tailor search to answer the question in USER_INPUT
- perform 2 searches in parallel sorted by relevance and date respectively
- create a markdown table of <=5 results of both searches
- header row: Date, Title, Summary, and Link
 
Respond with a single table, no extra text.`
