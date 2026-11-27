const { withGradleProperties } = require("expo/config-plugins");

module.exports = function withAndroidWindowLayout(config) {
  return withGradleProperties(config, (modConfig) => {
    const properties = modConfig.modResults;
    const existing = properties.find(
      (property) =>
        property.type === "property" && property.key === "edgeToEdgeEnabled",
    );

    if (existing) {
      existing.value = "true";
    } else {
      properties.push({
        type: "property",
        key: "edgeToEdgeEnabled",
        value: "true",
      });
    }

    return modConfig;
  });
};
