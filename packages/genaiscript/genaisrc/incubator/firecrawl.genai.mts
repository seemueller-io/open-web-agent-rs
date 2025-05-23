script({
    title: "Stock Market News Scraper",
    tools: ["searxng"],
})

defTool({
    "mcp-server-firecrawl": {
        command: "npx",
        args: ["-y", "firecrawl-mcp"],
    },
})

def("QUERY_NEWS", "Latest news on AAPL")
def("QUERY_SENTIMENT", "Market sentiment for technology sector")


$`Search the query with searxng: QUERY_NEWS`


$`Scrape the top search result with firecrawl`


$`Search the query with searxng: QUERY_SENTIMENT`


$`Scrape the top search result with firecrawl`