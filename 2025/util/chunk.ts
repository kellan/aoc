export const str_chunk = function* (str: string, size: number) {
  for (let i = 0; i < str.length; i += size) {
    yield str.slice(i, i + size);
  }
};


