import { getArea } from '#lib/api.ts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => ({
	area: await getArea(fetch, params.area)
});
