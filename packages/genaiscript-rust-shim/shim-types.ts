type FenceFormat = "markdown" | "xml" | "none"

export interface WorkspaceFile {
    /**
     * Name of the file, relative to project root.
     */
    filename: string

    /**
     * Content mime-type if known
     */
    type?: string

    /**
     * Encoding of the content
     */
    encoding?: "base64"

    /**
     * Content of the file.
     */
    content?: string
}

export interface RunScriptOptions {
    excludedFiles: string[]
    excludeGitIgnore: boolean
    runRetry: string
    out: string
    retry: string
    retryDelay: string
    maxDelay: string
    json: boolean
    yaml: boolean
    outTrace: string
    outOutput: string
    outAnnotations: string
    outChangelogs: string
    pullRequest: string
    pullRequestComment: string | boolean
    pullRequestDescription: string | boolean
    pullRequestReviews: boolean
    outData: string
    label: string
    temperature: string | number
    topP: string | number
    seed: string | number
    maxTokens: string | number
    maxToolCalls: string | number
    maxDataRepairs: string | number
    model: string
    smallModel: string
    visionModel: string
    embeddingsModel: string
    modelAlias: string[]
    provider: string
    csvSeparator: string
    cache: boolean | string
    cacheName: string
    applyEdits: boolean
    failOnErrors: boolean
    removeOut: boolean
    vars: string[] | Record<string, string | boolean | number | object>
    fallbackTools: boolean
    jsSource: string
    logprobs: boolean
    topLogprobs: number
    fenceFormat: FenceFormat
    workspaceFiles?: WorkspaceFile[]
}
