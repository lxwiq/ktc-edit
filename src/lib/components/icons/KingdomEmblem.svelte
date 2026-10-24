<script lang="ts">
	import type { SVGAttributes } from "svelte/elements";

	let { class: className, ...restProps }: SVGAttributes<SVGSVGElement> = $props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	width="64"
	height="64"
	viewBox="0 0 64 64"
	class={className}
	aria-hidden="true"
	{...restProps}
>
	<!-- Shield, with a subtle shade on its right half -->
	<path
		class="fill-primary"
		d="M14.5 26.5h35a2 2 0 0 1 2 2v10c0 11.5-8 19-19.5 23.5-11.5-4.5-19.5-12-19.5-23.5v-10a2 2 0 0 1 2-2z"
	/>
	<path class="fill-primary-foreground/15" d="M32 26.5h17.5a2 2 0 0 1 2 2v10c0 11.5-8 19-19.5 23.5z" />
	<!-- Tower with gate -->
	<path
		class="fill-primary-foreground"
		d="M23.5 33h4v3H30v-3h4v3h2.5v-3h4v19h-6v-4.5a2.5 2.5 0 0 0-5 0V52h-6z"
	/>
	<!-- Crown -->
	<path
		class="fill-primary stroke-primary"
		stroke-width="3"
		stroke-linejoin="round"
		d="M16 21 13 9.5l9.5 6L32 6l9.5 9.5L51 9.5 48 21z"
	/>
	<circle class="fill-primary" cx="13" cy="7.5" r="2.5" />
	<circle class="fill-primary" cx="32" cy="4.5" r="2.5" />
	<circle class="fill-primary" cx="51" cy="7.5" r="2.5" />
	<path class="stroke-primary-foreground" fill="none" stroke-width="2" stroke-linecap="round" d="M18 17h28" />
</svg>
