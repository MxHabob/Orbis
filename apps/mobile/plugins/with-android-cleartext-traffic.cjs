const { withAndroidManifest } = require("expo/config-plugins");

module.exports = function withAndroidCleartextTraffic(config, { enabled }) {
  return withAndroidManifest(config, (modConfig) => {
    const application = modConfig.modResults.manifest.application?.[0];
    if (!application) {
      throw new Error(
        "Could not find the Android application manifest to configure cleartext traffic.",
      );
    }

    application.$["android:usesCleartextTraffic"] = enabled ? "true" : "false";
    return modConfig;
  });
};
