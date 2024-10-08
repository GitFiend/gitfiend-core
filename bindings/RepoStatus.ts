// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { BranchState } from "./BranchState";
import type { GitConfig } from "./GitConfig";
import type { WipPatches } from "./WipPatches";

export type RepoStatus = { patches: WipPatches, config: GitConfig, branches: Array<string>, branchName: string, headRefId: string, localCommitId: string | null, remoteCommitId: string | null, remoteAhead: number, remoteBehind: number, state: BranchState, };
