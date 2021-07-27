//! Sync singletons

/// A singleton.
pub trait Singleton {
    /// Inner value contained by the singleton.
    type Inner;

    /// Initialize the singleton.
    ///
    /// This function is expected to panic if called more than once.
    ///
    /// Maybe use std::sync::Once?
    fn init_singleton(inner: Self::Inner);

    /// Use the singleton with an immutable reference.
    // F: Function to be run.
    // R: Function result.
    fn use_singleton<F, R>(clojure: F) -> R
    where
        F: FnOnce(&Self::Inner) -> R;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result.
    fn use_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> R
    where
    F: FnOnce(&Self::Inner, A) -> R;

    /// Use the singleton with a mutable reference.
    // F: Function to be run.
    // R: Function result.
    fn use_mut_singleton<F, R>(clojure: F) -> R
    where
        F: FnOnce(&mut Self::Inner) -> R;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result.
    fn use_mut_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> R
    where
        F: FnOnce(&mut Self::Inner, A) -> R;
}
