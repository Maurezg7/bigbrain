<script lang="ts">
	import type { TopicSummary } from '#lib/api.ts';
	import { progress } from '#lib/progress.svelte.ts';

	let { topic, areaTitle = '' }: { topic: TopicSummary; areaTitle?: string } = $props();
</script>

<a class="card" href="/{topic.area}/{encodeURIComponent(topic.slug)}">
	<div class="meta">
		<span>{areaTitle ? `${areaTitle} · ${topic.branch}` : topic.branch}</span>
		<span>
			{#if progress.has(topic.area, topic.slug)}<b>✓ Leído · </b>{/if}{topic.minutes} min
		</span>
	</div>
	<h3>{topic.title}</h3>
	<p>{topic.summary}</p>
	<div class="foot">
		<span class="chip level">{topic.level}</span>
		{#each topic.tags.slice(0, 3) as tag (tag)}
			<span class="chip">{tag}</span>
		{/each}
	</div>
</a>

<style>
	.card {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 1.1rem 1.2rem;
		background: var(--bg-card);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		transition:
			border-color 0.15s,
			transform 0.15s;
	}
	.card:hover {
		border-color: var(--area, var(--accent));
		transform: translateY(-2px);
	}
	.meta {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		color: var(--text-faint);
		font-size: 0.75rem;
	}
	b {
		color: #4ade80;
		font-weight: 600;
	}
	h3 {
		margin: 0;
		font-size: 1.1rem;
	}
	p {
		flex: 1;
		margin: 0;
		color: var(--text-dim);
		font-size: 0.92rem;
	}
	.foot {
		display: flex;
		flex-wrap: wrap;
		gap: 0.35rem;
		margin-top: 0.3rem;
	}
	.level {
		border-color: color-mix(in srgb, var(--area, var(--accent)) 60%, transparent);
		color: var(--area, var(--accent));
	}
</style>
