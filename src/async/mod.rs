//! Async singletons.

mod futures;

use std::future::Future;

pub use self::futures::*;

// compile_error!("The `async` feature is not supported yet.");

/// An async singleton.
pub trait AsyncSingleton {
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
    fn use_singleton<Func, Fut, Ret>(clojure: Func) -> UseSingletonFuture<Func, Fut, Self::Inner, Ret>
    where
        Func: FnOnce(&Self::Inner) -> Fut,
        Fut: Future<Output = Ret>;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result.
    fn use_singleton_with_arg<Arg, Func, Fut, Ret>(clojure: Func, arg: Arg) -> Fut
    where
        Func: FnOnce(&Self::Inner, Arg) -> Ret,
        Fut: Future<Output = Ret>;

    /// Use the singleton with a mutable reference.
    // F: Function to be run.
    // R: Function result.
    fn use_mut_singleton<Func, Fut, Ret>(clojure: Func) -> Fut
    where
        Func: FnOnce(&mut Self::Inner) -> Ret,
        Fut: Future<Output = Ret>;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result.
    fn use_mut_singleton_with_arg<Arg, Func, Fut, Ret>(clojure: Func, arg: Arg) -> Fut
    where
        Func: FnOnce(&mut Self::Inner, Arg) -> Ret,
        Fut: Future<Output = Ret>;
}
