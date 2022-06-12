import type { LineInstruction } from "./LineInstruction";
import type { PointInstruction } from "./PointInstruction";

export interface Instructions {
  points: Array<PointInstruction>;
  lines: Array<LineInstruction>;
}
