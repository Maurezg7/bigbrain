<script lang="ts">
	import katex from 'katex';
	import { enhanceCode } from '#lib/highlight.ts';
	import { groupByBranch } from '#lib/groups.ts';
	import { progress } from '#lib/progress.svelte.ts';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let topic = $derived(data.topic);
	let groups = $derived(groupByBranch(data.area));
	let done = $derived(progress.has(topic.area, topic.slug));

	let scrollY = $state(0);
	let innerHeight = $state(0);
	let prose = $state<HTMLElement>();
	let read = $state(0);
	let active = $state('');
	let hash = $state('');

	// Avance de lectura y sección visible, recalculados al desplazarse.
	$effect(() => {
		void scrollY;
		void hash;
		if (!prose) return;
		const box = prose.getBoundingClientRect();
		const span = box.height - innerHeight * 0.6;
		read = span > 0 ? Math.min(1, Math.max(0, -box.top / span)) : 1;
		const headings = [...prose.querySelectorAll<HTMLElement>('h2[id]')];
		let current = '';
		for (const h of headings) {
			if (h.getBoundingClientRect().top > 130) break;
			current = h.id;
		}
		// Al pie de la página las últimas secciones no llegan al umbral superior:
		// se marca la elegida en el índice si está a la vista, o si no la última.
		const atBottom = scrollY + innerHeight >= document.documentElement.scrollHeight - 2;
		if (atBottom && scrollY > 0 && headings.length) {
			const visible = headings.filter((h) => {
				const top = h.getBoundingClientRect().top;
				return top > 130 && top < innerHeight;
			});
			const target = decodeURIComponent(location.hash.slice(1));
			const chosen = visible.find((h) => h.id === target) ?? visible.at(-1);
			if (chosen) current = chosen.id;
		}
		active = current;
	});

	// El servidor entrega HTML con las fórmulas como LaTeX dentro de <span class="math">.
	function enhance(node: HTMLElement) {
		for (const el of node.querySelectorAll<HTMLElement>('.math')) {
			katex.render(el.textContent ?? '', el, {
				displayMode: el.classList.contains('math-display'),
				throwOnError: false,
				strict: 'ignore'
			});
		}
		enhanceCode(node);
		for (const table of node.querySelectorAll('table')) {
			const wrap = document.createElement('div');
			wrap.className = 'table-wrap';
			table.replaceWith(wrap);
			wrap.append(table);
		}
		for (const h of node.querySelectorAll<HTMLElement>('h2[id], h3[id]')) {
			const link = document.createElement('a');
			link.className = 'anchor';
			link.href = `#${h.id}`;
			link.textContent = '#';
			link.setAttribute('aria-label', `Enlace a «${h.textContent}»`);
			h.append(link);
		}
		prose = node;
	}
</script>

<svelte:window bind:scrollY bind:innerHeight onhashchange={() => (hash = location.hash)} />

<svelte:head>
	<title>{topic.title} · {topic.area_title} · BigBrain</title>
	<meta name="description" content={topic.summary} />
</svelte:head>

