// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type CommitFilter =
  | { branch: { id: string; short_name: string } }
  | { user: { author: string; email: string } }
  | { commit: { commit_id: string } }
  | { file: { file_name: string } };