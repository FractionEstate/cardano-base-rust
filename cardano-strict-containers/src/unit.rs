/// Helper utilities that conceptually "force" elements to weak head normal
/// form. In Rust evaluation is already strict, so these helpers act as the
/// identity function while retaining the original API surface.
pub fn force_elems_to_whnf<T>(collection: T) -> T {
    collection
}
