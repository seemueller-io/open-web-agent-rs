script({
    isSystem: true
})

import {SearxngClient} from "@agentic/searxng";
import ky from 'ky';

const kyWithHeaders = ky.create({
    referrerPolicy: "unsafe-url",

    headers: {
        'Authorization': 'Basic ' + btoa(`admin:${process.env.SEARXNG_PASSWORD}`),
    }
});

const searxng = new SearxngClient({ky: kyWithHeaders});

defTool(searxng)