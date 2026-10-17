import type { ExpoConfig } from "expo/config";

const configuredProjectId =
  process.env.EAS_PROJECT_ID?.trim() ||
  process.env.EXPO_PUBLIC_EAS_PROJECT_ID?.trim();
const easProjectId = configuredProjectId || undefined;

if (
  easProjectId &&
  !/^[\da-f]{8}-(?:[\da-f]{4}-){3}[\da-f]{12}$/i.test(easProjectId)
) {
  throw new Error("EAS_PROJECT_ID must be a valid Expo project UUID.");
}

export default ({ config }: { config: ExpoConfig }): ExpoConfig => {
  const updatesEnabled = Boolean(easProjectId);
  const android = {
    ...config.android,
    usesCleartextTraffic: process.env.ORBIS_DEV_ALLOW_LOCAL_HTTP === "true",
  } as NonNullable<ExpoConfig["android"]> & {
    usesCleartextTraffic: boolean;
  };

  return {
    ...config,
    android,
    plugins: [
      ...(config.plugins ?? []),
      ["expo-sqlite", { useSQLCipher: true }],
    ],
    ...(updatesEnabled
      ? { runtimeVersion: { policy: "appVersion" as const } }
      : {}),
    updates: {
      ...config.updates,
      enabled: updatesEnabled,
      checkAutomatically: updatesEnabled ? "ON_LOAD" : "NEVER",
      ...(easProjectId
        ? { url: `https://u.expo.dev/${easProjectId}` }
        : {}),
    },
    extra: {
      ...config.extra,
      ...(easProjectId
        ? {
            eas: {
              ...config.extra?.eas,
              projectId: easProjectId,
            },
          }
        : {}),
    },
  };
};
