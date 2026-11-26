import { getArea, getTopic } from '#lib/api.ts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const [topic, area] = await Promise.all([
		getTopic(fetch, params.area, params.topic),
		getArea(fetch, params.area)
	]);
	return { topic, area };
};
