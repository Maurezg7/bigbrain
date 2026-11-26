<script lang="ts">
	import { goto } from '$app/navigation';

	let { value = '', large = false }: { value?: string; large?: boolean } = $props();
	let query = $state('');
	$effect(() => {
		query = value;
	});

	function submit(event: SubmitEvent) {
		event.preventDefault();
		const q = query.trim();
		if (q) goto(`/buscar?q=${encodeURIComponent(q)}`);
	}
</script>

<form class:large role="search" onsubmit={submit}>
	<svg viewBox="0 0 24 24" aria-hidden="true">
		<circle cx="11" cy="11" r="7" />
		<path d="m20 20-3.5-3.5" />
	</svg>
	<input
		type="search"
		bind:value={query}
		placeholder="Buscar: derivadas, Kirchhoff, enlaces, recursión…"
		aria-label="Buscar temas"
	/>
</form>

<style>
	form {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.45rem 0.8rem;
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: 999px;
		transition: border-color 0.15s;
	}
	form:focus-within {
		border-color: var(--accent);
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
</style>
