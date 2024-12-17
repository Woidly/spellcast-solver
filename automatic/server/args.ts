import { parseArgs } from "util";

export type Args = {
  dictionary: string;
  port: number;
  solver: string;
};

export default async function parse(): Promise<Args> {
  let { values } = parseArgs({
    options: {
      dictionary: {
        type: "string",
        short: "d",
      },
      port: {
        type: "string",
        short: "p",
      },
      solver: {
        type: "string",
        short: "s",
      },
    },
  });
  return {
    dictionary: await checkOrDefault(
      values.dictionary,
      ["../dictionary.txt", "dictionary.txt", "../../dictionary.txt"],
      "dictionary"
    ),
    port: parseInt(values.port || "27974") || 27974,
    solver: await checkOrDefault(
      values.solver,
      [
        "../target/release/spellcast-solver",
        "../../target/release/spellcast-solver",
        "target/release/spellcast-solver",
        "spellcast-solver",
      ],
      "solver"
    ),
  };
}

async function checkOrDefault(path: string | undefined, defaults: string[], what: string): Promise<string> {
  if (path) {
    if (await Bun.file(path).exists()) return path;
    console.error("Invalid path specified for", what);
    process.exit(1);
  }
  for (let def of defaults) {
    if (await Bun.file(def).exists()) {
      return def;
    }
  }
  console.error("Failed to get path for", what);
  process.exit(1);
}
