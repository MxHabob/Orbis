const {
  AndroidConfig,
  withAndroidColors,
  withAndroidColorsNight,
  withAndroidStyles,
  withGradleProperties,
} = require("expo/config-plugins");

module.exports = function withAndroidWindowLayout(config) {
  config = withAndroidColors(config, (modConfig) => {
    modConfig.modResults = AndroidConfig.Colors.assignColorValue(
      modConfig.modResults,
      { name: "orbis_status_bar_background", value: "#fcfcfc" },
    );
    return modConfig;
  });

  config = withAndroidColorsNight(config, (modConfig) => {
    modConfig.modResults = AndroidConfig.Colors.assignColorValue(
      modConfig.modResults,
      { name: "orbis_status_bar_background", value: "#1f1f1f" },
    );
    return modConfig;
  });

  config = withGradleProperties(config, (modConfig) => {
    const properties = modConfig.modResults;
    const existing = properties.find(
      (property) =>
        property.type === "property" && property.key === "edgeToEdgeEnabled",
    );

    if (existing) {
      existing.value = "false";
    } else {
      properties.push({
        type: "property",
        key: "edgeToEdgeEnabled",
        value: "false",
      });
    }

    return modConfig;
  });

  return withAndroidStyles(config, (modConfig) => {
    let styles = modConfig.modResults;
    styles = AndroidConfig.Styles.assignStylesValue(styles, {
      add: true,
      parent: AndroidConfig.Styles.getAppThemeGroup(),
      name: "android:statusBarColor",
      value: "@color/orbis_status_bar_background",
    });
    modConfig.modResults = styles;
    return modConfig;
  });
};
