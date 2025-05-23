import {PerigonClient as NewsSearchTool} from "@agentic/perigon";

script({
    system: ["system.tools"],
    tools: "agent",
    maxTokens: 8192
})

const newSearchTool = new NewsSearchTool();

defTool(newSearchTool)

$`You are a chat assistant that uses agent tools to solve problems.

while true:
    - ask the user for a question using the agent_user_input
    - make a plan to answer the question step by step
    - answer the question
end while

## guidance:
    - use the agent tools to help you
    - do NOT try to ask the user questions directly, use the agent_user_input tool instead.
`