export const isDesktopApp = (() => {
  if (typeof window === "undefined") return false;

  const internals = Reflect.get(window, "__TAURI_INTERNALS__");
  return typeof internals?.invoke === "function";
})();
