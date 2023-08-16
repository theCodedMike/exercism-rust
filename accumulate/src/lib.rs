/// What should the type of _function be?
pub fn map<T, R, F>(input: Vec<T>, mut f: F) -> Vec<R>
where
    F: FnMut(T) -> R,
{
    input.into_iter().map(|e| f(e)).collect()
}
