export const tryFn = async (fn: () => Promise<any>) => {
  try {
    return await fn();
  } catch (e) {
    return e;
  }
}