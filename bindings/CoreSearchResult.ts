import type { Patch } from "./Patch";
import type { SearchMatchType } from "./SearchMatchType";

export interface CoreSearchResult {
  commitId: string;
  matches: Array<SearchMatchType>;
  patches: Array<Patch>;
}
