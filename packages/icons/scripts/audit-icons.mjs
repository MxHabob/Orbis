import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const workspaceRoot = path.resolve(packageRoot, '../..');
const svelteRoot = path.join(packageRoot, 'src/svelte');
const nativeRoot = path.join(packageRoot, 'src/native');
const svelteIndex = fs.readFileSync(path.join(svelteRoot, 'index.ts'), 'utf8');
const nativeIndex = fs.readFileSync(path.join(nativeRoot, 'index.ts'), 'utf8');
const errors = [];

function parseExports(source, platform) {
	const expression = /export\s*\{([^}]+)\}\s*from\s*['"]\.\/([^'"]+)['"]/g;
	const exports = new Map();
	const aliases = [];

	for (const [, names, sourceFile] of source.matchAll(expression)) {
		const file = sourceFile.replace(/\.(svelte|tsx?)$/, '');
		for (const entry of names.split(',')) {
			const specifier = entry.trim().split(/\s+as\s+/);
			const imported = specifier[0];
			const exported = specifier[1] ?? imported;

			if (imported === 'default') {
				if (platform === 'native') aliases.push({ name: exported, file });
				else exports.set(exported, file);
				continue;
			}

			exports.set(exported, file);
		}
	}

	return { exports, aliases };
}

function listFiles(directory) {
	return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
		const entryPath = path.join(directory, entry.name);
		return entry.isDirectory() ? listFiles(entryPath) : [entryPath];
	});
}

function getViewBox(source) {
	return source.match(/\bviewBox\s*=\s*["']([^"']+)["']/i)?.[1] ?? null;
}

function getSvelteMetadata(file) {
	const source = fs.readFileSync(path.join(svelteRoot, `${file}.svelte`), 'utf8');
	const defaultSizeClass = source.match(/className\s*\|\|\s*['"]([^'"]+)['"]/)?.[1] ?? null;
	const defaultSize = defaultSizeClass?.match(/\bsize-(\d+(?:\.\d+)?)/)?.[1];

	return {
		viewBox: getViewBox(source),
		defaultSize: defaultSize ? Number(defaultSize) * 4 : null,
		defaultColor: source.match(/\bstroke=["']([^"']+)["']/i)?.[1] ?? null,
		defaultStrokeWidth:
			source.match(/stroke-width=\{strokeWidth\s*\?\?\s*['"]([^'"]+)['"]\}/)?.[1] ?? null,
		primitives: [...source.matchAll(/<(path|circle|rect|line|polyline|polygon|g)\b/gi)]
			.map(([, primitive]) => primitive.toLowerCase())
			.reduce((counts, primitive) => {
				counts[primitive] = (counts[primitive] ?? 0) + 1;
				return counts;
			}, {}),
	};
}

function getNativeMetadata(file) {
	const source = fs.readFileSync(path.join(nativeRoot, `${file}.tsx`), 'utf8');
	const primitives = [...source.matchAll(/<(Path|Circle|Rect|Line|Polyline|Polygon|G)\b/g)]
		.map(([, primitive]) => primitive.toLowerCase())
		.reduce((counts, primitive) => {
			counts[primitive] = (counts[primitive] ?? 0) + 1;
			return counts;
		}, {});

	return {
		viewBox: getViewBox(source),
		defaultSize: Number(source.match(/\bsize\s*=\s*(\d+)/)?.[1] ?? NaN) || null,
		defaultColor: source.match(/\bcolor\s*=\s*['"]([^'"]+)['"]/)?.[1] ?? null,
		defaultStrokeWidth: source.match(/\bstrokeWidth\s*=\s*([\d.]+)/)?.[1] ?? null,
		primitives,
	};
}

function walkSourceFiles(directory) {
	return listFiles(directory).filter((file) => /\.(svelte|tsx?|jsx?)$/i.test(file));
}

function readImportUsage(app, importPattern, exports) {
	const directory = path.join(workspaceRoot, `apps/${app}/src`);
	const counts = new Map();
	const sizes = { renderedIcons: 0, explicitSize: 0, unspecifiedSize: 0 };
	const importingFiles = new Map();

	for (const filePath of walkSourceFiles(directory)) {
		const source = fs.readFileSync(filePath, 'utf8');
		for (const match of source.matchAll(importPattern)) {
			for (const item of match[1].split(',')) {
				const [importedName, alias] = item.trim().split(/\s+as\s+/);
				if (!importedName) continue;

				const file = exports.get(importedName);
				if (!file) continue;
				const localName = alias ?? importedName;
				counts.set(importedName, (counts.get(importedName) ?? 0) + 1);
				(importingFiles.get(importedName) ?? importingFiles.set(importedName, new Set()).get(importedName))
					.add(path.relative(workspaceRoot, filePath));

				const instances = [...source.matchAll(new RegExp(`<${localName}\\b([^>]*)>`, 'g'))];
				for (const [, attributes] of instances) {
					sizes.renderedIcons += 1;
					if (/\bsize\s*=/.test(attributes)) sizes.explicitSize += 1;
					else sizes.unspecifiedSize += 1;
				}
			}
		}
	}

	return {
		importingFiles: Object.fromEntries(
			[...counts.entries()].map(([name, count]) => [name, { count, files: [...importingFiles.get(name)].sort() }]),
		),
		renderedIconSizes: sizes,
	};
}

