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
		if (!read.delete(key)) read.add(key);
		try {
			localStorage.setItem(KEY, JSON.stringify([...read]));
		} catch {
			// Sin almacenamiento disponible, el progreso dura lo que la pestaña.
		}
	}
};
