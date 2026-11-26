export interface Area {
	slug: string;
	title: string;
	description: string;
	symbol: string;
	color: string;
	branches: string[];
	topic_count: number;
}

export interface TopicSummary {
	area: string;
	slug: string;
	title: string;
	summary: string;
	branch: string;
	level: string;
	tags: string[];
	minutes: number;
}

export interface AreaDetail extends Area {
	topics: TopicSummary[];
}

export interface TopicDetail extends TopicSummary {
	area_title: string;
	color: string;
	html: string;
	headings: { id: string; text: string }[];
	prev: TopicSummary | null;
	next: TopicSummary | null;
}

type Fetch = typeof fetch;

async function get<T>(fetch: Fetch, path: string): Promise<T> {
	const res = await fetch(`/api${path}`);
	if (!res.ok) {
		const { error } = await import('@sveltejs/kit');
		error(res.status, res.status === 404 ? 'No encontramos esa página' : 'Error del servidor');
	}
	return res.json();
}

export const getAreas = (f: Fetch) => get<Area[]>(f, '/areas');
export const getArea = (f: Fetch, slug: string) =>
	get<AreaDetail>(f, `/areas/${encodeURIComponent(slug)}`);
export const getTopic = (f: Fetch, area: string, slug: string) =>
	get<TopicDetail>(f, `/topics/${encodeURIComponent(area)}/${encodeURIComponent(slug)}`);
export const searchTopics = (f: Fetch, q: string) =>
	get<TopicSummary[]>(f, `/search?q=${encodeURIComponent(q)}`);

/** Minúsculas y sin diacríticos, igual que la búsqueda del servidor. */
export const normalize = (s: string) =>
	s
		.toLowerCase()
		.normalize('NFD')
		.replace(/[\u0300-\u036f]/g, '');
