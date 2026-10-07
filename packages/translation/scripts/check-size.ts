import { brotliCompressSync } from "node:zlib";

const maxCompressedBytes = 50 * 1024;
const result = await Bun.build({
  entrypoints: ["./src/index.ts", "./src/dictionaries/academic.ts"],
  target: "browser",
  minify: true,
  splitting: true,
  write: false,
});

if (!result.success) {
  for (const message of result.logs) console.error(message);
  process.exit(1);
}

const compressedBytes = await Promise.all(
  result.outputs.map(
    async (output) =>
      brotliCompressSync(Buffer.from(await output.arrayBuffer())).byteLength,
  ),
).then((sizes) => sizes.reduce((total, size) => total + size, 0));

console.log(
  `Translation engine + base dictionary: ${(compressedBytes / 1024).toFixed(2)} KiB Brotli`,
);

if (compressedBytes > maxCompressedBytes) {
  console.error(
    `Translation package exceeds the ${maxCompressedBytes / 1024} KiB Brotli budget.`,
  );
  process.exit(1);
}
