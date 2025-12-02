// util/cli.ts
import { readFileSync } from "node:fs";
import { join } from "node:path";

export type PartFn = (input: string) => unknown;

type RunOptions = {
  dir: string;            // usually import.meta.dir from the caller
  part1: PartFn;
  part2?: PartFn;
};

export function runCli({ dir, part1, part2 }: RunOptions) {
  const [, , partArg, inputArg] = process.argv;

  const part = Number(partArg ?? "1");
  if (part !== 1 && part !== 2) {
    console.error("Usage: bun run <day>/index.ts [1|2] [sample|input]");
    process.exit(1);
  }

  const filePath = join(dir, inputArg);
  const input = readFileSync(filePath, "utf8");

  const fn = part === 1 ? part1 : (part2 ?? part1);
  const result = fn(input);

  console.log(result);
}
