export const blinkAnimation = {
  name: "blink",
  bodyScale: 1.0,
  bodyY: 0,
  bodyShake: false,
  eyeScaleX: 1.0,
  eyeScaleY: 0.05, // Almost closed
  eyeX: 0,
  eyeY: 0,
  glowColor: "var(--brand-purple, #a855f7)",
  glowOpacity: 0.65,
  springs: {
    body: { stiffness: 0.05, damping: 0.8 },
    eyes: { stiffness: 0.65, damping: 0.35 }, // Fast snap springs for blink
  },
};
