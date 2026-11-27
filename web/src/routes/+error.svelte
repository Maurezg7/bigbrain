<script lang="ts">
	import { page } from '$app/state';
</script>

<section class="container">
	<p class="eyebrow">Error {page.status}</p>
	<h1>{page.error?.message}</h1>
	<a href="/">Volver al inicio</a>
</section>

<style>
	section {
		padding: 6rem 0;
		text-align: center;
	}
	a {
		color: var(--accent);
	}
</style>
