import fs from "node:fs/promises";
import path from "node:path";

const ICONS_API = "https://api.github.com/repos/open-webui/open-webui/contents/src/lib/components/icons";

const PKG_DIR = path.resolve(__dirname, "../packages/icons");
const SVELTE_DIR = path.join(PKG_DIR, "src/svelte");
const NATIVE_DIR = path.join(PKG_DIR, "src/native");

interface GitHubContentItem {
  name: string;
  download_url: string;
  type: string;
}

// Convert PascalCase/camelCase to kebab-case for file names
function toKebabCase(str: string): string {
  return str
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .replace(/([A-Z])([A-Z][a-z])/g, "$1-$2")
    .toLowerCase();
}

// Convert kebab-case attributes to camelCase for React Native SVG
function toCamelCase(str: string): string {
  return str.replace(/-([a-z0-9])/g, (_, g1) => g1.toUpperCase());
}

function convertInnerToReactNativeSvg(rawInner: string): string {
  const tagMap: Record<string, string> = {
    path: "Path",
    circle: "Circle",
    rect: "Rect",
    line: "Line",
    polyline: "Polyline",
    polygon: "Polygon",
    g: "G",
    defs: "Defs",
    clippath: "ClipPath",
    mask: "Mask",
    use: "Use",
    symbol: "Symbol",
    linearGradient: "LinearGradient",
    radialGradient: "RadialGradient",
    stop: "Stop",
  };

  let converted = rawInner;

  for (const [tag, Component] of Object.entries(tagMap)) {
    const openRegex = new RegExp(`<${tag}(\\s|>|/)`, "gi");
    converted = converted.replace(openRegex, `<${Component}$1`);

    const closeRegex = new RegExp(`</${tag}\\s*>`, "gi");
    converted = converted.replace(closeRegex, `</${Component}>`);
  }

  const attrsToCamel = [
    "stroke-width",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-miterlimit",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-opacity",
    "fill-rule",
    "fill-opacity",
    "clip-rule",
    "clip-path",
    "stop-color",
    "stop-opacity",
  ];

  for (const attr of attrsToCamel) {
    const camel = toCamelCase(attr);
    const regex = new RegExp(`${attr}=`, "g");
    converted = converted.replace(regex, `${camel}=`);
  }

  converted = converted.replace(/\sclass="[^"]*"/g, "");
  converted = converted.replace(/\sclassName="[^"]*"/g, "");
  converted = converted.replace(/\sstyle="[^"]*"/g, "");

  return converted.trim();
}

