import type { WipPatchType } from "./WipPatchType";

export interface WipPatch {
  oldFile: string;
  newFile: string;
  patchType: WipPatchType;
  stagedType: WipPatchType;
  unStagedType: WipPatchType;
  conflicted: boolean;
  id: string;
  isImage: boolean;
}
