// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { RefLocation } from "./RefLocation";
import type { RefType } from "./RefType";

export interface RefInfo { id: string, location: RefLocation, fullName: string, shortName: string, remoteName: string | null, siblingId: string | null, refType: RefType, head: boolean, commitId: string, time: number, }