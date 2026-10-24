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
	<path d="M3 21V5h4v3h3V5h4v3h3V5h4v16z" />
	<path d="M3 11.5h18M3 17.5h18" />
	<path d="M7.5 14.5h.01M12 14.5h.01M16.5 14.5h.01" />
</svg>
