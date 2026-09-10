export { default as Avatar } from "./components/avatar.svelte";
export { AvatarController } from "./core/avatar-controller.svelte";
export {
  AvatarStateMachine,
  type AvatarState,
} from "./core/avatar-state-machine.js";
export { default as AvatarHats } from "./components/avatar-hats.svelte";
export { default as AvatarPilotPlane } from "./components/avatar-pilot-plane.svelte";
export type { AvatarHat } from "./core/types.js";
export { ALL_AVATAR_HATS } from "./core/types.js";
export { idleAnimation } from "./animations/idle.js";
export { blinkAnimation } from "./animations/blink.js";
export { thinkAnimation } from "./animations/think.js";
export { happyAnimation } from "./animations/happy.js";
export { errorAnimation } from "./animations/error.js";
