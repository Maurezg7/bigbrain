<script lang="ts">
	import SearchBox from '#lib/components/SearchBox.svelte';
	import TopicCard from '#lib/components/TopicCard.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let areas = $derived(new Map(data.areas.map((a) => [a.slug, a])));
</script>

<svelte:head>
	<title>{data.q ? `"${data.q}"` : 'Buscar'} · BigBrain</title>
</svelte:head>

<div class="container">
	<h1>Buscar</h1>
	{#key data.q}
		<div class="search"><SearchBox large value={data.q} /></div>
	{/key}

	{#if data.q}
		<p class="count">
			{data.results.length}
			{data.results.length === 1 ? 'resultado' : 'resultados'} para «{data.q}»
		</p>
		<div class="grid">
			{#each data.results as topic (topic.area + topic.slug)}
				<div style:--area={areas.get(topic.area)?.color}>
					<TopicCard {topic} areaTitle={areas.get(topic.area)?.title} />
				</div>
			{/each}
		</div>
		{#if !data.results.length}
			<p class="empty">Probá con otra palabra o recorré las áreas desde el inicio.</p>
		{/if}
	{/if}
</div>

<style>
	h1 {
		margin: 3rem 0 1rem;
	}
	.search {
		max-width: 40rem;
	}
	.count,
	.empty {
		color: var(--text-dim);
	}
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(290px, 1fr));
		gap: 1rem;
	}
	.grid > div {
		display: grid;
	}
</style>
