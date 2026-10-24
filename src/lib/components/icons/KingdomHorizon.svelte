<!--
	Decorative night-time kingdom horizon (Kingdom Two Crowns mood): moonlit sky,
	layered forests, a small castle, the crowned monarch on horseback, and the whole
	land mirrored in the river below. Purely decorative (aria-hidden). Every color is
	derived from the theme variables, so it follows the palette.
-->
<script lang="ts">
	import type { SVGAttributes } from "svelte/elements";

	let { class: className, ...restProps }: SVGAttributes<SVGSVGElement> = $props();
	const uid = $props.id();

	const W = 1200;
	const GROUND = 268; // shore line: land above, river below
	const H = 360;

	// Deterministic PRNG (mulberry32): the silhouettes are identical on every render.
	function rng(seed: number) {
		return () => {
			seed = (seed + 0x6d2b79f5) | 0;
			let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
			t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
			return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
		};
	}
	const r1 = (v: number) => Math.round(v * 10) / 10;
	const clamp01 = (v: number) => Math.min(1, Math.max(0, v));

	// Tiered pine, right half only: [half-width, depth] as fractions of the tree height.
	const PINE: [number, number][] = [
		[0.07, 0.17],
		[0.035, 0.17],
		[0.12, 0.38],
		[0.065, 0.38],
		[0.17, 0.6],
		[0.1, 0.6],
		[0.22, 0.9],
		[0.035, 0.9],
		[0.035, 1],
	];

	function pine(x: number, base: number, h: number, w: number) {
		const top = base - h;
		const right = PINE.map(([dx, dy]) => `L${r1(x + dx * h * w)} ${r1(top + dy * h)}`);
		const left = [...PINE].reverse().map(([dx, dy]) => `L${r1(x - dx * h * w)} ${r1(top + dy * h)}`);
		return `M${r1(x)} ${r1(top)}${right.join("")}${left.join("")}Z`;
	}

	/** A row of pines merged into one path, standing on a solid forest floor `floor` units high. */
	function forest(
		seed: number,
		x0: number,
		x1: number,
		base: number,
		height: (x: number) => number,
		gap: number,
		floor = 2,
	) {
		const rand = rng(seed);
		let d = "";
		for (let x = x0; x <= x1; x += gap * (0.6 + rand() * 0.8)) {
			const h = height(x) * (0.8 + rand() * 0.4);
			if (h > 3) d += pine(x, base + rand() * 2, h, 0.85 + rand() * 0.3);
		}
		return `${d}M${x0} ${base - floor}H${x1}V${GROUND}H${x0}Z`;
	}

	/** A block with crenellations (n merlons) on top, standing on the ground line. */
	function crenellated(x: number, w: number, top: number, mh: number, n: number) {
		const sw = w / (2 * n - 1);
		let d = `M${x} ${GROUND}V${top - mh}`;
		for (let i = 0; i < 2 * n - 1; i++) {
			d += `H${r1(x + (i + 1) * sw)}`;
			if (i < 2 * n - 2) d += `V${i % 2 === 0 ? top : top - mh}`;
		}
		return `${d}V${GROUND}Z`;
	}

	function palisade(x: number, seed: number) {
		const rand = rng(seed);
		let d = "";
		for (let i = 0; i < 5; i++) {
			const sx = x + i * 5;
			const top = GROUND - 20 - rand() * 5;
			d += `M${sx} ${GROUND}V${r1(top + 3)}L${sx + 2.2} ${r1(top)}L${sx + 4.4} ${r1(top + 3)}V${GROUND}Z`;
		}
		return d;
	}

	// Distant treeline across the whole width; side forests grow toward the edges.
	const farForest = forest(7, -10, W + 10, GROUND - 6, () => 20, 9);
	const nearLeft = forest(11, -10, 320, GROUND - 2, (x) => 20 + 70 * clamp01((330 - x) / 330), 11, 5);
	const nearRight = forest(23, 880, W + 10, GROUND - 2, (x) => 20 + 70 * clamp01((x - 870) / 330), 11, 5);
	const frontLeft = forest(5, -20, 235, GROUND, (x) => 50 + 150 * clamp01((240 - x) / 240) ** 1.2, 17, 14);
	const frontRight = forest(9, 965, W + 20, GROUND, (x) => 50 + 150 * clamp01((x - 960) / 240) ** 1.2, 17, 14);

	const castle = [
		crenellated(504, 16, 224, 4, 2), // outer turrets
		crenellated(680, 16, 224, 4, 2),
		crenellated(518, 164, 236, 4, 13), // curtain wall
		crenellated(574, 52, 180, 6, 4), // keep
		"M534 268V206H558V268ZM530 206L546 174L562 206Z", // left tower + roof
		"M642 268V206H666V268ZM638 206L654 174L670 206Z", // right tower + roof
		"M599.3 174V145H600.7V174Z", // flag pole
	].join("");

	const walls = [
		crenellated(346, 14, 236, 4, 2),
		crenellated(840, 14, 236, 4, 2),
		palisade(264, 3),
		palisade(914, 4),
	].join("");

	const MOON = { x: 840, y: 156, r: 34 };

	const stars = (() => {
		const rand = rng(42);
		const out: { x: number; y: number; r: number; o: number; d: number | null }[] = [];
		while (out.length < 34) {
			const x = rand() * W;
			const y = 8 + rand() * 200;
			const r = 0.5 + rand() * 1.1;
			const o = 0.35 + rand() * 0.55;
			const tw = rand() < 0.3 ? r1(rand() * 6) : null;
			if (Math.hypot(x - MOON.x, y - MOON.y) < MOON.r + 40) continue;
			out.push({ x: r1(x), y: r1(y), r: r1(r), o: r1(o), d: tw });
		}
		return out;
	})();

	// The reflection is sliced into bands, each nudged sideways like a gently moving river.
	const bands = [
		{ y: GROUND, h: 7, dx: 0 },
		{ y: 276, h: 8, dx: 2.5 },
		{ y: 285, h: 10, dx: -2 },
		{ y: 296, h: 12, dx: 3.5 },
		{ y: 309, h: 14, dx: -3 },
		{ y: 324, h: 17, dx: 2 },
		{ y: 342, h: 18, dx: -1.5 },
	];

	const ripples = [
		[110, 283, 90],
		[300, 291, 60],
		[520, 279, 120],
		[690, 300, 80],
		[960, 286, 110],
		[400, 313, 70],
		[1040, 319, 60],
		[200, 327, 100],
		[620, 336, 90],
	];

	const glints = [
		[-15, 278, 30, 0.7],
		[-6, 284, 14, 0.55],
		[-20, 292, 26, 0.5],
		[2, 293, 12, 0.4],
		[-9, 301, 18, 0.4],
		[-16, 311, 10, 0.3],
		[-2, 312, 14, 0.3],
		[-8, 324, 16, 0.2],
	];
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	viewBox="0 0 1200 360"
	preserveAspectRatio="xMidYMax slice"
	class={className}
	aria-hidden="true"
	{...restProps}
