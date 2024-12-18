export type ServerResponse = {
  ok: boolean;
  data?: Results;
  error?: string;
};

export type Results = {
  elapsed_ms: {
    dict: number;
    solver: number;
  };
  words: Word[];
};

export type Word = {
  gems_collected: number;
  steps: Step[];
  score: number;
  swaps_used: number;
  word: string;
};

export type Step = {
  swap: boolean;
  index: number;
  new_letter?: string;
};
