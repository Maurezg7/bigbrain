import { SvelteSet } from 'svelte/reactivity';

const KEY = 'bigbrain:leidos';

function load(): string[] {
	try {
		const raw = JSON.parse(localStorage.getItem(KEY) ?? '[]');
		return Array.isArray(raw) ? raw.filter((x) => typeof x === 'string') : [];
	} catch {
		return [];
	}
}

const read = new SvelteSet<string>(load());

// Otra pestaña cambió el avance: se adopta lo que quedó guardado.
function sync() {
	const stored = new Set(load());
	for (const key of read) if (!stored.has(key)) read.delete(key);
	for (const key of stored) read.add(key);
}
if (typeof window !== 'undefined') {
	window.addEventListener('storage', (event) => {
		if (event.key === KEY) sync();
	});
}

const id = (area: string, slug: string) => `${area}/${slug}`;

/** Temas marcados como leídos; se guardan solo en este navegador. */
export const progress = {
	has: (area: string, slug: string) => read.has(id(area, slug)),
	count: (area: string) => {
		let n = 0;
		for (const key of read) if (key.startsWith(`${area}/`)) n++;
		return n;
	},
	toggle(area: string, slug: string) {
		const key = id(area, slug);
		const marked = !read.has(key);
		if (marked) read.add(key);
		else read.delete(key);
		// Solo se guarda este cambio sobre lo almacenado, para no pisar otra pestaña.
		const stored = new Set(load());
		if (marked) stored.add(key);
		else stored.delete(key);
		try {
			localStorage.setItem(KEY, JSON.stringify([...stored]));
		} catch {
			// Sin almacenamiento disponible, el progreso dura lo que la pestaña.
		}
	}
};
