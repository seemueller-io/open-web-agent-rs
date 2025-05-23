import {task, entrypoint, interrupt, MemorySaver} from "@langchain/langgraph"
import "./tools/searxng.genai.mjs"
import {SearxngClient} from "@agentic/searxng";


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

        return result.json
    }
)

const globalCtx = this;


const researchSubQuestion = task(
    "research_subquestion",
    async (subQuestion: { id: string; question: string }) => {

        const searxng = new SearxngClient({apiBaseUrl: "https://search-engine-gsio.fly.dev"});

        const {text} = await runPrompt(
            (_) => {
                _.defTool(searxng)
                _.$`You are an expert researcher with access to comprehensive information.
    
Task: Thoroughly research the following question and provide a detailed answer.

Question ID: ${subQuestion.id}
Question: ${subQuestion.question}

Provide your findings in a structured format that includes:
- Your answer to the sub-question
- Relevant sources that support your answer
- Your confidence level in the answer (0-1)`
            },
            {
                model: "small",
                label: `research subquestion ${subQuestion.id}`,
                maxDataRepairs: 2,
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
        return text
    }
)


const synthesizeFindings = task(
    "synthesize_findings",
    async (mainQuestion: string, findings: any[]) => {
        const result = await runPrompt(
            async (ctx) => {
                ctx.$`You are an expert research synthesizer.
    
Task: Synthesize the following research findings into a coherent response to the main research question.

Main Research Question: ${mainQuestion}

Findings:
${JSON.stringify(findings, null, 2)}

Provide a synthesis that:
1. Directly answers the main research question
2. Integrates the findings from all sub-questions
3. Identifies limitations in the current research
4. Suggests next steps for further investigation`
            },
            {
                label: "synthesize findings",
                responseType: "markdown",
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

        return result.json
    }
)


const summarizeAndIdentifyGaps = task(
    "summarize_and_identify_gaps",
    async (synthesis: any, findings: any[]) => {
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
3. Formulate follow-up questions to address these gaps`
            },
            {
                label: "identify research gaps",
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
        return result.json
    }
)


const researchWorkflow = entrypoint(
    {checkpointer: new MemorySaver(), name: "research_workflow"},
    async (input: { question: string; context?: string }) => {

        const breakdown = await breakdownResearch(input.question)


        const subQuestionFindings = []

        for (const sq of breakdown.subQuestions) {
            const analysis = await researchSubQuestion(sq);
            console.log(analysis);
            subQuestionFindings.push(analysis);
        }


        let synthesis = await synthesizeFindings(
            input.question,
            subQuestionFindings
        )

        const gapAnalysis = await summarizeAndIdentifyGaps(
            synthesis,
            subQuestionFindings
        )


        const followUpFindings = [];
        for (const fq of gapAnalysis.followUpQuestions) {
            const anwser = await researchSubQuestion(fq);
            console.log(anwser);
            followUpFindings.push(anwser);
        }


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


const researchQuestion =
    env.vars.question ||
    "What are the most promising approaches to climate change mitigation?"


const threadId = `research-${Date.now()}`


const config = {
    configurable: {
        thread_id: threadId,
    },
}


const results = await researchWorkflow.invoke(
    {
        question: researchQuestion,
        context: vars.context || "",
    },
    config
)
output.fence(results, "json")