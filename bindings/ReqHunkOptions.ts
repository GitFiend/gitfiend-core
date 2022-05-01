import type { Commit } from "./Commit";
import type { Patch } from "./Patch";

export interface ReqHunkOptions {
  repoPath: string;
  commit: Commit;
  patch: Patch;
}
