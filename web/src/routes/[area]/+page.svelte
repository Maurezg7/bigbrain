<script lang="ts">
	import TopicCard from '#lib/components/TopicCard.svelte';
	import { normalize } from '#lib/api.ts';
	import { groupByBranch } from '#lib/groups.ts';
	import { progress } from '#lib/progress.svelte.ts';
	import type { PageProps } from './$types';

	const LEVELS = ['Básico', 'Intermedio', 'Avanzado'];

	let { data }: PageProps = $props();
	let area = $derived(data.area);
	let filter = $state('');
	let level = $state('');

	let groups = $derived.by(() => {
		const terms = normalize(filter).split(/\s+/).filter(Boolean);
		return groupByBranch(area)
			.map((g) => ({
				...g,
				id: `rama-${normalize(g.branch).replace(/[^a-z0-9]+/g, '-')}`,
				topics: g.topics.filter((t) => {
					if (level && t.level !== level) return false;
					const text = normalize(`${t.title} ${t.summary} ${t.branch} ${t.tags.join(' ')}`);
					return terms.every((term) => text.includes(term));
				})
			}))
			.filter((g) => g.topics.length);
	});
	let shown = $derived(groups.reduce((n, g) => n + g.topics.length, 0));
	let read = $derived(progress.count(area.slug));
</script>

<svelte:head>
	<title>{area.title} · BigBrain</title>
	<meta name="description" content={area.description} />
</svelte:head>

<div class="container" style:--area={area.color}>
	<header>
		<div class="symbol" aria-hidden="true">{area.symbol}</div>
		<div>
			<p class="eyebrow">{area.topic_count} temas · {area.branches.length} ramas</p>
			<h1>{area.title}</h1>
			<p class="lead">{area.description}</p>
			{#if read}
				<div class="bar" role="img" aria-label="{read} de {area.topic_count} temas leídos">
					<div style:width="{(100 * read) / area.topic_count}%"></div>
				</div>
				<p class="read">{read} de {area.topic_count} temas leídos</p>
			{/if}
		</div>
	</header>

	<div class="tools">
		<input
			type="search"
			bind:value={filter}
			placeholder="Filtrar temas de {area.title}…"
			aria-label="Filtrar temas"
		/>
		<div class="levels" role="group" aria-label="Nivel">
			<button type="button" aria-pressed={level === ''} onclick={() => (level = '')}>Todos</button>
			{#each LEVELS as l (l)}
				<button type="button" aria-pressed={level === l} onclick={() => (level = l)}>{l}</button>
			{/each}
		</div>
	</div>

	{#if groups.length > 1}
		<nav class="branches" aria-label="Ramas">
			{#each groups as group (group.branch)}
				<a href="#{group.id}">{group.branch} <small>{group.topics.length}</small></a>
			{/each}
		</nav>
	{/if}

	{#if filter || level}
		<p class="count" aria-live="polite">
			{shown}
			{shown === 1 ? 'tema coincide' : 'temas coinciden'}
		</p>
	{/if}

	{#each groups as group (group.branch)}
		<section id={group.id}>
			<h2>{group.branch} <small>{group.topics.length}</small></h2>
			<div class="grid">
				{#each group.topics as topic (topic.slug)}
					<TopicCard {topic} />
				{/each}
			</div>
		</section>
	{:else}
		<p class="empty">
			Ningún tema coincide con el filtro.
			<button
				type="button"
				onclick={() => {
					filter = '';
					level = '';
				}}>Quitar filtros</button
			>
		</p>
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
	.bar {
		max-width: 22rem;
		height: 6px;
		margin-top: 1rem;
		overflow: hidden;
		background: var(--border);
		border-radius: 999px;
	}
	.bar div {
		height: 100%;
		background: var(--area);
		border-radius: 999px;
	}
	.read {
		margin: 0.4rem 0 0;
		color: var(--text-faint);
		font-size: 0.85rem;
	}
	.tools {
		display: flex;
		flex-wrap: wrap;
		gap: 0.8rem;
		margin-top: 1.5rem;
	}
	input {
		flex: 1 1 16rem;
		max-width: 28rem;
		padding: 0.55rem 1rem;
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: 999px;
		color: var(--text);
		font: inherit;
		font-size: 0.95rem;
	}
	input:focus {
		border-color: var(--area);
		outline: 0;
	}
	.levels {
		display: flex;
		padding: 3px;
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: 999px;
	}
	.levels button {
		padding: 0.3rem 0.8rem;
		background: none;
		border: 0;
		border-radius: 999px;
		color: var(--text-dim);
		font: inherit;
		font-size: 0.85rem;
		cursor: pointer;
	}
	.levels button[aria-pressed='true'] {
		background: color-mix(in srgb, var(--area) 18%, transparent);
		color: var(--area);
		font-weight: 600;
	}
	.branches {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
		margin-top: 1rem;
	}
	.branches a {
		padding: 0.25rem 0.7rem;
		border: 1px solid var(--border);
		border-radius: 999px;
		color: var(--text-dim);
		font-size: 0.82rem;
	}
	.branches a:hover {
		border-color: var(--area);
		color: var(--area);
	}
	small {
		color: var(--text-faint);
		font-size: 0.8em;
		font-weight: 400;
	}
	.count,
	.empty {
		margin: 1.2rem 0 0;
		color: var(--text-dim);
	}
	.empty button {
		padding: 0;
		background: none;
		border: 0;
		color: var(--area);
		font: inherit;
		text-decoration: underline;
		cursor: pointer;
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
