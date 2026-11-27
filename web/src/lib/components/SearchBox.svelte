<script lang="ts">
	import { goto } from '$app/navigation';
	import { searchTopics, type TopicSummary } from '#lib/api.ts';

	let { value = '', large = false }: { value?: string; large?: boolean } = $props();
	let query = $state('');
	let input = $state<HTMLInputElement>();
	let results = $state<TopicSummary[]>([]);
	let open = $state(false);
	let index = $state(-1);

	$effect(() => {
		query = value;
	});

	// Sugerencias mientras se escribe, con una pausa para no pedir en cada tecla.
	$effect(() => {
		const q = query.trim();
		index = -1;
		if (q.length < 2 || q === value) {
			results = [];
			return;
		}
		let stale = false;
		const timer = setTimeout(async () => {
			try {
				const found = await searchTopics(fetch, q);
				if (!stale) results = found.slice(0, 6);
			} catch {
				if (!stale) results = [];
			}
		}, 180);
		return () => {
			stale = true;
			clearTimeout(timer);
		};
	});

	const href = (t: TopicSummary) => `/${t.area}/${encodeURIComponent(t.slug)}`;

	function go(url: string) {
		open = false;
		input?.blur();
		goto(url);
	}

	function submit(event: SubmitEvent) {
		event.preventDefault();
		const q = query.trim();
		if (index >= 0 && results[index]) go(href(results[index]));
		else if (q) go(`/buscar?q=${encodeURIComponent(q)}`);
	}

	function keydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			open = false;
			index = -1;
		}
		if (!results.length) return;
		if (event.key === 'ArrowDown') {
			event.preventDefault();
			open = true;
			index = (index + 1) % results.length;
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			index = index <= 0 ? results.length - 1 : index - 1;
		}
	}

	function shortcut(event: KeyboardEvent) {
		const target = event.target as HTMLElement;
		if (event.key !== '/' || event.ctrlKey || event.metaKey || event.altKey) return;
		if (target.closest('input, textarea, [contenteditable]')) return;
		if (!input?.offsetParent) return;
		event.preventDefault();
		input.focus();
	}
</script>

<svelte:window onkeydown={shortcut} />

<form
	class:large
	role="search"
	onsubmit={submit}
	onfocusin={() => (open = true)}
	onfocusout={(e) => {
		if (!e.currentTarget.contains(e.relatedTarget as Node)) open = false;
	}}
>
	<svg viewBox="0 0 24 24" aria-hidden="true">
		<circle cx="11" cy="11" r="7" />
		<path d="m20 20-3.5-3.5" />
	</svg>
	<input
		bind:this={input}
		type="search"
		bind:value={query}
		onkeydown={keydown}
		placeholder="Buscar: derivadas, Kirchhoff, enlaces, recursión…"
		aria-label="Buscar temas"
		autocomplete="off"
	/>
	<kbd aria-hidden="true">/</kbd>

	{#if open && results.length}
		<ul class="suggestions">
			{#each results as topic, i (topic.area + topic.slug)}
				<li>
					<a href={href(topic)} class:active={i === index} onclick={() => (open = false)}>
						<strong>{topic.title}</strong>
						<span>{topic.branch} · {topic.minutes} min</span>
					</a>
				</li>
			{/each}
			<li>
				<a class="all" href="/buscar?q={encodeURIComponent(query.trim())}" onclick={() => (open = false)}>
					Ver todos los resultados para «{query.trim()}»
				</a>
			</li>
		</ul>
	{/if}
</form>

<style>
	form {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.45rem 0.8rem;
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: 999px;
		transition:
			border-color 0.15s,
			box-shadow 0.15s;
	}
	form:focus-within {
		border-color: var(--accent);
		box-shadow: 0 0 0 4px color-mix(in srgb, var(--accent) 18%, transparent);
	}
	form.large {
		padding: 0.85rem 1.2rem;
		font-size: 1.05rem;
	}
	svg {
		width: 1.1em;
		height: 1.1em;
		flex: none;
		fill: none;
		stroke: var(--text-faint);
		stroke-width: 2;
		stroke-linecap: round;
	}
	input {
		flex: 1;
		min-width: 0;
		border: 0;
		outline: 0;
		background: none;
		color: var(--text);
		font: inherit;
	}
	input::placeholder {
		color: var(--text-faint);
	}
	kbd {
		padding: 0 0.4rem;
		border: 1px solid var(--border);
		border-radius: 5px;
		color: var(--text-faint);
		font-family: var(--mono);
		font-size: 0.75rem;
	}
	form:focus-within kbd {
		display: none;
	}
	.suggestions {
		position: absolute;
		top: calc(100% + 0.5rem);
		right: 0;
		left: 0;
		z-index: 30;
		min-width: 18rem;
		margin: 0;
		padding: 0.4rem;
		overflow: hidden;
		background: var(--bg-card);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		box-shadow: 0 16px 40px rgb(0 0 0 / 0.5);
		font-size: 0.92rem;
		list-style: none;
		text-align: left;
	}
	.suggestions a {
		display: flex;
		flex-direction: column;
		padding: 0.5rem 0.7rem;
		border-radius: 9px;
	}
	.suggestions a:hover,
	.suggestions a.active {
		background: color-mix(in srgb, var(--accent) 14%, transparent);
	}
	.suggestions span {
		color: var(--text-faint);
		font-size: 0.8rem;
	}
	.all {
		color: var(--accent);
	}
	@media (pointer: coarse) {
		kbd {
			display: none;
		}
	}
</style>
