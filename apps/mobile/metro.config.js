const { getDefaultConfig } = require("expo/metro-config");
const path = require("path");
const { withNativewind } = require("nativewind/metro");

const projectRoot = __dirname;
const monorepoRoot = path.resolve(projectRoot, "../..");

const config = getDefaultConfig(projectRoot);

const defaultResolveRequest = config.resolver.resolveRequest;
config.resolver.resolveRequest = (context, moduleName, platform) => {
  if (moduleName.endsWith(".js")) {
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

module.exports = withNativewind(config);
