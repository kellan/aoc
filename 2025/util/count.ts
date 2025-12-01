export const countGroups = <T extends string | number | symbol>(list: T[]) => {
	const counts: Partial<Record<T, number>> = {};

	for (const i of list) counts[i] = (counts[i] ?? 0) + 1;
	return counts;
};