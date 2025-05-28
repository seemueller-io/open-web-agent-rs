import {entrypoint, InMemoryStore, MemorySaver, task} from "@langchain/langgraph"
import "./tools/searxng.genai.mjs"
import {SearxngClient} from "@agentic/searxng";
import ky from "ky";

script({
    title: "Deep Research Program",
    description: "Researchers can use this program to conduct deep research on a topic",
    model: "large",
    cache: "ephemeral",
})
const {output, vars} = env


const breakdownResearch = task(
    "breakdown_research",
    async (question: string) => {
        output.log("Breaking down question:", question);
        const result = await runPrompt(
            async (ctx) => {
                ctx.$`You are an expert research strategist.

Task: Break down the following research question into 3-5 focused sub-questions that would help comprehensively answer the main question.

Research question: ${question}

For each sub-question:
1. Assign a unique ID (e.g., SQ1, SQ2)
2. Explain the rationale for why this sub-question is important
3. Ensure the sub-questions collectively cover the main research question

Output the breakdown as a JSON object.`
            },
            {
                label: "breakdown research",
                responseSchema: {
                    type: "object",
                    properties: {
                        mainQuestion: {type: "string"},
                        subQuestions: {
                            type: "array",
                            items: {
                                type: "object",
                                properties: {
                                    id: {type: "string"},
                                    question: {type: "string"},
                                    rationale: {type: "string"},
                                },
                            },
                        },
                    },
                },
            }
        )
        output.fence(result.json, "json");
        return result.json
    }
)


const researchSubQuestion = task(
    "research_subquestion",
    async (subQuestion: { id: string; question: string }) => {
        output.log(`Researching sub-question: ${subQuestion.question}`);
        const kyWithHeaders = ky.create({
            referrerPolicy: "unsafe-url",

            headers: {
                'Authorization': 'Basic ' + btoa(`admin:${process.env.SEARXNG_PASSWORD}`),
            }
        });

        const searxng = new SearxngClient({ky: kyWithHeaders});

        const {json} = await runPrompt(
            (_) => {
                _.defTool(searxng)
                _.$`You are an expert researcher with access to comprehensive information.
    
Task: Thoroughly research the following question and create a JSON formatted response.

Question ID: ${subQuestion.id}
Question: ${subQuestion.question}

Respond with the specified JSON format.
`
            },
            {
                model: "small",
                label: `research subquestion ${subQuestion.id}`,
                maxDataRepairs: 2,
                responseType: "json_object",
                responseSchema: {
                    type: "object",
                    properties: {
                        subQuestionId: {type: "string"},
                        answer: {type: "string"},
                        sources: {
                            type: "array",
                            items: {
                                type: "object",
                                properties: {
                                    title: {type: "string"},
                                    url: {type: "string"},
                                    relevance: {type: "string"},
                                },
                            },
                        },
                        confidence: {type: "number"},
                    },
                },
            }
        )
        output.fence(json, "json");
        return json
    }
)

const synthesizeFindings = task(
    "synthesize_findings",
    async (mainQuestion: string, findings: any[]) => {
        output.log(`Synthesizing Findings: ${JSON.stringify(findings, null, 2)}`);
        const result = await runPrompt(
            async (ctx) => {
                ctx.$`You are an expert research synthesizer.
    
Task: Synthesize the following research findings into a JSON object to answer the main research question.

Main Research Question: ${mainQuestion}

Findings:
${JSON.stringify(findings, null, 2)}

Provide a synthesis that:
1. Directly answers the main research question
2. Integrates the findings from all sub-questions
3. Identifies limitations in the current research
4. Suggests next steps for further investigation

Respond in the specified JSON format.`
            },
            {
                label: "synthesize findings",
                responseType: "json_object",
                maxDataRepairs: 2,
                responseSchema: {
                    type: "object",
                    properties: {
                        summary: {type: "string"},
                        findings: {type: "array", items: {type: "string"}},
                        limitations: {
                            type: "array",
                            items: {type: "string"},
                        },
                        nextSteps: {type: "array", items: {type: "string"}},
                    },
                },
            }
        )
        output.fence(result.json, "json");
        return result.json
    }
)

const summarizeAndIdentifyGaps = task(
    {name: "summarize_and_identify_gaps"},
    async (synthesis: any, findings: any[]) => {
        output.log(`Summarizing and identifying gaps: ${JSON.stringify(findings, null, 2)}`);
        const result = await runPrompt(
            async (ctx) => {
                ctx.$`You are an expert research evaluator.
                        
Task: Review the research synthesis and identify any gaps or areas that need deeper investigation.

Current synthesis:
${JSON.stringify(synthesis, null, 2)}

Research findings:
${JSON.stringify(findings, null, 2)}

Please provide:
1. A concise summary of current findings
2. Identify 2-3 specific knowledge gaps
3. Formulate follow-up questions to address these gaps

Respond using the specified JSON schema.`
            },
            {
                label: "identify research gaps",
                maxDataRepairs: 2,
                responseSchema: {
                    type: "object",
                    properties: {
                        summary: {type: "string"},
                        gaps: {
                            type: "array",
                            items: {type: "string"},
                        },
                        followUpQuestions: {
                            type: "array",
                            items: {
                                type: "object",
                                properties: {
                                    id: {type: "string"},
                                    question: {type: "string"},
                                },
                            },
                        },
                    },
                },
            }
        )
        output.fence(result.json, "json");
        return result.json
    }
)

// Research Workflow
const researchWorkflow = entrypoint(
    {checkpointer: new MemorySaver(), name: "research_workflow", store: new InMemoryStore() },
    async (input: { question: string; context?: string }) => {
        output.log(`Deep research initiated`);
        // Step 1: Break down the research question
        const breakdown = await breakdownResearch(input.question)


        // Step 2: Research each sub-question in parallel
        const subQuestionFindings = []
        // handle both subQuestions and sub_questions, since the API returns one or the other
        const subquestions = breakdown?.sub_questions ? breakdown.sub_questions : breakdown.subQuestions;
        const forSq = await Promise.all(subquestions.map(async (q) => await researchSubQuestion(q)));
        forSq.map(subQuestionFindings.push)

        // Step 3: Synthesize the findings
        let synthesis = await synthesizeFindings(
            input.question,
            subQuestionFindings
        )

        const gapAnalysis = await summarizeAndIdentifyGaps(
            synthesis,
            subQuestionFindings
        )

        // Step 5: Conduct follow-up research on identified gaps
        const followUpFindings = [];
        for (const fq of gapAnalysis.followUpQuestions) {
            const anwser = await researchSubQuestion(fq);
            followUpFindings.push(anwser);
        }


        // Step 6: Final synthesis with deep research
        const allFindings = [...subQuestionFindings, ...followUpFindings]
        const finalSynthesis = await synthesizeFindings(
            input.question,
            allFindings
        )

        return {
            question: input.question,
            breakdown: breakdown,
            initialFindings: subQuestionFindings,
            gapAnalysis: gapAnalysis,
            followUpFindings: followUpFindings,
            synthesis: finalSynthesis,
        }
    }
)

// An arbitrary ID locked to this workflow run
const threadId = `research-${Date.now()}`

const options = {
    configurable: {thread_id: threadId},
};

const researchQuestion = env.vars.user_input;

const inputs =  {
    question: researchQuestion,
    context: vars.context || "",
};

// Execute workflow. Checkpoints are manually sent across the wire in the tasks.
const results = await researchWorkflow.invoke(
    inputs,
    {
        ...options,
    }
)

env.output.fence(results)