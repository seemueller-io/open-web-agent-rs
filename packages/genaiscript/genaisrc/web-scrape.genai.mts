import {Window} from 'happy-dom';
import {platform} from 'os';

script({
    title: "scrape",
    cache: false,
});

/*
  "url": "Full URL in the conversation that references the URL being interacted with. No trailing slash!",
  "query": "Implied question about the resources at the URL.",
  "action": "read | scrape | crawl"
*/

try {
    const {url, query, action} = JSON.parse(env.vars.user_input);
} catch (e) {
    throw "Sorry! Something went wrong.";
}

const {url, query, action} = JSON.parse(env.vars.user_input);

def("URL", url);

def("QUERY", query);

def("ACTION", action);

// console.log({url, query, action});

if(!(new URL(url) ?? undefined)) {
    throw "Bad URL. Maybe try again?"
}

function getBrowser(): "webkit" | "chromium" | "firefox" {
    if (platform() === 'darwin') {
        return "webkit"; // macOS is identified by 'darwin'
    }
    return "chromium"; // default to chromium for other platforms
}

const {text} = await host.fetchText(new URL(url).toString());

// const browser = getBrowser();

// const page = await host.browse(new URL(url).toString(), {
//     browser: getBrowser(),
//     headless: true,
//     javaScriptEnabled: browser !== "chromium",
//     // timeout: 3777,
//     // bypassCSP: true,
//     // baseUrl: new URL(url).origin,
// });
//
// const html = (await page.content());
// const title = (await page.title());

// console.log({html});

const window = new Window({
    // url: "http://localhost:8080",
    height: 1920,
    width: 1080,
    settings: {
        navigator: {
            userAgent: 'Mozilla/5.0 (compatible; GeoffsAI/1.0; +https://geoff.seemueller.io)',
        },
    }
});

window.document.body.innerHTML = text;

const textContent = window.document.body.textContent;

def("PAGE_TEXT", textContent);

$`You a helpful assistant interacting with resources found at the URL.

- markdown table is concise representation of PAGE_TEXT relevant to the QUERY

### Respond Example:
### Data from ${url}:
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Data 1   | Data 2   | Data 3   |
\n---[Example explanation of data significance to query.]
---
Respond with the markdown table and an explanation of significance. Do not include extra text.`;
