import type { HunkLine } from "./HunkLine";
import type { HunkRange } from "./HunkRange";

export interface Hunk {
  oldLineRange: HunkRange;
  newLineRange: HunkRange;
  contextLine: string;
  lines: Array<HunkLine>;
  index: number;
}
