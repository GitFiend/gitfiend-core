import type { CommitPoint } from "./CommitPoint";

export interface LineInstruction {
  points: Array<CommitPoint>;
  colour: string;
  id: string;
}
