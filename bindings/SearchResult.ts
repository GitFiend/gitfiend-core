import type { SearchMatchType } from "./SearchMatchType";

export interface SearchResult {
  commitId: string;
  matches: Array<SearchMatchType>;
}