{#snippet areaNav()}
	{#each groups as group (group.branch)}
		<details open={group.branch === topic.branch}>
			<summary>{group.branch} <small>{group.topics.length}</small></summary>
			<ul>
				{#each group.topics as t (t.slug)}
					<li>
						<a
							href="/{t.area}/{encodeURIComponent(t.slug)}"
							aria-current={t.slug === topic.slug ? 'page' : undefined}
							class:done={progress.has(t.area, t.slug)}
						>
							{t.title}
						</a>
					</li>
				{/each}
			</ul>
		</details>
	{/each}
{/snippet}

{#snippet toc()}
	<ul>
		{#each topic.headings as h (h.id)}
			<li><a href="#{h.id}" class:active={active === h.id}>{h.text}</a></li>
		{/each}
	</ul>
{/snippet}

<div class="progress" style:--area={topic.color} aria-hidden="true">
	<div style:transform="scaleX({read})"></div>
</div>

<div class="layout" style:--area={topic.color}>
	<nav class="side" aria-label="Temas de {topic.area_title}">
		<a class="area-link" href="/{topic.area}">{topic.area_title}</a>
		{#key topic.branch}
			{@render areaNav()}
		{/key}
	</nav>

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
			{#if done}<span class="chip read">✓ Leído</span>{/if}
			{#each topic.tags as tag (tag)}
				<a class="chip" href="/buscar?q={encodeURIComponent(tag)}">{tag}</a>
			{/each}
		</div>

		<div class="fold">
			<details>
				<summary>Temas de {topic.area_title}</summary>
				<div class="side-inner">
					{#key topic.branch}
						{@render areaNav()}
					{/key}
				</div>
			</details>
			{#if topic.headings.length}
				<details>
					<summary>En esta página</summary>
					{@render toc()}
				</details>
			{/if}
		</div>

		{#key topic.area + topic.slug}
			<div class="prose" {@attach enhance}>
				{@html topic.html}
			</div>
		{/key}

		<div class="finish">
			<button
				type="button"
				class:done
				aria-pressed={done}
				onclick={() => progress.toggle(topic.area, topic.slug)}
			>
				{done ? '✓ Tema leído' : 'Marcar como leído'}
			</button>
			<span>Tu avance se guarda solo en este navegador.</span>
		</div>

		<nav class="pager" aria-label="Temas vecinos">
			{#if topic.prev}
				<a href="/{topic.prev.area}/{encodeURIComponent(topic.prev.slug)}">
					<small>← Anterior</small>
					{topic.prev.title}
				</a>
			{:else}
				<span></span>
			{/if}
			{#if topic.next}
				<a class="next" href="/{topic.next.area}/{encodeURIComponent(topic.next.slug)}">
					<small>Siguiente →</small>
					{topic.next.title}
				</a>
			{/if}
		</nav>
	</article>

	{#if topic.headings.length}
		<aside>
			<p class="eyebrow">En esta página</p>
			{@render toc()}
		</aside>
	{/if}
</div>

{#if scrollY > 900}
	<button
		type="button"
		class="top"
		aria-label="Volver arriba"
		onclick={() => window.scrollTo({ top: 0 })}
	>
		↑
	</button>
{/if}

<style>
	.progress {
		position: fixed;
		inset: 0 0 auto;
		z-index: 20;
		height: 3px;
	}
	.progress div {
		height: 100%;
		background: linear-gradient(90deg, var(--area), var(--accent-2));
		transform-origin: left;
	}
	.layout {
		display: grid;
		grid-template-columns: 16rem minmax(0, 50rem) 14rem;
		justify-content: center;
		gap: 2.5rem;
		width: min(1440px, 100% - 2.5rem);
		margin-inline: auto;
		padding-top: 2.5rem;
	}
	.side,
	aside {
		position: sticky;
		top: 5.5rem;
		align-self: start;
		max-height: calc(100vh - 7rem);
		overflow-y: auto;
		font-size: 0.88rem;
		scrollbar-width: thin;
		scrollbar-color: var(--border) transparent;
	}
	.area-link {
		display: block;
		margin-bottom: 0.6rem;
		color: var(--area);
		font-weight: 700;
	}
	.side details,
	.side-inner details {
		border-top: 1px solid var(--border);
	}
	summary {
		display: flex;
		justify-content: space-between;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: var(--text);
		font-weight: 600;
		cursor: pointer;
		list-style: none;
	}
	summary::-webkit-details-marker {
		display: none;
	}
	summary:hover {
		color: var(--area);
	}
	summary small {
		color: var(--text-faint);
	}
	.side ul,
	.side-inner ul {
		margin: 0 0 0.6rem;
		padding: 0;
		list-style: none;
	}
	.side li a,
	.side-inner li a {
		display: block;
		padding: 0.28rem 0.6rem;
		border-left: 2px solid var(--border);
		color: var(--text-dim);
	}
	.side li a:hover,
	.side-inner li a:hover {
		color: var(--text);
	}
	li a.done::after {
		content: ' ✓';
		color: #4ade80;
	}
	li a[aria-current='page'] {
		border-left-color: var(--area);
		background: color-mix(in srgb, var(--area) 10%, transparent);
		color: var(--area);
		font-weight: 600;
	}
	.fold {
		display: none;
		margin-top: 1rem;
	}
	.fold > details {
		margin-bottom: 0.6rem;
		padding: 0 1rem;
		background: var(--bg-card);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		font-size: 0.92rem;
	}
	.fold > details > summary::after {
		content: '▾';
		color: var(--text-faint);
	}
	.fold ul {
		margin: 0 0 0.8rem;
		padding: 0;
		list-style: none;
	}
	.fold > details > ul a {
		display: block;
		padding: 0.3rem 0;
		color: var(--text-dim);
	}
	.read {
		border-color: #4ade80;
		color: #4ade80;
	}
	a.chip:hover {
		border-color: var(--area);
		color: var(--area);
	}
	.finish {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.6rem 1rem;
		margin-top: 3rem;
		padding-top: 1.5rem;
		border-top: 1px solid var(--border);
		color: var(--text-faint);
		font-size: 0.85rem;
	}
	.finish button {
		padding: 0.6rem 1.1rem;
		background: var(--area);
		border: 1px solid var(--area);
		border-radius: 999px;
		color: #0b0d14;
		font: inherit;
		font-size: 0.95rem;
		font-weight: 700;
		cursor: pointer;
	}
	.finish button.done {
		background: none;
		border-color: #4ade80;
		color: #4ade80;
	}
	.top {
		position: fixed;
		right: 1.2rem;
		bottom: 1.2rem;
		z-index: 15;
		width: 2.8rem;
		height: 2.8rem;
		background: var(--bg-card);
		border: 1px solid var(--border);
		border-radius: 50%;
		color: var(--text);
		font-size: 1.1rem;
		cursor: pointer;
	}
	.top:hover {
		border-color: var(--accent);
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
	aside ul {
		margin: 0.6rem 0 0;
		padding: 0;
		list-style: none;
	}
	aside li a {
		display: block;
		padding: 0.25rem 0.8rem;
		border-left: 2px solid var(--border);
		color: var(--text-dim);
	}
	aside li a:hover {
		color: var(--text);
	}
	aside li a.active {
		border-left-color: var(--area);
		color: var(--area);
	}
	.pager {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		margin-top: 1.5rem;
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
	.prose :global(.anchor) {
		margin-left: 0.4rem;
		color: var(--text-faint);
		font-weight: 400;
		text-decoration: none;
		opacity: 0;
	}
	.prose :global(:is(h2, h3):hover .anchor),
	.prose :global(.anchor:focus-visible) {
		opacity: 1;
	}
	.prose :global(.code-box) {
		margin: 1.4rem 0;
		overflow: hidden;
		background: #0e111a;
		border: 1px solid var(--border);
		border-radius: var(--radius);
	}
	.prose :global(.code-bar) {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.35rem 0.6rem 0.35rem 1.2rem;
		background: var(--bg-raised);
		border-bottom: 1px solid var(--border);
		color: var(--text-faint);
		font-size: 0.75rem;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}
	.prose :global(.code-bar button) {
		padding: 0.2rem 0.6rem;
		background: none;
		border: 1px solid var(--border);
		border-radius: 6px;
		color: var(--text-dim);
		font: inherit;
		letter-spacing: 0;
		text-transform: none;
		cursor: pointer;
	}
	.prose :global(.code-bar button:hover) {
		border-color: var(--area);
		color: var(--text);
	}
	.prose :global(pre) {
		overflow-x: auto;
		margin: 0;
		padding: 1rem 1.2rem;
		line-height: 1.55;
	}
	.prose :global(.hljs-keyword),
	.prose :global(.hljs-selector-tag),
	.prose :global(.hljs-built_in),
	.prose :global(.hljs-name) {
		color: #c099ff;
	}
	.prose :global(.hljs-string),
	.prose :global(.hljs-attr),
	.prose :global(.hljs-regexp) {
		color: #c3e88d;
	}
	.prose :global(.hljs-comment),
	.prose :global(.hljs-quote) {
		color: #7a88cf;
		font-style: italic;
	}
	.prose :global(.hljs-number),
	.prose :global(.hljs-literal),
	.prose :global(.hljs-symbol) {
		color: #ff966c;
	}
	.prose :global(.hljs-title),
	.prose :global(.hljs-section),
	.prose :global(.hljs-function) {
		color: #82aaff;
	}
	.prose :global(.hljs-type),
	.prose :global(.hljs-class),
	.prose :global(.hljs-meta),
	.prose :global(.hljs-attribute) {
		color: #86e1fc;
	}
	.prose :global(.hljs-variable),
	.prose :global(.hljs-template-variable),
	.prose :global(.hljs-tag) {
		color: #ffc777;
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
	.prose :global(.table-wrap) {
		overflow-x: auto;
		margin: 1.4rem 0;
		border: 1px solid var(--border);
		border-radius: var(--radius);
	}
	.prose :global(table) {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.95rem;
	}
	.prose :global(th),
	.prose :global(td) {
		padding: 0.55rem 0.9rem;
		border-bottom: 1px solid var(--border);
		text-align: left;
	}
	.prose :global(td + td),
	.prose :global(th + th) {
		border-left: 1px solid var(--border);
	}
	.prose :global(tr:last-child td) {
		border-bottom: 0;
	}
	.prose :global(tbody tr:nth-child(even)) {
		background: color-mix(in srgb, var(--bg-raised) 60%, transparent);
	}
	.prose :global(tbody tr:hover) {
		background: color-mix(in srgb, var(--area) 7%, transparent);
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

	@media (max-width: 1280px) {
		.layout {
			grid-template-columns: 15rem minmax(0, 50rem);
		}
		aside {
			display: none;
		}
		.fold {
			display: block;
		}
		.fold > details:first-child {
			display: none;
		}
	}
	@media (max-width: 900px) {
		.layout {
			grid-template-columns: minmax(0, 1fr);
		}
		.side {
			display: none;
		}
		.fold > details:first-child {
			display: block;
		}
	}
</style>
