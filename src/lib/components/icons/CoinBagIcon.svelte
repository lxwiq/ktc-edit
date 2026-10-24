<script lang="ts">
	import type { SVGAttributes } from "svelte/elements";

	let { class: className, ...restProps }: SVGAttributes<SVGSVGElement> = $props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	width="24"
	height="24"
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width="2"
	stroke-linecap="round"
	stroke-linejoin="round"
	class={className}
	aria-hidden="true"
	{...restProps}
>
	<path d="M9.5 7.5C6 9.5 4 13 4 16.5A4.5 4.5 0 0 0 8.5 21h7a4.5 4.5 0 0 0 4.5-4.5c0-3.5-2-7-5.5-9" />
	<path d="M9.5 7.5 7.5 3c1.5.8 3 .8 4.5 0 1.5.8 3 .8 4.5 0l-2 4.5z" />
	<circle cx="12" cy="15" r="2.5" />
</svg>
