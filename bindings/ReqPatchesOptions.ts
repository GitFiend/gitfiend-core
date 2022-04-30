import type { Commit } from "./Commit";

export interface ReqPatchesOptions {
  repoPath: string;
  commits: Array<Commit>;
}
