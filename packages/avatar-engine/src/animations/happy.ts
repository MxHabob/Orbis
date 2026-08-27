export const happyAnimation = {
  name: "happy",
  bodyScale: 1.06,
  bodyY: -4.0,
  bodyShake: false,
  eyeScaleX: 1.15,
  eyeScaleY: 1.15,
  eyeX: 0,
  eyeY: -3.0,
  glowColor: "var(--brand-lime, #84cc16)", // Lime green success color for happy state
  glowOpacity: 0.8,
  springs: {
    body: { stiffness: 0.18, damping: 0.45 },
    eyes: { stiffness: 0.18, damping: 0.45 },
  },
};
