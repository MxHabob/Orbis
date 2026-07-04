<script lang="ts">
// BackgroundPattern.svelte
</script>

<!-- Ambient gradient background (drawn behind icons) -->
<div class="absolute inset-0 bg-linear-to-t from-background via-background/40 to-background pointer-events-none z-0"></div>

<!-- Ambient Canvas with SVG scattered school-themed background icons (drawn on top of gradient for crisp visibility) -->
<svg class="absolute inset-0 w-full h-full text-primary/[0.22] dark:text-primary/[0.15] pointer-events-none select-none z-0" 
	viewBox="0 0 1440 900" 
	preserveAspectRatio="xMidYMid slice" 
	xmlns="http://www.w3.org/2000/svg">
	<defs>
		<!-- Graduation Cap (Centered at 0,0) -->
		<g id="icon-cap">
			<path d="M0 -8 L18 0 L0 8 L-18 0 Z" fill="none" stroke="currentColor" stroke-width="1.5" />
			<path d="M-10 3.5 L-10 9 C-10 12 10 12 10 9 L10 3.5" fill="none" stroke="currentColor" stroke-width="1.5" />
			<path d="M12 0.5 L15 6 L15 9" fill="none" stroke="currentColor" stroke-width="1.2" />
			<circle cx="15" cy="10" r="0.8" fill="currentColor" />
		</g>
		
		<!-- Globe (Centered at 0,0) -->
		<g id="icon-globe">
			<circle cx="0" cy="0" r="14" fill="none" stroke="currentColor" stroke-width="1.5" />
			<ellipse cx="0" cy="0" rx="14" ry="5" fill="none" stroke="currentColor" stroke-width="1.5" />
			<ellipse cx="0" cy="0" rx="5" ry="14" fill="none" stroke="currentColor" stroke-width="1.5" />
			<line x1="-14" y1="0" x2="14" y2="0" stroke="currentColor" stroke-width="0.8" stroke-dasharray="1 1" />
			<line x1="0" y1="-14" x2="0" y2="14" stroke="currentColor" stroke-width="0.8" stroke-dasharray="1 1" />
		</g>
		
		<!-- Pencil & Ruler (Centered at 0,0) -->
		<g id="icon-pencil">
			<path d="M-12 8 L8 -12 L14 -6 L-6 14 Z" fill="none" stroke="currentColor" stroke-width="1.5" />
			<path d="M-12 8 L-17 13 L-6 14 Z" fill="none" stroke="currentColor" stroke-width="1.5" />
			<path d="M-17 13 L-19 15" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
			<path d="M-7 -17 L18 8" stroke="currentColor" stroke-width="1.5" />
			<line x1="-3 -13" x2="0 -16" stroke="currentColor" stroke-width="0.8" />
			<line x1="2 -8" x2="5 -11" stroke="currentColor" stroke-width="0.8" />
			<line x1="7 -3" x2="10 -6" stroke="currentColor" stroke-width="0.8" />
			<line x1="12 2" x2="15 -1" stroke="currentColor" stroke-width="0.8" />
		</g>
		
		<!-- Lightbulb (Centered at 0,0) -->
		<g id="icon-bulb">
			<circle cx="0" cy="0" r="10" fill="none" stroke="currentColor" stroke-width="1.5" />
			<path d="M-4 10 L4 10 M-3 13 L3 13 M-1 16 L1 16" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
			<line x1="0" y1="-14" x2="0" y2="-10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
			<line x1="14" y1="0" x2="10" y2="0" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
			<line x1="-14" y1="0" x2="-10" y2="0" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
			<line x1="10" y1="-10" x2="13" y2="-13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
			<line x1="-10" y1="-10" x2="-13" y2="-13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
		</g>
	</defs>

	<!-- Scattered Icons around the canvas (avoiding center area for card visibility) -->
	<!-- Top Left Corner -->
	<use href="#icon-cap" x="120" y="150" transform="rotate(-15 120 150) scale(1.1)" />
	<use href="#icon-pencil" x="220" y="80" transform="rotate(-20 220 80) scale(0.95)" />
	<use href="#icon-globe" x="380" y="100" transform="rotate(20 380 100) scale(0.9)" />
	<use href="#icon-bulb" x="150" y="270" transform="rotate(-30 150 270) scale(1.0)" />
	
	<!-- Top Middle / Right -->
	<use href="#icon-cap" x="650" y="70" transform="rotate(12 650 70) scale(0.95)" />
	<use href="#icon-bulb" x="800" y="110" transform="rotate(40 800 110) scale(0.85)" />
	<use href="#icon-pencil" x="920" y="90" transform="rotate(-35 920 90) scale(1.2)" />
	<use href="#icon-bulb" x="1250" y="140" transform="rotate(10 1250 140) scale(1.0)" />
	<use href="#icon-globe" x="1420" y="100" transform="rotate(15 1420 100) scale(1.15)" />

	<!-- Side Left -->
	<use href="#icon-pencil" x="80" y="380" transform="rotate(45 80 380) scale(0.95)" />
	<use href="#icon-bulb" x="240" y="550" transform="rotate(-25 240 550) scale(1.15)" />

	<!-- Side Right -->
	<use href="#icon-cap" x="1360" y="350" transform="rotate(30 1360 350) scale(1.0)" />
	<use href="#icon-globe" x="1180" y="500" transform="rotate(-10 1180 500) scale(1.25)" />
	<use href="#icon-pencil" x="1400" y="580" transform="rotate(35 1400 580) scale(1.05)" />

	<!-- Bottom Left Corner -->
	<use href="#icon-globe" x="150" y="720" transform="rotate(15 150 720) scale(1.05)" />
	<use href="#icon-cap" x="280" y="780" transform="rotate(25 280 780) scale(0.9)" />
	<use href="#icon-cap" x="420" y="820" transform="rotate(-20 420 820) scale(1.2)" />
	<use href="#icon-bulb" x="100" y="860" transform="rotate(-40 100 860) scale(1.0)" />

	<!-- Bottom Middle / Right -->
	<use href="#icon-pencil" x="600" y="840" transform="rotate(-45 600 840) scale(1.1)" />
	<use href="#icon-bulb" x="850" y="830" transform="rotate(35 850 830) scale(0.9)" />
	<use href="#icon-globe" x="1040" y="840" transform="rotate(25 1040 840) scale(1.1)" />
	<use href="#icon-pencil" x="1200" y="760" transform="rotate(-15 1200 760) scale(1.1)" />
	<use href="#icon-globe" x="1320" y="720" transform="rotate(10 1320 720) scale(1.0)" />
	<use href="#icon-cap" x="1380" y="820" transform="rotate(5 1380 820) scale(0.85)" />
</svg>
