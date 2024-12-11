export function sleep(duration: number): [Promise<void>, () => void] {
  let resolve: (_: void) => void;
  let reject: (error: Error) => void;
  let promise: Promise<void> = new Promise((_resolve, _reject) => {
    resolve = _resolve;
    reject = _reject;
  });
  let timer = setTimeout(() => resolve(), duration);
  return [
    promise,
    () => {
      reject(new Error("Interrupted"));
      clearTimeout(timer);
    },
  ];
}

export function waitForValue<T>(predicate: () => T | void, interval: number = 100): [Promise<T>, () => void] {
  let resolve: (value: T) => void;
  let reject: (error: Error) => void;
  let promise: Promise<T> = new Promise((_resolve, _reject) => {
    resolve = _resolve;
    reject = _reject;
  });
  let value;
  let timer = setInterval(() => {
    if ((value = predicate())) {
      resolve(value);
      clearInterval(timer);
    }
  }, interval);
  return [
    promise,
    () => {
      reject(new Error("Interrupted"));
      clearInterval(timer);
    },
  ];
}
