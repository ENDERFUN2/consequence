<div align="center">

# Consequence

[![Crates.io](https://img.shields.io/crates/v/consequence.svg)](https://crates.io/crates/consequence)
[![Documentation](https://docs.rs/consequence/badge.svg)](https://docs.rs/consequence)
[![Downloads](https://img.shields.io/crates/d/consequence.svg)](https://crates.io/crates/consequence)
[![License](https://img.shields.io/crates/l/consequence.svg)](https://github.com/ENDERFUN2/consequence/blob/main/LICENSE)
</div>


`consequence` provides yet another tri-state container type, `Outcome`, similar to `Result` and `Option`.

Unlike `Result`, which distinguishes between success and error, `Outcome` introduces an additional state: a logical failure. This makes it particularly useful when an operation can execute successfully but still produce a negative outcome.

## Author's Motivation

I had been frustrated every time I came across a situation where the state of an application could be one of the following:

- An operation was executed successfully with a positive result.
- An operation was executed successfully with a negative result.
- An operation failed during execution.

Rust's type system is a powerful tool for modelling such situations. Nevertheless, I hadn't yet encountered a type that expressed this exact distinction. That's why I decided to write `consequence`: a type that encapsulates all three scenarios.

## Overview

Type `Outcome<S, F, E>`, similar to `Result<T, E>`, is intended for returning and propagating errors, but with one important distinction: it differentiates between logical failures and runtime errors.

It has three variants:

- `Outcome::Success` - the operation executed successfully with a positive outcome.
- `Outcome::Failure` - the operation executed successfully with a negative outcome.
- `Outcome::Error` - an error occurred while executing the operation.

It is not intended as a general replacement for `Result`, but rather as a situation-specific type for cases where distinguishing between a failure and an error is desirable.

`Outcome` is implemented in a manner very similar to `Result`, especially when it comes to its member functions, which mimic `Result`'s behavior to a great extent.

### A practical example

Consider a user registration operation. There are three possible scenarios:

| Scenario | Outcome |
|---|---|
| The user was successfully registered. | `Success(user)` |
| The username is already taken. | `Failure(reason)` |
| The database connection could not be established. | `Error(error)` |

Notice that an already occupied username does not necessarily indicate that the registration operation itself failed. The operation executed correctly and determined that registration should be rejected.

This distinction is precisely what `Outcome` is designed to express.

## Quick Start

A simple example demonstrating how `Outcome` can be used:

```rust
use consequence::Outcome;

fn register_user(username: &str) -> Outcome<String, String, String> {
    if username.is_empty() {
        return Outcome::Failure(
            "Username cannot be empty".into()
        );
    }

    if let Err(e) = model.save(username) {
      return Outcome::Error(e.to_string())
    }

    Outcome::Success(username.to_string())
}

fn main() {
    match register_user("alice") {
        Outcome::Success(user) => {
            println!("Registered user: {user}");
        }
        Outcome::Failure(reason) => {
            println!("Registration rejected: {reason}");
        }
        Outcome::Error(error) => {
            println!("Unexpected error: {error}");
        }
    }
}
```

The example uses `String` for all three contained types for simplicity. In practice, each variant can hold a completely different type.

### Working with `Result`

`Outcome` also provides interoperability with Rust's standard `Result` type.

For example, an operation returning `Result<Result<S, F>, E>` can be converted into `Outcome<S, F, E>`:

```rust
use consequence::Outcome;

fn main() {
    let result: Result<Result<i32, String>, String> =
        Ok(Ok(42));

    let outcome: Outcome<i32, String, String> =
        result.into();

    assert_eq!(outcome, Outcome::Success(42));
}
```

Conversely, `Outcome::into_result` converts an `Outcome` into `Result<Result<S, F>, E>`.

```rust
let outcome: Outcome<i32, String, String> =
    Outcome::Failure("Invalid input".into());

let result = outcome.into_result();

assert_eq!(
    result,
    Ok(Err("Invalid input".into()))
);
```

### The `?` operator

Note that because this library is stable, it does not support the `?` operator directly at the moment.

Once the [`try_trait_v2`](https://github.com/rust-lang/rust/issues/84277) feature stabilizes, this operator will probably be implemented.

For now, to achieve analogous behaviour, use `outcome.into_result()?`. See `Outcome::into_result`.

---

# Method Overview

Type `Outcome` provides methods that closely correlate with `Result` and can also be used with pattern matching.

## Query Functions

The `Outcome::is_success`, `Outcome::is_failure` and `Outcome::is_error` methods return `true` if the `Outcome` is the respective variant.

The `Outcome::is_success_and`, `Outcome::is_failure_and` and `Outcome::is_error_and` methods apply a provided function to the contained value if the `Outcome` is the respective variant. They return `false` otherwise.

```rust
let outcome: Outcome<i32, String, String> =
    Outcome::Success(42);

assert!(outcome.is_success());
assert!(!outcome.is_failure());
assert!(!outcome.is_error());

let outcome: Outcome<i32, String, String> =
    Outcome::Success(42);

assert!(outcome.is_success_and(|value| value > 40));
```

## Unwrapping Functions

These methods return the contained value from `Outcome<S, F, E>` when it is the `Success` variant.

In other scenarios:

- `Outcome::expect` - panics with provided messages, printing an identifier `[FAILURE]` or `[ERROR]`.
- `Outcome::unwrap` - panics with a default message.
- `Outcome::unwrap_or` - returns a provided default value.
- `Outcome::unwrap_or_default` - returns a default value of type `S`.

For example:

```rust
let outcome: Outcome<i32, String, String> =
    Outcome::Success(42);

assert_eq!(outcome.unwrap(), 42);

let outcome: Outcome<i32, String, String> =
    Outcome::Failure("Rejected".into());

assert_eq!(outcome.unwrap_or(0), 0);
```

Analogously to `Success`, `Failure` has the following methods:

- `Outcome::expect_failure` - panics with provided messages, printing an identifier `[SUCCESS]` or `[ERROR]`.
- `Outcome::unwrap_failure` - panics with a default message.
- `Outcome::unwrap_failure_or` - returns a provided default value.
- `Outcome::unwrap_failure_or_default` - returns a default value of type `F`.

These methods extract the error value if the `Outcome` is the `Error` variant:

- `Outcome::expect_error` - panics with provided messages, printing an identifier `[SUCCESS]` or `[FAILURE]`.
- `Outcome::unwrap_error` - panics with a default message.

## Transforming Functions

### Converting to `Option`

These methods transform `Outcome` into `Option`:

- `Outcome::success` transforms `Outcome<S, F, E>` into `Option<S>`, mapping `Success(s)` to `Some(s)` and other variants to `None`.
- `Outcome::failure` transforms `Outcome<S, F, E>` into `Option<F>`, mapping `Failure(f)` to `Some(f)` and other variants to `None`.
- `Outcome::error` transforms `Outcome<S, F, E>` into `Option<E>`, mapping `Error(e)` to `Some(e)` and other variants to `None`.

```rust
let outcome: Outcome<i32, String, String> =
    Outcome::Success(42);

assert_eq!(outcome.success(), Some(42));
```

### Converting and restructuring `Outcome`

- `Outcome::transpose` transposes `Outcome<Option<S>, F, E>` into `Option<Outcome<S, F, E>>`.
- `Outcome::flatten` flattens `Outcome<Outcome<S, F, E>, F, E>` into `Outcome<S, F, E>`.
- `Outcome::into_result` transforms `Outcome<S, F, E>` into `Result<Result<S, F>, E>`.

### Mapping `Success`

These methods transform the encapsulated value if the `Outcome` is `Outcome::Success`:

- `Outcome::map` transforms `Outcome<S, F, E>` into `Outcome<U, F, E>` by applying the provided function to the contained value of `Success`.
- `Outcome::inspect` applies the provided function to the contained value of `Success` by reference and returns the original `Outcome`.
- `Outcome::map_or` applies the supplied function to the contained value of `Success` or returns a supplied default value.
- `Outcome::map_or_default` applies the supplied function to the contained value of `Success` or returns a default value of the resulting type `U`.

```rust
let outcome: Outcome<i32, String, String> =
    Outcome::Success(10);

let doubled = outcome.map(|value| value * 2);

assert_eq!(doubled, Outcome::Success(20));
```

### Mapping `Failure`

These methods transform the encapsulated value if the `Outcome` is `Outcome::Failure`:

- `Outcome::map_failure` transforms `Outcome<S, F, E>` into `Outcome<S, U, E>` by applying the provided function to the contained value of `Failure`.
- `Outcome::inspect_failure` applies the provided function to the contained value of `Failure` by reference and returns the original `Outcome`.
- `Outcome::map_failure_or` applies the supplied function to the contained value of `Failure` or returns a supplied default value.
- `Outcome::map_failure_or_default` applies the supplied function to the contained value of `Failure` or returns a default value of the resulting type `U`.

### Mapping `Error`

These methods transform the encapsulated value if the `Outcome` is `Outcome::Error`:

- `Outcome::map_error` transforms `Outcome<S, F, E>` into `Outcome<S, F, U>` by applying the provided function to the contained value of `Error`.
- `Outcome::inspect_error` applies the provided function to the contained value of `Error` by reference and returns the original `Outcome`.

### Mapping all variants

`Outcome::map_all` is a wildcard function that applies a different function to each of the three variants. It is roughly equivalent to pattern matching on `Outcome`.

```rust
let outcome: Outcome<i32, String, String> =
    Outcome::Success(42);

let message = outcome.map_all(
    |value| format!("Success: {value}"),
    |failure| format!("Failure: {failure}"),
    |error| format!("Error: {error}"),
);

assert_eq!(message, "Success: 42");
```

## Reference Functions

These methods provide access to the contained values without consuming the original `Outcome`:

- `Outcome::as_ref` converts `&Outcome<S, F, E>` into `Outcome<&S, &F, &E>`.
- `Outcome::as_mut` converts `&mut Outcome<S, F, E>` into `Outcome<&mut S, &mut F, &mut E>`.
- `Outcome::as_deref` converts `&Outcome<S, F, E>` into `Outcome<&S::Target, &F, &E>`.
- `Outcome::as_deref_mut` converts `&mut Outcome<S, F, E>` into `Outcome<&mut S::Target, &mut F, &mut E>`.

## Iterator Functions

Similar to `Result`, `Outcome` can be iterated over. Iteration produces either a single value if `Outcome` is `Success` or no values otherwise.

There are three functions that produce iterators:

- `Outcome::into_iter` consumes the `Outcome` and produces the contained value if it is `Success`.
- `Outcome::iter` produces an immutable reference of type `&S` to the contained value.
- `Outcome::iter_mut` produces a mutable reference of type `&mut S` to the contained value.

Additionally, `Outcome` implements `IntoIterator` for owned, immutable-reference and mutable-reference values.

## Boolean Functions

The methods below treat `Outcome` somewhat as if it were a boolean value.

### `and`

Returns `other` if `self` is `Success`, or returns `self` otherwise.

| `self` | `other` | Output |
|---|---|---|
| `Success(s)` | Any variant | `other` |
| `Failure(f)` | Ignored | `Failure(f)` |
| `Error(e)` | Ignored | `Error(e)` |

### `or`

Returns `other` if `self` is `Failure`, or returns `self` otherwise.

| `self` | `other` | Output |
|---|---|---|
| `Success(s)` | Ignored | `Success(s)` |
| `Failure(f)` | Any variant | `other` |
| `Error(e)` | Ignored | `Error(e)` |

### `or_error`

Returns `other` if `self` is `Error`, or returns `self` otherwise.

| `self` | `other` | Output |
|---|---|---|
| `Success(s)` | Ignored | `Success(s)` |
| `Failure(f)` | Ignored | `Failure(f)` |
| `Error(e)` | Any variant | `other` |

### Chaining operations

Additionally, there are methods that allow operations to be chained without explicitly matching every variant:

- `Outcome::and_then` calls the passed-in function if `self` is `Success`, or returns the original outcome otherwise.
- `Outcome::or_else` calls the passed-in function if `self` is `Failure`, or returns the original outcome otherwise.
- `Outcome::or_error_else` calls the passed-in function if `self` is `Error`, or returns the original outcome otherwise.

For example:

```rust
fn validate(value: i32) -> Outcome<i32, String, String> {
    if value > 0 {
        Outcome::Success(value)
    } else {
        Outcome::Failure("Value must be positive".into())
    }
}

fn process(value: i32) -> Outcome<i32, String, String> {
    Outcome::Success(value * 2)
}

let outcome = validate(10).and_then(process);

assert_eq!(outcome, Outcome::Success(20));
```

## Collection and Aggregation

`Outcome` supports collecting multiple outcomes into a single outcome through `FromIterator`.

If all outcomes are `Success`, their contained values are collected into a collection. If a `Failure` or `Error` is encountered, collection stops and that variant is returned.

```rust
let outcomes = vec![
    Outcome::<i32, String, String>::Success(10),
    Outcome::Success(20),
    Outcome::Success(30),
];

let collected: Outcome<Vec<i32>, String, String> =
    outcomes.into_iter().collect();

assert_eq!(
    collected,
    Outcome::Success(vec![10, 20, 30])
);
```

`Outcome` also implements `Sum` and `Product`, allowing successful values to be aggregated while propagating failures and errors.

---

For the complete API documentation, see the generated [API documentation](https://docs.rs/consequence).
