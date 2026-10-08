import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifest = JSON.parse(fs.readFileSync(path.join(packageRoot, 'docs/icon-manifest.json'), 'utf8'));
const svelteRoot = path.join(packageRoot, 'src/svelte');
const outputPath = path.join(packageRoot, 'docs/icon-gallery.html');
const sizes = [16, 20, 24];

function getAttribute(attributes, name, fallback) {
	return attributes.match(new RegExp(`\\b${name}=["']([^"']+)["']`, 'i'))?.[1] ?? fallback;
}

function getSvgMarkup(icon) {
	const source = fs.readFileSync(
		path.join(svelteRoot, `${icon.files.svelte}.svelte`),
		'utf8',
	);
	const match = source.match(/<svg\b([^>]*)>([\s\S]*?)<\/svg>/i);
	if (!match) throw new Error(`Missing Svelte SVG root for ${icon.name}`);

	const [, rootAttributes, rawContents] = match;
	const viewBox = getAttribute(rootAttributes, 'viewBox', manifest.viewBox);
	const fill = getAttribute(rootAttributes, 'fill', 'none');
	const stroke = getAttribute(rootAttributes, 'stroke', 'currentColor');
	let contents = rawContents
		.replace(/<!--[\s\S]*?-->/g, '')
		.replace(/fill=\{dotClass \? undefined : '#A821D9'\}/g, 'fill="#A821D9"')
		.replace(/class=\{dotClass\}/g, '')
		.replace(/stroke-width=\{strokeWidth\}/g, 'stroke-width="1.75"');

	const unresolvedExpression = contents.match(/\{[^}]*\}/);
	if (unresolvedExpression) {
		throw new Error(`Unsupported Svelte expression in ${icon.name}: ${unresolvedExpression[0]}`);
	}

	return { viewBox, fill, stroke, contents };
}

function escapeAttribute(value) {
	return String(value)
		.replaceAll('&', '&amp;')
		.replaceAll('"', '&quot;')
		.replaceAll('<', '&lt;')
		.replaceAll('>', '&gt;');
}

const icons = manifest.icons.map((icon) => ({
	...icon,
	svg: getSvgMarkup(icon),
}));
const families = [...new Set(icons.map(({ family }) => family))].sort();
const cards = icons
	.map(({ name, family, variant, kind, svg }) => {
		const previews = ['light', 'dark']
			.map(
				(theme) => `
					<div class="theme-strip ${theme}">
						${sizes
							.map(
								(size) => `
									<div class="sample">
										<svg viewBox="${escapeAttribute(svg.viewBox)}" width="${size}" height="${size}" fill="${escapeAttribute(svg.fill)}" stroke="${escapeAttribute(svg.stroke)}" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${svg.contents}</svg>
										<span>${size}</span>
									</div>`,
							)
							.join('')}
					</div>`,
			)
			.join('');

		return `
			<article class="icon-card" data-name="${escapeAttribute(name.toLowerCase())}" data-family="${escapeAttribute(family)}">
				<header><strong>${escapeAttribute(name)}</strong><span>${escapeAttribute(family)} · ${escapeAttribute(variant)} · ${escapeAttribute(kind)}</span></header>
				${previews}
			</article>`;
	})
	.join('');

const html = `<!doctype html>
<html lang="en">
<head>
	<meta charset="utf-8">
	<meta name="viewport" content="width=device-width, initial-scale=1">
	<title>Orbis icon catalog</title>
	<style>
		:root { color-scheme: light; font: 14px/1.45 Inter, ui-sans-serif, system-ui, sans-serif; color: #20232b; background: #f4f5f8; }
		* { box-sizing: border-box; }
		body { margin: 0; padding: 28px clamp(16px, 4vw, 56px) 56px; }
		.toolbar { position: sticky; top: 0; z-index: 2; display: flex; gap: 12px; align-items: center; padding: 14px 0; background: rgb(244 245 248 / 94%); backdrop-filter: blur(12px); }
		h1 { margin: 0; font-size: 24px; letter-spacing: -.03em; }
		#search, #family { min-height: 38px; border: 1px solid #d9dce5; border-radius: 9px; background: white; color: inherit; padding: 0 12px; font: inherit; }
		#search { width: min(300px, 40vw); }
		#count { color: #737887; font-variant-numeric: tabular-nums; }
		.legend { margin: 8px 0 18px; color: #6e7380; }
		.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 12px; }
		.icon-card { overflow: hidden; border: 1px solid #e2e4eb; border-radius: 12px; background: white; box-shadow: 0 1px 2px rgb(20 24 35 / 4%); }
		.icon-card[hidden] { display: none; }
		.icon-card header { display: flex; flex-wrap: wrap; align-items: baseline; justify-content: space-between; gap: 6px; padding: 12px 14px 8px; }
		.icon-card header span { color: #777d89; font-size: 11px; }
		.theme-strip { display: flex; justify-content: space-around; align-items: center; min-height: 52px; padding: 8px 12px; }
		.theme-strip + .theme-strip { border-top: 1px solid #eceef2; }
		.theme-strip.light { color: #343844; background: #fff; }
		.theme-strip.dark { color: #e6e8ee; background: #20232b; }
		.sample { display: flex; min-width: 64px; align-items: center; justify-content: center; gap: 8px; }
		.sample span { color: #9297a2; font-size: 10px; font-variant-numeric: tabular-nums; }
		svg { flex: none; }
		@media (max-width: 540px) { .toolbar { flex-wrap: wrap; } .grid { grid-template-columns: 1fr; } }
	</style>
</head>
<body>
	<div class="toolbar">
		<h1>Orbis icons</h1>
		<input id="search" type="search" placeholder="Search icons" aria-label="Search icons">
		<select id="family" aria-label="Filter by family"><option value="">All families</option>${families.map((family) => `<option value="${escapeAttribute(family)}">${escapeAttribute(family)}</option>`).join('')}</select>
		<span id="count" aria-live="polite">${icons.length} / ${icons.length}</span>
	</div>
	<p class="legend">Every public icon · light and dark surfaces · 16, 20, and 24px · generated from the paired Svelte source manifest.</p>
	<main class="grid">${cards}
	</main>
	<script>
		const cards = [...document.querySelectorAll('.icon-card')];
		const search = document.querySelector('#search');
		const family = document.querySelector('#family');
		const count = document.querySelector('#count');
		function filterCards() {
			const query = search.value.trim().toLowerCase();
			let visible = 0;
			for (const card of cards) {
				const show = card.dataset.name.includes(query) && (!family.value || card.dataset.family === family.value);
				card.hidden = !show;
				if (show) visible += 1;
			}
			count.textContent = visible + ' / ' + cards.length;
		}
		search.addEventListener('input', filterCards);
		family.addEventListener('change', filterCards);
	</script>
</body>
</html>
`;

fs.writeFileSync(outputPath, html.replace(/[ \t]+(?=\r?$)/gm, ''));
console.log(`Generated visual gallery for ${icons.length} icons at ${path.relative(packageRoot, outputPath)}`);
