export type AvatarHat =
  | "none"
  | "crown"
  | "graduation"
  | "firefighter"
  | "wizard"
  | "detective"
  | "chef"
  | "astronaut"
  | "headset"
  | "pilot"
  | "party"
  | "pirate"
  | "ninja"
  | "tophat"
  | "viking"
  | "propeller"
  | "santa"
  | "random";

/** Single source of truth for available hats. */
export const ALL_AVATAR_HATS: readonly Exclude<AvatarHat, "random">[] = [
  "graduation",
  "crown",
  "wizard",
  "detective",
  "astronaut",
  "headset",
  "pilot",
  "party",
  "pirate",
  "ninja",
  "tophat",
  "viking",
  "propeller",
  "santa",
  "chef",
  "firefighter",
];

export function resolveAvatarHat(
  hat: AvatarHat = "random",
): Exclude<AvatarHat, "random"> {
  if (hat !== "random") return hat;
  const index = Math.floor(Math.random() * ALL_AVATAR_HATS.length);
  return ALL_AVATAR_HATS[index] ?? "graduation";
}
