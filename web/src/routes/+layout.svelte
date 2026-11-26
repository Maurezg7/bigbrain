<script lang="ts">
	import '../app.css';
	import 'katex/dist/katex.min.css';
	import favicon from '#lib/assets/favicon.svg';
	import SearchBox from '#lib/components/SearchBox.svelte';
	import { page } from '$app/state';
	import type { LayoutProps } from './$types';

	let { children, data }: LayoutProps = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<header>
	<div class="container bar">
		<a class="brand" href="/" aria-label="BigBrain, inicio">
			<img src={favicon} alt="" width="30" height="30" />
			<span>Big<b>Brain</b></span>
		</a>
		<nav aria-label="Áreas">
			{#each data.areas as area (area.slug)}
				<a
					href="/{area.slug}"
					style:--area={area.color}
					aria-current={page.params.area === area.slug ? 'page' : undefined}
				>
					{area.title}
				</a>
			{/each}
		</nav>
		{#if page.url.pathname !== '/'}
			<div class="search"><SearchBox /></div>
		{/if}
	</div>
</header>

<main>
	{@render children()}
</main>

<footer>
	<div class="container">
		<span>Big<b>Brain</b> · conocimiento universitario abierto</span>
		<span>Hecho con Rust y SvelteKit</span>
	</div>
</footer>

<style>
	header {
		position: sticky;
		top: 0;
		z-index: 10;
		background: color-mix(in srgb, var(--bg) 85%, transparent);
		backdrop-filter: blur(12px);
		border-bottom: 1px solid var(--border);
	}
	.bar {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		min-height: 3.75rem;
	}
	.brand {
		display: flex;
		align-items: center;
		gap: 0.55rem;
		font-size: 1.2rem;
		font-weight: 500;
		letter-spacing: -0.02em;
	}
	b {
		color: var(--accent);
		font-weight: 800;
	}
	nav {
		display: flex;
		flex: 1;
		gap: 0.25rem;
		overflow-x: auto;
		scrollbar-width: none;
	}
	nav a {
		padding: 0.35rem 0.65rem;
		border-radius: 8px;
		color: var(--text-dim);
		font-size: 0.88rem;
		white-space: nowrap;
	}
	nav a:hover {
		color: var(--text);
		background: var(--bg-raised);
	}
	nav a[aria-current='page'] {
		color: var(--area);
		background: color-mix(in srgb, var(--area) 12%, transparent);
	}
	.search {
		width: 17rem;
		font-size: 0.88rem;
	}
	main {
		min-height: calc(100vh - 9rem);
	}
	footer {
		margin-top: 4rem;
		padding: 1.5rem 0;
		border-top: 1px solid var(--border);
		color: var(--text-faint);
		font-size: 0.85rem;
	}
	footer .container {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		gap: 0.5rem;
	}
	@media (max-width: 900px) {
		.bar {
			flex-wrap: wrap;
			gap: 0.4rem 1rem;
			padding: 0.6rem 0;
		}
		nav {
			order: 3;
			flex-basis: 100%;
		}
		.search {
			flex: 1;
			width: auto;
		}
	}
</style>
