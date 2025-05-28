#!/usr/bin/env deno -A

const API_ROOT = "http://localhost:3006";

const sid = crypto.randomUUID();
// -------------------- 1.  Create the agent --------------------
const createAgentBody = {
  id: sid,
  resource: "web-search",
  parent: sid,
  payload: { input: "What is the capital of France?" },
};

const createRes = await fetch(`${API_ROOT}/api/agents`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify(createAgentBody),
});


const raw = await createRes.text();
console.log({raw});
const {stream_url: streamId} = JSON.parse(raw);

console.log("Agent created with streamId:", streamId);

// -------------------- 2.  Listen to the SSE stream --------------------
const streamUrl = `${API_ROOT}${streamId}`;
const es = new EventSource(streamUrl);


es.onopen = (e) => {
  console.log("connected", e);
};

es.onmessage = (e) => {
  console.log("⟶", e.data);
};

es.onerror = (e) => {
  console.error("SSE error:", e);
  es.close();
};