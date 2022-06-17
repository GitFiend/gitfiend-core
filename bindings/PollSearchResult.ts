import type { Patch } from "./Patch";

export interface PollSearchResult {
  searchId: number;
  complete: boolean;
  results: Array<[string, Array<Patch>]> | null;
}
