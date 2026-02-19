const path = require("path");

// Explicitly resolve tailwindcss from THIS package's node_modules to avoid
// Bun workspace hoisting picking up tailwindcss@4 from @midday/desktop-client.
const tailwindcss = require(require.resolve("tailwindcss", { paths: [__dirname] }));
const autoprefixer = require(require.resolve("autoprefixer", { paths: [__dirname] }));

module.exports = {
  plugins: [
    tailwindcss({ config: path.join(__dirname, "tailwind.config.ts") }),
    autoprefixer,
  ],
};
