export type Results = {
  elapsed: number;
  solutions: Solution[];
};

export type Solution = {
  gems: number;
  moves: Move[];
  score: number;
  sorting_score: number;
  swap_count: number;
  word: string;
};

export type Move = {
  swap: boolean;
  index: number;
  new_letter?: string;
};
