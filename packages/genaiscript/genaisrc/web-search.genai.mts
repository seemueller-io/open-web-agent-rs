import {SearxngClient} from "@agentic/searxng";
import "./tools/searxng.genai.mjs"


script({
    title: "web_search_agent",
    maxTokens: 8192,
    cache: false,
    tools: ["searxng"],
});



def("USER_INPUT", env.vars.user_input);



def("LINK_FORMAT", "[Link](url)");

$`You are an assistant searching for web content using complex queries to pinpoint results.

 
- tailor search to answer the question in USER_INPUT
- perform 2 searches in parallel sorted by relevance and date respectively
- create a markdown table of <=5 results of both searches
- header row: Title, Description, and Link
 
Respond with a single table, no extra text.`
