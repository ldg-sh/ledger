export type Result<T, E> =
    | { ok: true; value: T }
    | { ok: false; error: E };

export const Ok = <T>(value: T): Result<T, never> => ({ ok: true, value });
export const Err = <E>(error: E): Result<never, E> => ({ ok: false, error });

export function tryCatch<T, E = Error>(
    fn: () => T,
    mapError: (e: unknown) => E = (e) => e as E
): Result<T, E> {
    try {
        return Ok(fn());
    } catch (e) {
        return Err(mapError(e));
    }
}

export async function tryCatchAsync<T, E = Error>(
    fn: () => Promise<T>,
    mapError: (e: unknown) => E = (e) => e as E
): Promise<Result<T, E>> {
    try {
        return Ok(await fn());
    } catch (e) {
        return Err(mapError(e));
    }
}
