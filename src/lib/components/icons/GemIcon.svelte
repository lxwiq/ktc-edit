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
	<path d="M8 2h8l4 4v12l-4 4H8l-4-4V6z" />
	<path d="M9 7h6v10H9z" />
	<path d="m6 4 3 3m6 0 3-3m0 16-3-3m-6 0-3 3" />
</svg>