const { exports: svelteExports } = parseExports(svelteIndex, 'svelte');
const { exports: nativeExports, aliases: nativeAliases } = parseExports(nativeIndex, 'native');
const svelteFiles = new Set(
	listFiles(svelteRoot)
		.filter((file) => file.endsWith('.svelte'))
		.map((file) => path.basename(file, '.svelte')),
);
const nativeFiles = new Set(
	listFiles(nativeRoot)
		.filter((file) => file.endsWith('.tsx'))
		.map((file) => path.basename(file, '.tsx')),
);
const allNames = new Set([...svelteExports.keys(), ...nativeExports.keys()]);

for (const name of allNames) {
	const svelteFile = svelteExports.get(name);
	const nativeFile = nativeExports.get(name);

	if (!svelteFile) errors.push(`Missing Svelte export for ${name}`);
	if (!nativeFile) errors.push(`Missing native export for ${name}`);
	if (svelteFile && !svelteFiles.has(svelteFile)) errors.push(`Missing Svelte source for ${name}: ${svelteFile}`);
	if (nativeFile && !nativeFiles.has(nativeFile)) errors.push(`Missing native source for ${name}: ${nativeFile}`);
}

for (const alias of nativeAliases) {
	if (!nativeFiles.has(alias.file)) errors.push(`Missing native source for alias ${alias.name}: ${alias.file}`);
	if (![...nativeExports.values()].includes(alias.file)) {
		errors.push(`Native alias ${alias.name} has no canonical export for ${alias.file}`);
	}
}

const icons = [...allNames]
	.sort()
	.map((name) => {
		const svelteFile = svelteExports.get(name);
		const nativeFile = nativeExports.get(name);
		const svelte = svelteFile ? getSvelteMetadata(svelteFile) : null;
		const native = nativeFile ? getNativeMetadata(nativeFile) : null;

		if (svelte?.viewBox !== native?.viewBox) errors.push(`ViewBox mismatch for ${name}`);
		if (name !== 'Logo') {
			if (svelte?.defaultSize !== 16) errors.push(`Unexpected Svelte default size for ${name}`);
			if (svelte?.defaultStrokeWidth !== '1.75') errors.push(`Unexpected Svelte default stroke for ${name}`);
			if (native?.defaultSize !== 16) errors.push(`Unexpected native default size for ${name}`);
			if (native?.defaultStrokeWidth !== '1.75') errors.push(`Unexpected native default stroke for ${name}`);
		}
		if (!svelte?.viewBox || !native?.viewBox) errors.push(`Missing viewBox for ${name}`);

		return {
			name,
			files: { svelte: svelteFile, native: nativeFile },
			variant: /solid|outline/i.test(name) ? 'variant' : 'default',
			svelte,
			native,
		};
	});

const desktop = readImportUsage(
	'desktop',
	/import\s*\{([^}]+)\}\s*from\s*['"]@orbis\/icons(?:\/svelte)?['"]/gs,
	svelteExports,
);
const mobile = readImportUsage(
	'mobile',
	/import\s*\{([^}]+)\}\s*from\s*['"]@orbis\/icons\/native['"]/gs,
	nativeExports,
);
const report = {
	iconCount: icons.length,
	platformExports: {
		svelte: svelteExports.size,
		native: nativeExports.size,
	},
	nativeDefaultAliases: nativeAliases,
	platformSourceFiles: {
		svelte: svelteFiles.size,
		native: nativeFiles.size,
	},
	usage: {
		desktop: {
			distinctIcons: Object.keys(desktop.importingFiles).length,
			importedIcons: Object.entries(desktop.importingFiles)
				.map(([name, usage]) => ({ name, count: usage.count, files: usage.files.length }))
				.sort((left, right) => right.count - left.count || left.name.localeCompare(right.name)),
			renderedIconSizes: desktop.renderedIconSizes,
		},
		mobile: {
			distinctIcons: Object.keys(mobile.importingFiles).length,
			importedIcons: Object.entries(mobile.importingFiles)
				.map(([name, usage]) => ({ name, count: usage.count, files: usage.files.length }))
				.sort((left, right) => right.count - left.count || left.name.localeCompare(right.name)),
			renderedIconSizes: mobile.renderedIconSizes,
		},
	},
	icons,
	errors,
};

const outputFlag = process.argv.indexOf('--output');
if (outputFlag >= 0) {
	const outputPath = path.resolve(process.cwd(), process.argv[outputFlag + 1]);
	fs.mkdirSync(path.dirname(outputPath), { recursive: true });
	fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
}

console.log(
	JSON.stringify({
		iconCount: report.iconCount,
		platformExports: report.platformExports,
		platformSourceFiles: report.platformSourceFiles,
		desktopTop: report.usage.desktop.importedIcons.slice(0, 10),
		mobileUsage: report.usage.mobile,
		errors: report.errors,
	}),
);

if (errors.length > 0) process.exitCode = 1;
