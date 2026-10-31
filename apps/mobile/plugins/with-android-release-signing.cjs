const { withAppBuildGradle } = require("expo/config-plugins");

const signingEnvironment = [
  "ORBIS_ANDROID_KEYSTORE_PATH",
  "ORBIS_ANDROID_KEYSTORE_PASSWORD",
  "ORBIS_ANDROID_KEY_ALIAS",
  "ORBIS_ANDROID_KEY_PASSWORD",
];

module.exports = function withAndroidReleaseSigning(config) {
  return withAppBuildGradle(config, (modConfig) => {
    const configuredValues = signingEnvironment.map((key) => process.env[key]);
    if (configuredValues.every((value) => !value)) return modConfig;
    if (configuredValues.some((value) => !value)) {
      throw new Error(
        "Android release signing requires ORBIS_ANDROID_KEYSTORE_PATH, ORBIS_ANDROID_KEYSTORE_PASSWORD, ORBIS_ANDROID_KEY_ALIAS, and ORBIS_ANDROID_KEY_PASSWORD.",
      );
    }

    const signingConfigsEndMarker = "\n    }\n    buildTypes {";
    if (!modConfig.modResults.contents.includes(signingConfigsEndMarker)) {
      throw new Error(
        "Could not find the Android buildTypes block to configure release signing.",
      );
    }

    let contents = modConfig.modResults.contents.replace(
      signingConfigsEndMarker,
      `
        release {
            storeFile file(System.getenv('ORBIS_ANDROID_KEYSTORE_PATH'))
            storePassword System.getenv('ORBIS_ANDROID_KEYSTORE_PASSWORD')
            keyAlias System.getenv('ORBIS_ANDROID_KEY_ALIAS')
            keyPassword System.getenv('ORBIS_ANDROID_KEY_PASSWORD')
        }
    }
    buildTypes {`,
    );
    const releaseSigningPattern =
      /(buildTypes\s*\{[\s\S]*?^\s*release\s*\{[\s\S]*?^\s*signingConfig\s+)[^\r\n]+/m;
    if (!releaseSigningPattern.test(contents)) {
      throw new Error(
        "Could not find the generated Android release signing configuration.",
      );
    }
    contents = contents.replace(
      releaseSigningPattern,
      "$1signingConfigs.release",
    );
    modConfig.modResults.contents = contents;
    return modConfig;
  });
};
