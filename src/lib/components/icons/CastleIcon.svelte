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
	<path d="M5 21V8h3v2h2V8h4v2h2V8h3v13" />
	<path d="M10 21v-4a2 2 0 0 1 4 0v4" />
	<path d="M3 21h18" />
	<path d="M12 8V2.5l3.5 1.5L12 5.5" />
</svg>
