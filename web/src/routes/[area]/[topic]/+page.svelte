<script lang="ts">
	import katex from 'katex';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let topic = $derived(data.topic);

	// Las fórmulas llegan del servidor como LaTeX dentro de <span class="math">.
	function renderMath(node: HTMLElement) {
		for (const el of node.querySelectorAll<HTMLElement>('.math')) {
			katex.render(el.textContent ?? '', el, {
				displayMode: el.classList.contains('math-display'),
				throwOnError: false
			});
		}
	}
</script>

<svelte:head>
	<title>{topic.title} · {topic.area_title} · BigBrain</title>
	<meta name="description" content={topic.summary} />
</svelte:head>

<div class="container layout" style:--area={topic.color}>
	<article>
		<nav class="crumbs" aria-label="Ruta">
			<a href="/">Inicio</a> / <a href="/{topic.area}">{topic.area_title}</a> /
			<span>{topic.branch}</span>
		</nav>
		<h1>{topic.title}</h1>
		<p class="lead">{topic.summary}</p>
		<div class="meta">
			<span class="chip level">{topic.level}</span>
			<span class="chip">{topic.minutes} min de lectura</span>
			{#each topic.tags as tag (tag)}
				<span class="chip">{tag}</span>
			{/each}
		</div>

		{#key topic.area + topic.slug}
			<div class="prose" {@attach renderMath}>
				{@html topic.html}
			</div>
		{/key}

		<nav class="pager" aria-label="Temas vecinos">
			{#if topic.prev}
				<a href="/{topic.prev.area}/{topic.prev.slug}">
					<small>← Anterior</small>
					{topic.prev.title}
				</a>
			{:else}
				<span></span>
			{/if}
			{#if topic.next}
				<a class="next" href="/{topic.next.area}/{topic.next.slug}">
					<small>Siguiente →</small>
					{topic.next.title}
				</a>
			{/if}
		</nav>
	</article>

	{#if topic.headings.length}
		<aside>
			<p class="eyebrow">En esta página</p>
			<ul>
				{#each topic.headings as h (h.id)}
					<li><a href="#{h.id}">{h.text}</a></li>
				{/each}
			</ul>
		</aside>
	{/if}
</div>

<style>
	.layout {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 15rem;
		gap: 3rem;
		padding-top: 2.5rem;
	}
	article {
		max-width: 50rem;
	}
	.crumbs {
		color: var(--text-faint);
		font-size: 0.85rem;
	}
	.crumbs a:hover {
		color: var(--area);
	}
	h1 {
		margin: 0.8rem 0 0.6rem;
		font-size: clamp(2rem, 5vw, 2.8rem);
	}
	.lead {
		margin: 0 0 1rem;
		color: var(--text-dim);
		font-size: 1.15rem;
	}
	.meta {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid var(--border);
	}
	.level {
		border-color: var(--area);
		color: var(--area);
	}
	aside {
		position: sticky;
		top: 5.5rem;
		align-self: start;
		max-height: calc(100vh - 7rem);
		overflow-y: auto;
		padding-left: 1rem;
		border-left: 1px solid var(--border);
		font-size: 0.88rem;
	}
	aside ul {
		margin: 0.6rem 0 0;
		padding: 0;
		list-style: none;
	}
	aside li a {
		display: block;
		padding: 0.25rem 0;
		color: var(--text-dim);
	}
	aside li a:hover {
		color: var(--area);
	}
	.pager {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		margin-top: 3rem;
	}
	.pager a {
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
		padding: 0.9rem 1.1rem;
		background: var(--bg-card);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		font-weight: 600;
	}
	.pager a:hover {
		border-color: var(--area);
	}
	.pager .next {
		text-align: right;
	}
	small {
		color: var(--text-faint);
		font-weight: 400;
	}

	.prose {
		font-size: 1.05rem;
		line-height: 1.75;
	}
	.prose :global(h2) {
		margin: 2.4rem 0 0.8rem;
		padding-top: 0.4rem;
		font-size: 1.55rem;
	}
	.prose :global(h3) {
		margin: 1.8rem 0 0.5rem;
		color: var(--area);
		font-size: 1.15rem;
	}
	.prose :global(a) {
		color: var(--area);
		text-decoration: underline;
		text-underline-offset: 3px;
	}
	.prose :global(strong) {
		color: #fff;
	}
	.prose :global(li) {
		margin: 0.3rem 0;
	}
	.prose :global(img) {
		display: block;
		width: 100%;
		max-width: 640px;
		height: auto;
		margin: 1.6rem auto 0.4rem;
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius);
	}
	.prose :global(p:has(> img) + p > em:only-child) {
		display: block;
		color: var(--text-faint);
		font-size: 0.88rem;
		text-align: center;
	}
	.prose :global(code) {
		padding: 0.12em 0.4em;
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: 6px;
		font-family: var(--mono);
		font-size: 0.88em;
	}
	.prose :global(pre) {
		overflow-x: auto;
		padding: 1rem 1.2rem;
		background: #0e111a;
		border: 1px solid var(--border);
		border-radius: var(--radius);
		line-height: 1.55;
	}
	.prose :global(pre code) {
		padding: 0;
		background: none;
		border: 0;
		color: #c8d3f5;
	}
	.prose :global(.math-display) {
		display: block;
		overflow-x: auto;
		margin: 1.2rem 0;
		padding: 0.4rem 0;
	}
	.prose :global(table) {
		display: block;
		overflow-x: auto;
		width: 100%;
		margin: 1.4rem 0;
		border-collapse: collapse;
		font-size: 0.95rem;
	}
	.prose :global(th),
	.prose :global(td) {
		padding: 0.55rem 0.9rem;
		border: 1px solid var(--border);
		text-align: left;
	}
	.prose :global(th) {
		background: var(--bg-raised);
		color: var(--area);
	}
	.prose :global(blockquote) {
		margin: 1.4rem 0;
		padding: 0.2rem 1.2rem;
		background: color-mix(in srgb, var(--area) 8%, var(--bg-card));
		border-left: 3px solid var(--area);
		border-radius: 0 var(--radius) var(--radius) 0;
	}
	.prose :global(.markdown-alert-tip) {
		--area: #4ade80;
	}
	.prose :global(.markdown-alert-warning) {
		--area: #fbbf24;
	}
	.prose :global(.markdown-alert-important) {
		--area: #f472b6;
	}
	.prose :global(blockquote::before) {
		display: block;
		padding-top: 0.8rem;
		color: var(--area);
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}
	.prose :global(.markdown-alert-note::before) {
		content: 'Ejemplo resuelto';
	}
	.prose :global(.markdown-alert-tip::before) {
		content: 'Idea clave';
	}
	.prose :global(.markdown-alert-warning::before) {
		content: 'Error frecuente';
	}
	.prose :global(.markdown-alert-important::before) {
		content: 'Definición';
	}

	@media (max-width: 980px) {
		.layout {
			grid-template-columns: minmax(0, 1fr);
		}
		aside {
			display: none;
		}
	}
</style>
