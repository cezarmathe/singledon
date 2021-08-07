//! Sync singletons

/// A sync singleton.
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
    // Func: Function to be run.
    // Ret: Function result.
    fn use_singleton<Func, Return>(func: Func) -> Return
    where
        Func: FnOnce(&Self::Inner) -> Return;

    /// Use the singleton with an immutable reference and an argument.
    // Func: Function to be run.
    // Arg: Function argument.
    // Ret: Function result.
    fn use_singleton_with_arg<Func, Arg, Ret>(clojure: Func, arg: Arg) -> Ret
    where
        Func: FnOnce(&Self::Inner, Arg) -> Ret;

    /// Use the singleton with a mutable reference.
    // Func: Function to be run.
    // Ret: Function result.
    fn use_mut_singleton<Func, Ret>(clojure: Func) -> Ret
    where
        Func: FnOnce(&mut Self::Inner) -> Ret;

    /// Use the singleton with an immutable reference and an argument.
    // Func: Function to be run.
    // Arg: Function argument.
    // Ret: Function result.
    fn use_mut_singleton_with_arg<Func, Arg, Ret>(clojure: Func, arg: Arg) -> Ret
    where
        Func: FnOnce(&mut Self::Inner, Arg) -> Ret;
}
