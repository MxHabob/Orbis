export const STANDALONE_WINDOW_PARAM = "orbis_standalone";

export function isStandaloneWindow(url: URL): boolean {
  return url.searchParams.get(STANDALONE_WINDOW_PARAM) === "1";
}
