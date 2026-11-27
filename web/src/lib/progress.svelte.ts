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

// Otra pestaña cambió el avance: se vuelve a leer para no pisar sus marcas.
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
		const was = read.has(key);
		sync();
		if (was !== read.has(key)) return;
		if (!read.delete(key)) read.add(key);
		try {
			localStorage.setItem(KEY, JSON.stringify([...read]));
		} catch {
			// Sin almacenamiento disponible, el progreso dura lo que la pestaña.
		}
	}
};
