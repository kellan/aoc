export type LineOptions = {
    size?: 'line' | 'block';
    trim?: boolean;
};

export const lines = <T = string>(
  content: string,
  mapper: (line: string, index: number) => T = (x => x as T),
  { 
    size = "line", 
    trim = true 
  }: LineOptions = {}
): T[] => {
  const text = trim ? content.trimEnd() : content;
  const delimiter = size === "line" ? "\n" : "\n\n";
  return text.split(delimiter).map(mapper);

};


