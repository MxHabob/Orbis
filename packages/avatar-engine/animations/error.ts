export const errorAnimation = {
  name: "error",
  bodyScale: 0.96,
  bodyY: 1.0,
  bodyShake: true,
  eyeScaleX: 0.8,
  eyeScaleY: 0.8,
  eyeX: 0,
  eyeY: 0,
  glowColor: "var(--destructive, #ef4444)", // Destructive red theme color for error state
  glowOpacity: 0.8,
  springs: {
    body: { stiffness: 0.35, damping: 0.25 },
    eyes: { stiffness: 0.35, damping: 0.25 },
  },
};
