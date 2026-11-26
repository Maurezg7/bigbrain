import { getTopic } from '#lib/api.ts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => ({
	topic: await getTopic(fetch, params.area, params.topic)
});
