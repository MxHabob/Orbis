const {
  withGradleProperties,
  withProjectBuildGradle,
} = require("expo/config-plugins");

const NDK_VERSION = "30.0.16248370";
const CMAKE_OBJECT_PATH_MAX = "200";
const ROOT_PLUGIN = 'apply plugin: "expo-root-project"';
const REACT_ROOT_PLUGIN = 'apply plugin: "com.facebook.react.rootproject"';
const NATIVE_REGENERATION_MARKER =
  "// Orbis: stabilize Android native CMake builds";

const NATIVE_REGENERATION_CONFIG = `${NATIVE_REGENERATION_MARKER}
subprojects { nativeProject ->
  def configureNativeCmake = {
    def androidExtension = nativeProject.extensions.findByName("android")
    def cmakeOptions = androidExtension?.defaultConfig?.externalNativeBuild?.cmake
    if (cmakeOptions != null) {
      cmakeOptions.arguments("-DCMAKE_SUPPRESS_REGENERATION=ON")
    }
  }

  ["com.android.application", "com.android.library"].each { pluginId ->
    nativeProject.plugins.withId(pluginId) { configureNativeCmake() }
  }
}`;

module.exports = function withAndroidNativeBuildConfig(config) {
  config = withProjectBuildGradle(config, (modConfig) => {
    if (modConfig.modResults.language !== "groovy") {
      throw new Error(
        "The Android native build config plugin supports Groovy build.gradle files only.",
      );
    }

    const assignment = `ext.ndkVersion = "${NDK_VERSION}"`;
    let { contents } = modConfig.modResults;
    if (!contents.includes(ROOT_PLUGIN)) {
      throw new Error(
        "Could not find the Expo root project plugin in android/build.gradle.",
      );
    }

    if (!contents.includes(assignment)) {
      contents = contents.replace(ROOT_PLUGIN, `${assignment}\n${ROOT_PLUGIN}`);
    }

    if (!contents.includes(REACT_ROOT_PLUGIN)) {
      throw new Error(
        "Could not find the React Native root project plugin in android/build.gradle.",
      );
    }

    if (!contents.includes(NATIVE_REGENERATION_MARKER)) {
      contents = contents.replace(
        REACT_ROOT_PLUGIN,
        `${REACT_ROOT_PLUGIN}\n\n${NATIVE_REGENERATION_CONFIG}`,
      );
    }

    modConfig.modResults.contents = contents;
    return modConfig;
  });
  return withGradleProperties(config, (modConfig) => {
    const properties = modConfig.modResults;
    const existing = properties.find(
      (property) =>
        property.type === "property" &&
        property.key === "expo.android.cmakeObjectPathMax",
    );

    if (existing) {
      existing.value = CMAKE_OBJECT_PATH_MAX;
    } else {
      properties.push({
        type: "property",
        key: "expo.android.cmakeObjectPathMax",
        value: CMAKE_OBJECT_PATH_MAX,
      });
    }

    return modConfig;
  });
};
