export { blinkAnimation } from "./animations/blink.js";
export { errorAnimation } from "./animations/error.js";
export { happyAnimation } from "./animations/happy.js";
export { idleAnimation } from "./animations/idle.js";
export { thinkAnimation } from "./animations/think.js";
export { default as Avatar } from "./components/avatar.svelte";
export { default as AvatarHats } from "./components/avatar-hats.svelte";
export { default as AvatarPilotPlane } from "./components/avatar-pilot-plane.svelte";
export { AvatarController } from "./core/avatar-controller.svelte";
export {
  type AvatarState,
  AvatarStateMachine,
} from "./core/avatar-state-machine.js";
export type { AvatarHat } from "./core/types.js";
export { ALL_AVATAR_HATS } from "./core/types.js";
