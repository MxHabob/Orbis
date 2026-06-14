export async function isDesktopApp() {
  if (typeof window === "undefined") return false;
  return window.navigator.userAgent.includes("Orbis Desktop App");
}
