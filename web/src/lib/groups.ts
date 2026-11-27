import type { AreaDetail, TopicSummary } from '#lib/api.ts';

/** Temas del área agrupados por rama, en el orden declarado por el área. */
export function groupByBranch(area: AreaDetail): { branch: string; topics: TopicSummary[] }[] {
	return [...new Set([...area.branches, ...area.topics.map((t) => t.branch)])]
		.map((branch) => ({ branch, topics: area.topics.filter((t) => t.branch === branch) }))
		.filter((g) => g.topics.length);
}
