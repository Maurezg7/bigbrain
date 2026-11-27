import { searchTopics } from '#lib/api.ts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, url }) => {
	const q = url.searchParams.get('q')?.trim() ?? '';
	return { q, results: q ? await searchTopics(fetch, q) : [] };
};
