import type { DateResult } from "./DateResult";
import type { RefInfo } from "./RefInfo";

export interface Commit {
  author: string;
  email: string;
  date: DateResult;
  id: string;
  index: number;
  parentIds: Array<string>;
  isMerge: boolean;
  message: string;
  stashId: string | null;
  refs: Array<RefInfo>;
  filtered: boolean;
  numSkipped: number;
}
