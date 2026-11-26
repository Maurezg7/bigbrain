<script lang="ts">
	import SearchBox from '#lib/components/SearchBox.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let total = $derived(data.areas.reduce((n, a) => n + a.topic_count, 0));
</script>

<svelte:head>
	<title>BigBrain · Todo el conocimiento universitario, bien explicado</title>
	<meta
		name="description"
		content="Matemáticas, física, química, ingeniería electromecánica, software, informática e idiomas explicados paso a paso."
	/>
</svelte:head>

<section class="hero container">
	<p class="eyebrow">Enciclopedia universitaria</p>
	<h1>Todo lo que necesitás aprender, <span>explicado de verdad</span>.</h1>
	<p class="lead">
		Matemáticas, física, química, ingeniería, software, informática e idiomas: teoría clara,
		fórmulas, figuras y ejemplos resueltos para cualquier carrera.
	</p>
	<div class="search"><SearchBox large /></div>
	<p class="stats">{data.areas.length} áreas · {total} temas</p>
</section>

<section class="container">
	<h2>Áreas de estudio</h2>
	<div class="grid">
		{#each data.areas as area (area.slug)}
			<a class="area" href="/{area.slug}" style:--area={area.color}>
				<div class="symbol" aria-hidden="true">{area.symbol}</div>
				<h3>{area.title}</h3>
				<p>{area.description}</p>
				<ul>
					{#each area.branches as branch (branch)}
						<li class="chip">{branch}</li>
					{/each}
				</ul>
				<span class="count">{area.topic_count} temas →</span>
			</a>
		{/each}
	</div>
</section>

<style>
	.hero {
		padding: 5rem 0 3.5rem;
		text-align: center;
		background: radial-gradient(
			60% 70% at 50% 0%,
			color-mix(in srgb, var(--accent) 18%, transparent),
			transparent
		);
	}
	h1 {
		max-width: 18ch;
		margin: 0.6rem auto 1rem;
		font-size: clamp(2.2rem, 6vw, 3.8rem);
		font-weight: 800;
	}
	h1 span {
		background: linear-gradient(90deg, var(--accent), var(--accent-2));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}
	.lead {
		max-width: 58ch;
		margin: 0 auto 2rem;
		color: var(--text-dim);
		font-size: 1.1rem;
	}
	.search {
		max-width: 36rem;
		margin: 0 auto;
	}
	.stats {
		margin: 1rem 0 0;
		color: var(--text-faint);
		font-size: 0.85rem;
	}
	h2 {
		margin: 0 0 1.2rem;
		font-size: 1.4rem;
	}
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1rem;
	}
	.area {
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
		padding: 1.4rem;
		background: linear-gradient(
			160deg,
			color-mix(in srgb, var(--area) 9%, var(--bg-card)),
			var(--bg-card) 55%
		);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		transition:
			border-color 0.15s,
			transform 0.15s;
	}
	.area:hover {
		border-color: var(--area);
		transform: translateY(-3px);
	}
	.symbol {
		display: grid;
		place-items: center;
		width: 2.8rem;
		height: 2.8rem;
		border-radius: 12px;
		background: color-mix(in srgb, var(--area) 16%, transparent);
		color: var(--area);
		font-size: 1.4rem;
		font-weight: 700;
	}
	h3 {
		margin: 0.2rem 0 0;
		font-size: 1.25rem;
	}
	.area p {
		margin: 0;
		color: var(--text-dim);
		font-size: 0.93rem;
	}
	ul {
		display: flex;
		flex: 1;
		flex-wrap: wrap;
		align-content: flex-start;
		gap: 0.35rem;
		margin: 0.2rem 0;
		padding: 0;
		list-style: none;
	}
	.count {
		color: var(--area);
		font-size: 0.88rem;
		font-weight: 600;
	}
</style>
