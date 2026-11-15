//! Crate `consequence` provides a tri-state container type [`Outcome`], 
//! similar to [`Result`] and [`Option`].
//! 
//! ### Author's Motivation
//! I had been frustrated every time I came across a situation, where
//! a state of an application could be one of following:
//! - operation was executed successfully with a positive result;
//! - operation was executed successfully with a negative result;
//! - operation failed during execution.
//! The Rust's type system is a powerful tool for logical modelling of such
//! situations. Nevertheless, I hadn't yet enocountered a type for expressing
//! this exact situation. That's why I decided to write this - a type that
//! encapsulates all three scenarios mentioned above.
//! 
//! ---
//! 
//! Type [`Outcome<S, F, E>`], similar to [`Result<T, E>`], is meant for 
//! returning and propagating errors, but with one important distinction. 
//! It differentiates between "business logic errors" and "runtime errors":
//! - [`Outcome::Success`] - operation executed successfully with a positive outcome;
//! - [`Outcome::Failure`] - operation executed successfully with a negative outcome;
//! - [`Outcome::Error`] - an error occured while executing the operation.
//! It is not a general replacement for a `Result`, but rather a situation-specific
//! type for when a distinction between a "failure" and an "error" is desired.
//! 
//! `Outcome` is written in a very similar manner to `Result`, especially when it
//! comes to its member functions which mimic `Result`'s behaviour to a great extent.
//! 
//! <div class="warning">
//! 
//! Note that because this library is stable, it does not support the `?` operator
//! at the moment. Once the [`try_trait_v2`][https://github.com/rust-lang/rust/issues/84277]
//! feature stabilizes, this operator will probably be implemented. For now, to achieve analogous behaviour,
//! use `outcome.into_result()?`. See [`Outcome::into_result`].
//! 
//! </div>
//! 
//! # Method overview
//! 
//! Type [`Outcome`] has methods that closely correlate with `Result` and can be use with pattern matching too.
//! 
//! ### Query functions
//! The [`Outcome::is_success`], [`Outcome::is_failure`] and [`Outcome::is_error`] return `true` if
//! the [`Outcome`] is the respective variant.
//! 
//! The [`Outcome::is_success_and`], [`Outcome::is_failure_and`] and [`Outcome::is_error_and`] apply a provided
//! function that consumes the `Outcome` variant and returns a `bool` if the variant is the correct one or `false` if the
//! variant is not the specified one.
//! 
//! ### Unwrapping functions
//! 
//! These methods return the contained value from [`Outcome<S, F, E>`] when it is the `Success` variant.
//! In other scenarios:
//! - [`Outcome::expect`] - panics with a provided message, printing an identifier `[FAILURE]` or `[ERROR]`
//! - [`Outcome::unwrap`] - panics with a default message
//! - [`Outcome::unwrap_or`] - returns a provided default value
//! - [`Outcome::unwrap_or_default`] - returns a default value of the type `S`
//! 
//! Analogously to `Success`, `Failure` has these methods to unwrap it or:
//! - [`Outcome::expect_failure`] - panics with a provided message, printing an identifier `[SUCCESS]` or `[ERROR]`
//! - [`Outcome::unwrap_failure`] - panics with a default message
//! - [`Outcome::unwrap_failure_or`] - returns a provided default value
//! - [`Outcome::unwrap_failure_or_default`] - returns a default value of type `F`
//! 
//! These methods extract the error value if the `Outcome` is `Error` or else:
//! - [`Outcome::expect_error`] - panics with a provided message, printing an identifier `[SUCCESS]` or `[FAILURE]` 
//! - [`Outcome::unwrap_error`] - panics with a default message
//! 
//! ### Transforming functions
//! These methods transform [`Outcome`] to [`Option`]:
//! - [`Outcome::success`] transforms [`Outcome`] to [`Option<S>`], mapping [`Success(s)`] to [`Some(s)`] and other 
//! variants to [`None`]
//! - [`Outcome::failure`] transforms [`Outcome`] to [`Option<F>`], mapping [`Failure(f)`] to [`Some(f)`] and other 
//! variants to [`None`]
//! - [`Outcome::error`] transforms [`Outcome`] to [`Option<E>`], mapping [`Error(e))`] to [`Some(e)`] and other 
//! variants to [`None`]
//! - [`Outcome::transpose`] transposes [`Outcome`] of [`Option`] to [`Option`] of [`Outcome`]
//! - [`Outcome::flatten`] flattens `Outcome<Outcome<S, F, E>, F, E>` to `Outcome<S, F, E>`
//! - [`Outcome::into_result`] transforms [`Outcome<S, F, E>`] into [`Result<Result<S, F>, E>`]
//! 
//! These methods transform the encapsulated value if the [`Outcome`] is [`Success`]:
//! - [`Outcome::map`] transforms [`Outcome<S, F, E>`] into [`Outcome<U, F, E>`] by applying the
//! provided function to the contained value of [`Success`]
//! - [`Outcome::inspect`] applies the provided function to the contained value of [`Success`] by reference
//! and returns the modified [`Outcome`]
//! 
//! These methods transform the encapsulated value if the [`Outcome`] is [`Failure`]:
//! - [`Outcome::map_failure`] transforms [`Outcome<S, F, E>`] into [`Outcome<S, U, E>`] by applying the
//! provided function to the contained value of [`Failure`]
//! - [`Outcome::inspect_failure`] applies the provided function to the contained value of [`Failure`] by reference
//! and returns the modified [`Outcome`]
//! 
//! These methods transform the encapsulated value if the [`Outcome`] is [`Error`]:
//! - [`Outcome::map_error`] transforms [`Outcome<S, F, E>`] into [`Outcome<S, F, U>`] by applying the
//! provided function to the contained value of [`Error`]
//! - [`Outcome::inspect_error`] applies the provided function to the contained value of [`Error`] by reference
//! and returns the modified [`Outcome`]
//! 
//! There is a wildcard function [`Outcome::map_all`] which is roughly equivalent to pattern matching on [`Outcome`]
//! 
//! There are methods that transform [`Outcome<S, F, E>`] into a concrete value:
//! - [`Outcome::map_or`] applies the supplied function to the contained value of [`Success`]
//! or returns a supplied default value
//! - [`Outcome::map_or_default`] applies the supplied function to the contained value of [`Success`]
//! or returns a default value of type `F`
//! - [`Outcome::map_failure_or`] applies the supplied function to the contained value of [`Failure`]
//! or returns a supplied default value
//! - [`Outcome::map_failure_or_default`] applies the supplied function to the contained value of [`Failure`]
//! or returns a default value of type `F`
//! 
//! ### Reference functions
//! - [`Outcome::as_ref`] converts from [`&Outcome<S, F, E>`] to [`Outcome<&S, &F, &E>`]
//! - [`Outcome::as_mut`] converts from [`&mut Outcome<S, F, E>`] to [`Outcome<&mut S, &mut F, &mut E>`]
//! - [`Outcome::as_deref`] converts from [`&Outcome<S, F, E>`] to [`Outcome<&S::Target, &F, &E>`]
//! - [`Outcome::as_deref_mut`] converts from [`&mut Outcome<S, F, E>`] to [`Outcome<&mut S::Target, &mut F, &mut E>`]
//! 
//! ### Iterator functions
//! Similar to [`Result`], [`Outcome`] can be iterated over, that will either produce a single value if
//! [`Outcome`] is [`Success`] or none otherwise.
//! 
//! There are three functions that produce iterators:
//! - [`Outcome::into_iter`] consumes the [`Outcome`] and produces the value
//! - [`Outcome::iter`] produces an immutable reference of type `&S` to the contained value
//! - [`Outcome::iter_mut`] produces a mutable reference of type `&mut S` to the contained value
//! 
//! ### Boolean functions
//! Methods below treat the [`Outcome`] somewhat as if it was a boolean value.
//! 
//! | method                | self                      | other                                | output  |
//! |-----------------------|---------------------------|--------------------------------------|---------|
//! | [`Outcome::and`]      | `Success(s)`              | `Success(u)`/`Failure(f)`/`Error(e)` | `other` |
//! | [`Outcome::and`]      | `Failure(f)`/`Error(e)`   | (ignored)                            | `self`  |
//! | [`Outcome::or`]       | `Failure(f)`              | `Success(s)`/`Failure(u)`/`Error(e)` | `other` |
//! | [`Outcome::or`]       | `Success(s)`/`Error(e)`   | (ignored)                            | `self`  |
//! | [`Outcome::or_error`] | `Error(e)`                | `Success(s)`/`Failure(f)`/`Error(u)` | `other` |
//! | [`Outcome::or_error`] | `Success(f)`/`Failure(f)` | (ignored)                            | `self`  |
//! 
//! Additionally there is a method [`Outcome::and_then`] that calls the passed-in function if 
//! the [`Outcome`] is [`Success`] or returns the outcome otherwise.

#[warn(missing_docs)]

mod outcome;

pub use outcome::*;