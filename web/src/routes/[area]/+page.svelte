<script lang="ts">
	import TopicCard from '#lib/components/TopicCard.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let area = $derived(data.area);
	let groups = $derived(
		[...new Set([...area.branches, ...area.topics.map((t) => t.branch)])]
			.map((branch) => ({ branch, topics: area.topics.filter((t) => t.branch === branch) }))
			.filter((g) => g.topics.length)
	);
</script>

<svelte:head>
	<title>{area.title} · BigBrain</title>
	<meta name="description" content={area.description} />
</svelte:head>

<div class="container" style:--area={area.color}>
	<header>
		<div class="symbol" aria-hidden="true">{area.symbol}</div>
		<div>
			<p class="eyebrow">{area.topic_count} temas</p>
			<h1>{area.title}</h1>
			<p class="lead">{area.description}</p>
		</div>
	</header>

	{#each groups as group (group.branch)}
		<section>
			<h2>{group.branch}</h2>
			<div class="grid">
				{#each group.topics as topic (topic.slug)}
					<TopicCard {topic} />
				{/each}
			</div>
		</section>
	{/each}
</div>

<style>
	header {
		display: flex;
		gap: 1.2rem;
		align-items: flex-start;
		padding: 3rem 0 1rem;
	}
	.symbol {
		display: grid;
		flex: none;
		place-items: center;
		width: 4rem;
		height: 4rem;
		border-radius: 16px;
		background: color-mix(in srgb, var(--area) 16%, transparent);
		color: var(--area);
		font-size: 2rem;
		font-weight: 700;
	}
	h1 {
		margin: 0.2rem 0 0.4rem;
		font-size: clamp(1.9rem, 4vw, 2.6rem);
	}
	.lead {
		max-width: 65ch;
		margin: 0;
		color: var(--text-dim);
	}
	section {
		margin-top: 2.2rem;
	}
	h2 {
		display: flex;
		align-items: center;
		gap: 0.6rem;
		margin: 0 0 1rem;
		font-size: 1.2rem;
	}
	h2::before {
		content: '';
		width: 4px;
		height: 1.1em;
		border-radius: 2px;
		background: var(--area);
	}
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(290px, 1fr));
		gap: 1rem;
	}
</style>
