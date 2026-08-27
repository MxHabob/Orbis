export const idleAnimation = {
  name: "idle",
  bodyScale: 1.0,
  bodyY: 0,
  bodyShake: false,
  eyeScaleX: 1.0,
  eyeScaleY: 1.0,
  eyeX: 0,
  eyeY: 0,
  glowColor: "var(--brand-purple, #a855f7)", // Maps to primary theme purple
  glowOpacity: 0.65,
  springs: {
    body: { stiffness: 0.05, damping: 0.8 },
    eyes: { stiffness: 0.08, damping: 0.6 },
  },
};
