import type { Commit } from "./Commit";
import type { WipPatch } from "./WipPatch";

export interface ReqWipHunksOptions {
  repoPath: string;
  patch: WipPatch;
  headCommit: Commit | null;
}
