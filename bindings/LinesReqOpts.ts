// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Commit } from "./Commit";
import type { Patch } from "./Patch";

export interface LinesReqOpts {
  repoPath: string;
  commit: Commit;
  patch: Patch;
  searchText: string;
  numResults: number;
}
