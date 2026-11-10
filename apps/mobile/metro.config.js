const { getDefaultConfig } = require("expo/metro-config");
const path = require("path");
const { withNativeWind } = require("nativewind/metro");

const projectRoot = __dirname;
const monorepoRoot = path.resolve(projectRoot, "../..");
const reactNativeRoot = path.resolve(monorepoRoot, "node_modules/react-native");

const config = getDefaultConfig(projectRoot);

const defaultResolveRequest = config.resolver.resolveRequest;
config.resolver.resolveRequest = (context, moduleName, platform) => {
  if (
    moduleName === "react-native" ||
    moduleName.startsWith("react-native/")
  ) {
    const reactNativeContext = {
      ...context,
      originModulePath: path.join(reactNativeRoot, "index.js"),
    };

    return defaultResolveRequest
      ? defaultResolveRequest(reactNativeContext, moduleName, platform)
      : context.resolveRequest(reactNativeContext, moduleName, platform);
  }

  const originModulePath = context.originModulePath;
  const relativeOrigin = originModulePath
    ? path.relative(monorepoRoot, originModulePath)
    : "..";
  const isWorkspaceSource =
    relativeOrigin !== ".." &&
    !relativeOrigin.startsWith(`..${path.sep}`) &&
    !relativeOrigin.split(path.sep).includes("node_modules") &&
    !path.isAbsolute(relativeOrigin);

  if (
    moduleName.startsWith(".") &&
    moduleName.endsWith(".js") &&
    isWorkspaceSource
  ) {
    const sourceModuleName = moduleName.slice(0, -3);

    try {
      return context.resolveRequest(
        context,
        `${sourceModuleName}.ts`,
        platform,
      );
    } catch {
      // Fall through to Metro's default resolver for real JavaScript files.
    }
  }

  return defaultResolveRequest
    ? defaultResolveRequest(context, moduleName, platform)
    : context.resolveRequest(context, moduleName, platform);
};

// Support monorepo resolution
config.watchFolders = [monorepoRoot];
config.resolver.nodeModulesPaths = [
  path.resolve(projectRoot, "node_modules"),
  path.resolve(monorepoRoot, "node_modules"),
];

// Ensure extra node standard polyfills if requested
config.resolver.extraNodeModules = {
  buffer: require.resolve("buffer/"),
};

module.exports = withNativeWind(config, {
  input: "./src/global.css",
});
