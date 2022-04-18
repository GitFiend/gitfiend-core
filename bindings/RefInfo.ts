import type { RefLocation } from "./RefLocation";
import type { RefType } from "./RefType";

export interface RefInfo { id: string,  location: RefLocation, full_name: string, short_name: string, remote_name: string | null, sibling_id: string | null, ref_type: RefType, head: boolean, commit_id: string, time: bigint, }