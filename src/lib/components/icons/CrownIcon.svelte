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
	<path d="M4 20V8l4 4 4-6 4 6 4-4v12Z" />
	<path d="M4 15h16" />
	<circle cx="8" cy="17.5" r="1" fill="currentColor" stroke="none" />
	<circle cx="12" cy="17.5" r="1" fill="currentColor" stroke="none" />
	<circle cx="16" cy="17.5" r="1" fill="currentColor" stroke="none" />
</svg>
