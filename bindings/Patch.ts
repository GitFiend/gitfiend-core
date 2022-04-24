import type { PatchType } from "./PatchType";

export interface Patch { commitId: string, oldFile: string, newFile: string, patchType: PatchType, id: string, isImage: boolean, }