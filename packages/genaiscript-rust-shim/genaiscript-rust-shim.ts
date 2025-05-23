#!/usr/bin/env node
import minimist from "minimist";
import { run } from "genaiscript/api";
import { RunScriptOptions } from "./shim-types";

type Args = {
    file: string;
    vars: Record<string, unknown>;
    options: Partial<RunScriptOptions> & {
        envVars?: Record<string, string>;
        signal?: AbortSignal;
    };
};

async function wrapper(args: Args) {
    try {
        await run(args.file, [], { vars: args.vars }, args.options);
    } catch (error) {
        console.error("Error executing script:", error);
        process.exit(1);
    }
}

function parseCliArgs(): Args {
    const argv = minimist(process.argv.slice(2), {
        string: ["file"],
        alias: { f: "file" },
    });

    if (!argv.file) {
        console.error("Error: Missing required argument --file");
        process.exit(1);
    }

    const keyValuePairs = argv._;

    const vars: Record<string, unknown> = keyValuePairs.reduce((acc, pair) => {
        const [key, value] = pair.split("=");
        if (key && value !== undefined) {
            acc[key] = value; // Retain the `unknown` type for later flexibility
        } else {
            console.error(`Error: Invalid key=value pair "${pair}"`);
            process.exit(1);
        }
        return acc;
    }, {} as Record<string, unknown>);

    return {
        file: argv.file,
        vars,
        options: {},
    };
}

async function main() {

    const args = parseCliArgs();
    await wrapper(args);
}

main();
