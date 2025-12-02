export const range = function* (start, end) {
  for (let i = start; i <= end; i++) yield i;
};
