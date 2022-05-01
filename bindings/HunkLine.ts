import type { HunkLineStatus } from "./HunkLineStatus";

export interface HunkLine {
  status: HunkLineStatus;
  oldNum: number | null;
  newNum: number | null;
  hunkIndex: number;
  text: string;
  index: number;
  lineEnding: string;
}
