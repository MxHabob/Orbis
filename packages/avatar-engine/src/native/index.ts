export { AvatarController } from "../core/avatar-controller";
export {
  type AvatarState,
  AvatarStateMachine,
} from "../core/avatar-state-machine";
export {
  type AvatarSvgElement,
  type AvatarSvgOptions,
  getAvatarDrawing,
  getAvatarHatDrawing,
  renderAvatarSvg,
} from "../core/avatar-svg";
export {
  ALL_AVATAR_HATS,
  type AvatarHat,
  type InteractiveAvatarHat,
  pickNextAvatarHat,
  type ResolvedAvatarHat,
  resolveAvatarHat,
} from "../core/types";
export type { NativeAvatarProps } from "./avatar";
export { Avatar } from "./avatar";
