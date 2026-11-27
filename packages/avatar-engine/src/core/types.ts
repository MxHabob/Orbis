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
  | "flower"
  | "duck"
  | "cat"
  | "cowboy"
  | "beanie"
  | "rainbow"
  | "apple"
  | "leaf"
  | "random";

export type ResolvedAvatarHat = Exclude<AvatarHat, "random">;
export type InteractiveAvatarHat = Exclude<AvatarHat, "random" | "none">;

/** Single source of truth for available hats. */
export const ALL_AVATAR_HATS: readonly InteractiveAvatarHat[] = [
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
  "flower",
  "duck",
  "cat",
  "cowboy",
  "beanie",
  "rainbow",
  "apple",
  "leaf",
];

export function resolveAvatarHat(hat: AvatarHat = "random"): ResolvedAvatarHat {
  if (hat !== "random") return hat;
  const index = Math.floor(Math.random() * ALL_AVATAR_HATS.length);
  return ALL_AVATAR_HATS[index] ?? "graduation";
}

/** Pick a random hat different from `currentHat` for interactive cycling. */
export function pickNextAvatarHat(
  currentHat?: AvatarHat,
  hats: readonly InteractiveAvatarHat[] = ALL_AVATAR_HATS,
): InteractiveAvatarHat {
  const candidates = hats.filter((hat) => hat !== currentHat);
  const pool = candidates.length > 0 ? candidates : hats;
  const index = Math.floor(Math.random() * pool.length);
  return pool[index] ?? "graduation";
}
