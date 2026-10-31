const {
  AndroidConfig,
  withAndroidStyles,
  withGradleProperties,
} = require("expo/config-plugins");

module.exports = function withAndroidWindowLayout(config) {
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
      value: "?android:colorBackground",
    });
    modConfig.modResults = styles;
    return modConfig;
  });
};
