// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { FileMatch } from "./FileMatch";

export interface PollSearchResult {
  searchId: number;
  complete: boolean;
  results: Array<[string, Array<FileMatch>]> | null;
}
