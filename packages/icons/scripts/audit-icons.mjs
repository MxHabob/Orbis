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

function getDefaultSizeClass(source) {
	return (
		source.match(/className\s*\|\|\s*\(\s*size\s*===\s*undefined\s*\?\s*['"]([^'"]+)['"]/)?.[1] ??
		source.match(/className\s*\|\|\s*['"]([^'"]+)['"]/)?.[1] ??
		null
	);
}

function honorsExplicitSvelteSize(source) {
	return /className\s*\|\|\s*\(\s*size\s*===\s*undefined\s*\?\s*['"]size-4['"]\s*:\s*['"][^'"]*['"]\s*\)/.test(
		source,
	);
}

function getSvelteMetadata(file) {
	const source = fs.readFileSync(path.join(svelteRoot, `${file}.svelte`), 'utf8');
	const defaultSizeClass = getDefaultSizeClass(source);
	const defaultSize = defaultSizeClass?.match(/\bsize-(\d+(?:\.\d+)?)/)?.[1];
	const svgAttributes = source.match(/<svg\b([^>]*)>/i)?.[1] ?? '';

	return {
		viewBox: getViewBox(source),
		defaultSize: defaultSize ? Number(defaultSize) * 4 : null,
		honorsExplicitSize: honorsExplicitSvelteSize(source),
		defaultColor: source.match(/\bstroke=["']([^"']+)["']/i)?.[1] ?? null,
		defaultLineCap: svgAttributes.match(/\bstroke-linecap=["']([^"']+)["']/i)?.[1] ?? null,
		defaultLineJoin: svgAttributes.match(/\bstroke-linejoin=["']([^"']+)["']/i)?.[1] ?? null,
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
	const svgAttributes = source.match(/<Svg\b([^>]*)>/)?.[1] ?? '';
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
		defaultLineCap: svgAttributes.match(/\bstrokeLinecap=["']([^"']+)["']/)?.[1] ?? null,
		defaultLineJoin: svgAttributes.match(/\bstrokeLinejoin=["']([^"']+)["']/)?.[1] ?? null,
		defaultStrokeWidth: source.match(/\bstrokeWidth\s*=\s*([\d.]+)/)?.[1] ?? null,
		primitives,
	};
}

function validateManifest(manifest, icons, aliases) {
	const manifestErrors = [];
	const entries = new Map();
	const families = new Set([
		'actions',
		'brand',
		'communication',
		'data',
		'education',
		'media',
		'navigation',
		'people',
		'security',
		'status',
		'utility',
	]);
	const kinds = new Set(['canonical', 'intentional-variant', 'brand-asset', 'compatibility-alias']);

	for (const entry of manifest.icons ?? []) {
		if (entries.has(entry.name)) manifestErrors.push(`Duplicate manifest entry for ${entry.name}`);
		entries.set(entry.name, entry);
	}

	for (const icon of icons) {
		const entry = entries.get(icon.name);
		if (!entry) {
			manifestErrors.push(`Missing manifest entry for ${icon.name}`);
			continue;
		}
		if (entry.files?.svelte !== icon.files.svelte || entry.files?.native !== icon.files.native) {
			manifestErrors.push(`Manifest source mismatch for ${icon.name}`);
		}
		if (!families.has(entry.family)) manifestErrors.push(`Invalid manifest family for ${icon.name}`);
		if (!kinds.has(entry.kind)) manifestErrors.push(`Invalid manifest kind for ${icon.name}`);
	}

	for (const name of entries.keys()) {
		if (!icons.some((icon) => icon.name === name)) manifestErrors.push(`Unknown manifest icon ${name}`);
	}

	const manifestAliases = new Map((manifest.nativeAliases ?? []).map((alias) => [alias.name, alias]));
	for (const alias of aliases) {
		const entry = manifestAliases.get(alias.name);
		if (!entry || entry.file !== alias.file || entry.kind !== 'compatibility-alias') {
			manifestErrors.push(`Missing or invalid manifest alias ${alias.name}`);
		}
	}
	for (const name of manifestAliases.keys()) {
		if (!aliases.some((alias) => alias.name === name)) manifestErrors.push(`Unknown manifest alias ${name}`);
	}

	return manifestErrors;
}

function validateExports(svelteExports, nativeExports, svelteFiles, nativeFiles) {
	const exportErrors = [];
	const allNames = new Set([...svelteExports.keys(), ...nativeExports.keys()]);

	for (const name of allNames) {
		const svelteFile = svelteExports.get(name);
		const nativeFile = nativeExports.get(name);
		if (!svelteFile) exportErrors.push(`Missing Svelte export for ${name}`);
		if (!nativeFile) exportErrors.push(`Missing native export for ${name}`);
		if (svelteFile && !svelteFiles.has(svelteFile)) {
			exportErrors.push(`Missing Svelte source for ${name}: ${svelteFile}`);
		}
		if (nativeFile && !nativeFiles.has(nativeFile)) {
			exportErrors.push(`Missing native source for ${name}: ${nativeFile}`);
		}
	}
	return exportErrors;
}

function validateAliases(aliases, nativeExports, nativeFiles) {
	const aliasErrors = [];
	for (const alias of aliases) {
		if (!nativeFiles.has(alias.file)) aliasErrors.push(`Missing native source for alias ${alias.name}: ${alias.file}`);
		if (![...nativeExports.values()].includes(alias.file)) {
			aliasErrors.push(`Native alias ${alias.name} has no canonical export for ${alias.file}`);
		}
	}
	return aliasErrors;
}

function validateFixtures() {
	const explicitSizeClassFixture =
		"class={cn(className || (size === undefined ? 'size-4' : ''), klass)}";
	if (!honorsExplicitSvelteSize(explicitSizeClassFixture)) {
		throw new Error('Explicit Svelte size fixture was not accepted');
	}
	if (honorsExplicitSvelteSize("class={cn(className || 'size-4', klass)}")) {
		throw new Error('Svelte default-size override fixture was not rejected');
	}
	if (getDefaultSizeClass(explicitSizeClassFixture) !== 'size-4') {
		throw new Error('Conditional default Svelte size was not detected');
	}

	const validFixture = {
		icons: [
			{
				name: 'FixtureIcon',
				files: { svelte: 'fixture-icon', native: 'fixture-icon' },
				family: 'actions',
				kind: 'canonical',
			},
		],
		nativeAliases: [{ name: 'FixtureIconAlias', file: 'fixture-icon', kind: 'compatibility-alias' }],
	};
	const validErrors = validateManifest(
		validFixture,
		[validFixture.icons[0]],
		[{ name: 'FixtureIconAlias', file: 'fixture-icon' }],
	);
	if (validErrors.length) throw new Error(`Valid manifest fixture failed: ${validErrors.join('; ')}`);

	const pairedExports = validateExports(
		new Map([['FixtureIcon', 'fixture-icon']]),
		new Map([['FixtureIcon', 'fixture-icon']]),
		new Set(['fixture-icon']),
		new Set(['fixture-icon']),
	);
	if (pairedExports.length) throw new Error(`Valid export fixture failed: ${pairedExports.join('; ')}`);

	const missingExportErrors = validateExports(
		new Map([['FixtureIcon', 'fixture-icon']]),
		new Map(),
		new Set(['fixture-icon']),
		new Set(),
	);
	if (!missingExportErrors.some((error) => error.includes('Missing native export'))) {
		throw new Error('Missing-pair fixture was not rejected');
	}

	const missingEntryErrors = validateManifest({ icons: [], nativeAliases: [] }, [validFixture.icons[0]], []);
	if (!missingEntryErrors.some((error) => error.includes('Missing manifest entry'))) {
		throw new Error('Missing-export fixture was not rejected');
	}

	const brokenAliasErrors = validateManifest(
		{ ...validFixture, nativeAliases: [{ name: 'FixtureIconAlias', file: 'missing-icon', kind: 'compatibility-alias' }] },
		[validFixture.icons[0]],
		[{ name: 'FixtureIconAlias', file: 'fixture-icon' }],
	);
	if (!brokenAliasErrors.some((error) => error.includes('Missing or invalid manifest alias'))) {
		throw new Error('Broken-alias fixture was not rejected');
	}

	const invalidAliasErrors = validateAliases(
		[{ name: 'FixtureIconAlias', file: 'missing-icon' }],
		new Map([['FixtureIcon', 'fixture-icon']]),
		new Set(['fixture-icon']),
	);
	if (invalidAliasErrors.length !== 2) throw new Error('Broken native alias fixture was not rejected');
}

function walkSourceFiles(directory) {
	return listFiles(directory).filter((file) => /\.(svelte|tsx?|jsx?)$/i.test(file));
}

function classifyIcon(name) {
	const normalized = name.toLowerCase();
	const customFamilies = {
		aicenter: 'status',
		employeeaffairs: 'people',
		financesystem: 'data',
		studentaffairs: 'people',
		templatelibrary: 'education',
	};
	const familyRules = [
		['brand', /\b(logo|github|youtube)\b/],
		['navigation', /(arrow|chevron|bars|menu|sidebar|home|grid|collapse|expand|maximize|restore)/],
		['status', /(check|info|question|shield|bell|notification|warning|error|sparkle|star|heart|bolt)/],
		['people', /(user|users|people|face|glasses)/],
		['education', /(academic|book|calculator|calendar|clipboard|clock|document|list|note|numbered|page|schedule|task|chart)/],
		['data', /(database|banknote|credit|receipt|scale|tag|label|usage|union)/],
		['communication', /(chat|bubble|phone|mic|voice|headphone|sound|link|share)/],
		['media', /(camera|photo|image|play)/],
		['security', /(lock|key|face-id|sign-out)/],
		['actions', /(plus|minus|x-mark|check|edit|pencil|pen|trash|garbage|refresh|reset|undo|download|upload|arrow-path|search|funnel|adjustment|setting|cog|wrench)/],
	];
	const family =
		customFamilies[normalized] ??
		familyRules.find(([, pattern]) => pattern.test(normalized))?.[0] ??
		'utility';
	const variant = /solid/i.test(name) ? 'solid' : /outline/i.test(name) ? 'outline' : 'default';
	return {
		family,
		variant,
		kind: family === 'brand' ? 'brand-asset' : variant === 'default' ? 'canonical' : 'intentional-variant',
	};
}

function readImportUsage(app, importPattern, exports) {
	const directory = path.join(workspaceRoot, `apps/${app}/src`);
	const counts = new Map();
	const sizes = { renderedIcons: 0, explicitSize: 0, unspecifiedSize: 0, explicitSizeValues: {}, classSizeValues: {} };
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
					const sizeValue = attributes.match(/\bsize\s*=\s*(?:\{([^}]+)\}|['"]([^'"]+)['"])/);
					if (sizeValue) {
						sizes.explicitSize += 1;
						const value = (sizeValue[1] ?? sizeValue[2]).trim();
						sizes.explicitSizeValues[value] = (sizes.explicitSizeValues[value] ?? 0) + 1;
					} else {
						sizes.unspecifiedSize += 1;
					}
					const classValue = attributes.match(/\bclass\s*=\s*['"]([^'"]+)['"]/);
					for (const [, value] of classValue?.[1].matchAll(/(?:^|\s)(?:size|w|h)-(\d+(?:\.\d+)?)(?:\s|$)/g) ?? []) {
						sizes.classSizeValues[value] = (sizes.classSizeValues[value] ?? 0) + 1;
					}
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
if (process.argv.includes('--self-test')) {
	validateFixtures();
	console.log('Icon audit fixtures passed');
	process.exit(0);
}
const svelteFiles = new Set(
	listFiles(svelteRoot)
		.filter((file) => file.endsWith('.svelte'))
		.map((file) => path.relative(svelteRoot, file).replace(/\.svelte$/, '').replaceAll(path.sep, '/')),
);
const nativeFiles = new Set(
	listFiles(nativeRoot)
		.filter((file) => file.endsWith('.tsx'))
		.map((file) => path.relative(nativeRoot, file).replace(/\.tsx$/, '').replaceAll(path.sep, '/')),
);
const allNames = new Set([...svelteExports.keys(), ...nativeExports.keys()]);
errors.push(...validateExports(svelteExports, nativeExports, svelteFiles, nativeFiles));
errors.push(...validateAliases(nativeAliases, nativeExports, nativeFiles));

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
		if (!svelte?.honorsExplicitSize) {
			errors.push(`Explicit Svelte size is overridden by the default class for ${name}`);
		}
		if (!['Github', 'Logo', 'Youtube'].includes(name)) {
			if (svelte?.defaultLineCap !== 'round') errors.push(`Unexpected Svelte default line cap for ${name}`);
			if (svelte?.defaultLineJoin !== 'round') errors.push(`Unexpected Svelte default line join for ${name}`);
			if (native?.defaultLineCap !== 'round') errors.push(`Unexpected native default line cap for ${name}`);
			if (native?.defaultLineJoin !== 'round') errors.push(`Unexpected native default line join for ${name}`);
		}
		if (!svelte?.viewBox || !native?.viewBox) errors.push(`Missing viewBox for ${name}`);

		return {
			name,
			files: { svelte: svelteFile, native: nativeFile },
			...classifyIcon(name),
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

const manifestFlag = process.argv.indexOf('--manifest-output');
if (manifestFlag >= 0) {
	const manifestPath = path.resolve(process.cwd(), process.argv[manifestFlag + 1]);
	fs.mkdirSync(path.dirname(manifestPath), { recursive: true });
	fs.writeFileSync(
		manifestPath,
		`${JSON.stringify(
			{
				version: 1,
				viewBox: '0 0 24 24',
				defaultSize: 16,
				defaultStrokeWidth: 1.75,
				icons: report.icons.map(({ name, files, family, variant, kind }) => ({
					name,
					files,
					family,
					variant,
					kind,
				})),
				nativeAliases: nativeAliases.map(({ name, file }) => ({
					name,
					file,
					kind: 'compatibility-alias',
				})),
			},
			null,
			2,
		)}\n`,
	);
}

const manifestPath = path.join(packageRoot, 'docs/icon-manifest.json');
if (fs.existsSync(manifestPath)) {
	const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
	errors.push(...validateManifest(manifest, icons, nativeAliases));
}

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
