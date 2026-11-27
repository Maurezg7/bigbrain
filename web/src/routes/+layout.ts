import { getAreas } from '#lib/api.ts';
import type { LayoutLoad } from './$types';

export const ssr = false;
export const prerender = false;

export const load: LayoutLoad = async ({ fetch }) => ({ areas: await getAreas(fetch) });