>
	<defs>
		<linearGradient id="{uid}-sky" x1="0" y1="0" x2="0" y2={GROUND} gradientUnits="userSpaceOnUse">
			<stop offset="0" class="stop-bg" />
			<stop offset="0.5" class="stop-dusk" />
			<stop offset="0.8" class="stop-glow" />
			<stop offset="1" class="stop-horizon" />
		</linearGradient>
		<linearGradient id="{uid}-water" x1="0" y1={GROUND} x2="0" y2={H} gradientUnits="userSpaceOnUse">
			<stop offset="0" class="stop-water" />
			<stop offset="1" class="stop-bg" />
		</linearGradient>
		<linearGradient id="{uid}-fade" x1="0" y1="300" x2="0" y2={H} gradientUnits="userSpaceOnUse">
			<stop offset="0" class="stop-bg" stop-opacity="0" />
			<stop offset="1" class="stop-bg" />
		</linearGradient>
		<radialGradient id="{uid}-halo">
			<stop offset="0" class="stop-gold" stop-opacity="0.22" />
			<stop offset="0.4" class="stop-gold" stop-opacity="0.08" />
			<stop offset="1" class="stop-gold" stop-opacity="0" />
		</radialGradient>
		{#each bands as band, i (i)}
			<clipPath id="{uid}-band{i}">
				<rect x="-10" y={band.y} width={W + 20} height={band.h} />
			</clipPath>
		{/each}
	</defs>

	<!-- Sky -->
	<rect width={W} height={GROUND} fill="url(#{uid}-sky)" />
	<g class="star">
		{#each stars as s, i (i)}
			<circle
				cx={s.x}
				cy={s.y}
				r={s.r}
				opacity={s.o}
				class={{ twinkle: s.d !== null }}
				style:animation-delay={s.d !== null ? `${s.d}s` : undefined}
			/>
		{/each}
	</g>
	<circle cx={MOON.x} cy={MOON.y} r={MOON.r * 3.4} fill="url(#{uid}-halo)" />
	<circle cx={MOON.x} cy={MOON.y} r={MOON.r} class="moon" />
	<g class="crater">
		<circle cx={MOON.x - 12} cy={MOON.y - 8} r="7" />
		<circle cx={MOON.x + 10} cy={MOON.y + 12} r="5" />
		<circle cx={MOON.x + 14} cy={MOON.y - 14} r="3" />
	</g>

	<!-- Land (also reused, mirrored, for the reflection) -->
	<g id="{uid}-land">
		<path
			class="hill"
			d="M-10 268V238C60 230 110 220 180 224S300 242 380 234S520 212 610 218S760 238 840 228S990 208 1080 220S1170 234 1210 228V268Z"
		/>
		<path class="far" d={farForest} />
		<path class="near" d={nearLeft + nearRight} />

		<!-- Kingdom -->
		<path class="front" d={castle + walls} />
		<path class="front" d="M-20 262H1220V268H-20Z" />
		<path class="gold" d="M600.7 146L624 149.5L617 154L624 158.5L600.7 160Z" />
		<g class="lit">
			<rect x="584" y="192" width="4" height="8" rx="2" />
			<rect x="612" y="192" width="4" height="8" rx="2" />
			<rect x="598" y="205" width="4" height="9" rx="2" />
			<rect x="544" y="216" width="4" height="7" rx="2" />
			<rect x="652" y="216" width="4" height="7" rx="2" />
		</g>
		<path class="gate" d="M592 268V257A8 8 0 0 1 608 257V268Z" />
		{#each [353, 847] as tx (tx)}
			<circle cx={tx} cy="224" r="11" fill="url(#{uid}-halo)" />
			<rect class="front" x={tx - 0.6} y="225" width="1.2" height="8" />
			<circle class="gold" cx={tx} cy="224.5" r="1.6" />
		{/each}

		<!-- Monarch on horseback, facing the castle -->
		<g transform="translate(428 268) scale(1.12)">
			<g class="front">
				<!-- horse -->
				<path
					d="M13 -20C14 -24 16 -28 19 -32.5C21 -32 25 -31 27.3 -30.2C28.8 -30 29 -32 28 -33C26 -35.5 24 -38 22 -41L21.2 -45L19.3 -41.6C16 -39 11 -33 6 -29C1 -27.5 -9 -28 -14 -28.6C-18 -28.6 -21 -25 -20.5 -20C-20 -17 -18 -15 -15 -15C-7 -14 5 -14 10 -15.5C12 -16.5 13 -18 13 -20Z"
				/>
				<path d="M-19 -26.5C-24 -25 -26 -19 -25.5 -12C-24.5 -15 -22.5 -19 -20 -22Z" />
				<!-- rider: cape, body, head -->
				<path d="M0.5 -40.2C-3 -39.5 -8 -35 -12.5 -30L-14 -27.6C-10 -28.6 -5 -29 -1.5 -29Z" />
				<path d="M-2 -29L4 -29L5 -38.5C5 -40 4 -40.5 2.5 -40.5C1 -40.5 -0.5 -40 -0.5 -38.5Z" />
				<circle cx="2.8" cy="-43.3" r="2.7" />
			</g>
			<g class="front-line">
				<path d="M11 -17L13.5 -9L15 -1" />
				<path d="M7 -16L7.5 -8L6.5 -1" />
				<path d="M-15 -18L-12.5 -12L-16 -7L-14.5 -1" />
				<path d="M-11 -16L-9.5 -11L-12 -6L-10.5 -1" />
				<path d="M0 -30L5 -26L4 -20" />
				<path d="M3.5 -37L7 -33L10 -32.5" stroke-width="1.6" />
				<path d="M10 -32.5L24 -32.5" stroke-width="0.5" />
			</g>
			<path class="gold" d="M0.3 -45.4L0.3 -49.6L1.6 -47.6L2.8 -50.6L4 -47.6L5.3 -49.6L5.3 -45.4Z" />
		</g>

		<!-- Coins dropped on the road -->
		<circle class="gold" cx="474" cy="265.8" r="2.1" />
		<circle class="gold" cx="485" cy="266" r="2.1" />
		<path class="sparkle twinkle" d="M474 254.5L475 258.5L479 259.5L475 260.5L474 264.5L473 260.5L469 259.5L473 258.5Z" />

		<path class="front" d={frontLeft + frontRight} />
	</g>

	<!-- River: mirrored land, sliced into drifting bands -->
	<rect y={GROUND} width={W} height={H - GROUND} fill="url(#{uid}-water)" />
	<g opacity="0.7">
		{#each bands as band, i (i)}
			<g clip-path="url(#{uid}-band{i})">
				<use href="#{uid}-land" transform="matrix(1 0 0 -1 {band.dx} {GROUND * 2})" />
			</g>
		{/each}
	</g>
	<rect class="ripple" y={GROUND} width={W} height="1" opacity="0.7" />
	<g class="ripple" opacity="0.45">
		{#each ripples as [x, y, w] (`${x}-${y}`)}
			<rect {x} {y} width={w} height="1" rx="0.5" />
		{/each}
	</g>
	<ellipse cx={MOON.x} cy="296" rx="80" ry="26" fill="url(#{uid}-halo)" />
	<g class="moon">
		{#each glints as [dx, y, w, o] (y)}
			<rect x={MOON.x + dx} {y} width={w} height="1" rx="0.5" opacity={o} />
		{/each}
	</g>
	<rect y="300" width={W} height={H - 300} fill="url(#{uid}-fade)" />
</svg>

<style>
	svg {
		--kh-dusk: color-mix(in oklch, var(--secondary) 60%, var(--background));
		--kh-glow: color-mix(in oklch, var(--primary) 26%, var(--secondary));
		--kh-horizon: color-mix(in oklch, var(--primary) 36%, var(--secondary));
		--kh-water: color-mix(in oklch, var(--kh-glow) 75%, var(--background));
		--kh-far: color-mix(in oklch, var(--muted) 85%, var(--background));
		--kh-hill: color-mix(in oklch, var(--kh-glow) 55%, var(--kh-far));
		--kh-near: color-mix(in oklch, var(--card) 50%, var(--background));
		--kh-front: color-mix(in oklch, var(--background) 80%, black);
		--kh-moon: color-mix(in oklch, var(--primary) 45%, var(--foreground));
	}

	.stop-bg {
		stop-color: var(--background);
	}
	.stop-dusk {
		stop-color: var(--kh-dusk);
	}
	.stop-glow {
		stop-color: var(--kh-glow);
	}
	.stop-horizon {
		stop-color: var(--kh-horizon);
	}
	.stop-water {
		stop-color: var(--kh-water);
	}
	.stop-gold {
		stop-color: var(--primary);
	}

	.star {
		fill: color-mix(in oklch, var(--foreground) 75%, var(--primary));
	}
	.moon {
		fill: var(--kh-moon);
	}
	.crater {
		fill: color-mix(in oklch, var(--primary) 30%, var(--kh-moon));
		opacity: 0.5;
	}
	.hill {
		fill: var(--kh-hill);
	}
	.far {
		fill: var(--kh-far);
	}
	.near {
		fill: var(--kh-near);
	}
	.front {
		fill: var(--kh-front);
	}
	.front-line {
		fill: none;
		stroke: var(--kh-front);
		stroke-width: 2.4;
		stroke-linecap: round;
		stroke-linejoin: round;
	}
	.gold {
		fill: var(--primary);
	}
	.lit {
		fill: color-mix(in oklch, var(--primary) 65%, var(--background));
	}
	.gate {
		fill: color-mix(in oklch, var(--primary) 16%, var(--background));
	}
	.sparkle {
		fill: color-mix(in oklch, var(--primary) 50%, var(--foreground));
	}
	.ripple {
		fill: color-mix(in oklch, var(--primary) 22%, var(--muted));
	}

	@media (prefers-reduced-motion: no-preference) {
		.twinkle {
			animation: twinkle 5s ease-in-out infinite;
		}
	}

	@keyframes twinkle {
		50% {
			opacity: 0.2;
		}
	}
</style>