function parseSvgContent(content: string) {
  const svgStartIdx = content.indexOf("<svg");
  if (svgStartIdx === -1) return null;

  const svgOpenTagEndIdx = content.indexOf(">", svgStartIdx);
  if (svgOpenTagEndIdx === -1) return null;

  const svgCloseMatch = content.match(/<\/svg\s*>/i);
  if (!svgCloseMatch || svgCloseMatch.index === undefined) return null;
  const svgCloseIdx = svgCloseMatch.index;

  const rawAttrs = content.substring(svgStartIdx + 4, svgOpenTagEndIdx);
  const innerSvg = content.substring(svgOpenTagEndIdx + 1, svgCloseIdx).trim();

  const viewBoxMatch = rawAttrs.match(/viewBox="([^"]*)"/i);
  const viewBox = viewBoxMatch ? viewBoxMatch[1] : "0 0 24 24";

  const fillMatch = rawAttrs.match(/fill="([^"]*)"/i);
  const fill = fillMatch ? fillMatch[1] : "currentColor";

  const strokeMatch = rawAttrs.match(/stroke="([^"]*)"/i);
  const stroke = strokeMatch ? strokeMatch[1] : "currentColor";

  const strokeWidthMatch =
    rawAttrs.match(/stroke-width="([^"]*)"/i) ||
    rawAttrs.match(/stroke-width={([^}]*)}/i);
  const defaultStrokeWidth = strokeWidthMatch
    ? strokeWidthMatch[1].replace(/['"]/g, "")
    : "1.5";

  return {
    rawAttrs,
    innerSvg,
    viewBox,
    fill,
    stroke,
    defaultStrokeWidth,
  };
}

async function run() {
  console.log("Cleaning and preparing icon directories with system kebab-case naming...");
  await fs.rm(SVELTE_DIR, { recursive: true, force: true });
  await fs.rm(NATIVE_DIR, { recursive: true, force: true });
  await fs.mkdir(SVELTE_DIR, { recursive: true });
  await fs.mkdir(NATIVE_DIR, { recursive: true });

  const res = await fetch(ICONS_API, {
    headers: { "User-Agent": "bun-script" },
  });

  if (!res.ok) {
    throw new Error(`Failed to fetch icons list: ${res.status} ${res.statusText}`);
  }

  const items: GitHubContentItem[] = await res.json();
  const svelteFiles = items.filter(
    (item) => item.type === "file" && item.name.endsWith(".svelte")
  );

  console.log(`Found ${svelteFiles.length} icon files. Downloading and converting with kebab-case filenames...`);

  const iconEntries: { componentName: string; fileName: string }[] = [];
  let successCount = 0;

  const batchSize = 30;
  for (let i = 0; i < svelteFiles.length; i += batchSize) {
    const batch = svelteFiles.slice(i, i + batchSize);
    await Promise.all(
      batch.map(async (item) => {
        const componentName = item.name.replace(/\.svelte$/, "");
        const fileName = toKebabCase(componentName);

        const fileRes = await fetch(item.download_url);
        if (!fileRes.ok) {
          throw new Error(`Failed to download ${item.name}: ${fileRes.status}`);
        }
        const svelteContent = await fileRes.text();

        const parsed = parseSvgContent(svelteContent);
        if (!parsed) {
          console.warn(`[ERROR] Could not parse SVG in ${item.name}`);
          return;
        }

        iconEntries.push({ componentName, fileName });
        successCount++;

        const { innerSvg, viewBox, fill, stroke, defaultStrokeWidth } = parsed;

        // Generate Svelte 5 component (kebab-case filename)
        const svelteCode = `<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';

	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string;
		className?: string;
		size?: number | string;
		strokeWidth?: number | string;
	}

	let {
		class: klass = '',
		className = '',
		size,
		strokeWidth,
		...restProps
	}: Props = $props();

	const computedClass = $derived([className || 'size-4', klass].filter(Boolean).join(' '));
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	viewBox="${viewBox}"
	fill="${fill}"
	stroke="${stroke}"
	stroke-width={strokeWidth ?? '${defaultStrokeWidth}'}
	width={size}
	height={size}
	class={computedClass}
	aria-hidden="true"
	{...restProps}
>
	${innerSvg}
</svg>
`;

        await fs.writeFile(path.join(SVELTE_DIR, `${fileName}.svelte`), svelteCode, "utf-8");

        // Generate React Native component (kebab-case filename)
        const nativeInner = convertInnerToReactNativeSvg(innerSvg);
        const nativeFillAttr = fill === "currentColor" ? "fill={color}" : `fill="${fill}"`;
        const nativeStrokeAttr = stroke === "currentColor" ? "stroke={color}" : `stroke="${stroke}"`;

        const nativeCode = `import React from 'react';
import Svg, {
  Path,
  Circle,
  Rect,
  Line,
  Polyline,
  Polygon,
  G,
  Defs,
  ClipPath,
  Mask,
  Use,
  Symbol,
  LinearGradient,
  RadialGradient,
  Stop,
  type SvgProps,
} from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface ${componentName}Props extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ${componentName}({
  size = 20,
  color = 'currentColor',
  strokeWidth = ${defaultStrokeWidth},
  style,
  ...props
}: ${componentName}Props) {
  return (
    <Svg
      viewBox="${viewBox}"
      width={size}
      height={size}
      ${nativeFillAttr}
      ${nativeStrokeAttr}
      strokeWidth={strokeWidth}
      style={style}
      aria-hidden={true}
      {...props}
    >
      ${nativeInner}
    </Svg>
  );
}

export default ${componentName};
`;

        await fs.writeFile(path.join(NATIVE_DIR, `${fileName}.tsx`), nativeCode, "utf-8");
      })
    );
    console.log(`Processed batch ${Math.min(i + batchSize, svelteFiles.length)} / ${svelteFiles.length}`);
  }

  // Sort alphabetically by component name
  iconEntries.sort((a, b) => a.componentName.localeCompare(b.componentName));

  // Generate Svelte index
  const svelteIndex = iconEntries
    .map(
      ({ componentName, fileName }) =>
        `export { default as ${componentName} } from './${fileName}.svelte';`
    )
    .join("\n") + "\n";
  await fs.writeFile(path.join(SVELTE_DIR, "index.ts"), svelteIndex, "utf-8");

  // Generate Native index
  const nativeIndex = iconEntries
    .map(
      ({ componentName, fileName }) =>
        `export { ${componentName}, default as ${componentName}Default, type ${componentName}Props } from './${fileName}';`
    )
    .join("\n") + "\n";
  await fs.writeFile(path.join(NATIVE_DIR, "index.ts"), nativeIndex, "utf-8");

  // Generate Root index (re-exports for bundlers)
  const rootIndex = `export * from './svelte/index';\n`;
  await fs.writeFile(path.join(PKG_DIR, "src/index.ts"), rootIndex, "utf-8");

  console.log(`Successfully generated ALL ${successCount} / ${svelteFiles.length} icons in kebab-case format!`);
}

run().catch((err) => {
  console.error("Error generating icons:", err);
  process.exit(1);
});
