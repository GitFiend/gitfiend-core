// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Patch } from "./Patch";

export interface PollSearchResult {
  searchId: number;
  complete: boolean;
  results: Array<[string, Array<Patch>]> | null;
}