export const thinkAnimation = {
  name: "think",
  bodyScale: 1.0,
  bodyY: 0,
  bodyShake: false,
  eyeScaleX: 0.9,
  eyeScaleY: 1.0,
  eyeX: 3.5,
  eyeY: -4.0,
  glowColor: "var(--brand-pink, #d946ef)", // Pink theme color for thinking state
  glowOpacity: 0.55,
  springs: {
    body: { stiffness: 0.04, damping: 0.85 },
    eyes: { stiffness: 0.08, damping: 0.65 },
  },
};
