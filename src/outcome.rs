use std::{fmt::Debug, iter::FusedIterator, ops::{Deref, DerefMut}};

use crate::outcome::Outcome::{Failure, Success, Error};

#[derive(Copy, Debug, PartialEq, Eq, Hash)]
#[must_use]
pub enum Outcome<S, F, E> {
    Success(S),
    Failure(F),
    Error(E)
}

impl <S, F, E> Outcome<S, F, E> {
    #[inline]
    pub const fn is_success(&self) -> bool {
        matches!(*self, Self::Success(_))
    }

    #[inline]
    pub fn is_success_and<G>(self, fun: G) -> bool
    where G: FnOnce(S) -> bool {
        match self {
            Success(x) => fun(x),
            _ => false
        }
    }
    
    #[inline]
    pub const fn is_failure(&self) -> bool {
        matches!(*self, Self::Failure(_))
    }

    #[inline]
    pub fn is_failure_and<G>(self, fun: G) -> bool
    where G: FnOnce(F) -> bool {
        match self {
            Failure(x) => fun(x),
            _ => false
        }
    }

    #[inline]
    pub fn is_failure_or<G>(self, op: G) -> bool
    where
        G: FnOnce(F) -> bool,
    {
        match self {
            Failure(failure) => op(failure),
            _ => false,
        }
    }

    #[inline]
    pub const fn is_error(&self) -> bool {
        matches!(*self, Self::Error(_))
    }

    #[inline]
    pub fn is_error_and<G>(self, fun: G) -> bool
    where G: FnOnce(E) -> bool {
        match self {
            Error(x) => fun(x),
            _ => false
        }
    }

    #[inline]
    pub fn is_error_or<FN>(self, f: FN) -> bool
    where
        FN: FnOnce(E) -> bool,
    {
        match self {
            Error(error) => f(error),
            Success(_) | Failure(_) => false,
        }
    }
    
    #[inline]
    pub fn success(self) -> Option<S> {
        match self {
            Success(x) => Some(x),
            _ => None
        }
    }

    #[inline]
    pub fn success_or(self, default: S) -> S {
        match self {
            Success(success) => success,
            _ => default,
        }
    }
    
    pub fn success_or_else<G>(self, op: G) -> S
    where
        G: FnOnce() -> S,
    {
        match self {
            Success(success) => success,
            _ => op(),
        }
    }


    #[inline]
    pub fn failure(self) -> Option<F> {
        match self {
            Failure(x) => Some(x),
            _ => None
        }
    }

    #[inline]
    pub fn error(self) -> Option<E> {
        match self {
            Error(x) => Some(x),
            _ => None
        }
    }

    #[inline]
    pub fn into_result(self) -> Result<Result<S, F>, E> {
        match self {
            Success(s) => Ok(Ok(s)),
            Failure(f) => Ok(Err(f)),
            Error(e) => Err(e),
        }
    }
    
    #[inline]
    pub const fn as_ref(&self) -> Outcome<&S, &F, &E> {
        match *self {
            Success(ref x) => Success(x),
            Failure(ref x) => Failure(x),
            Error(ref x) => Error(x),
        }
    }

    #[inline]
    pub const fn as_mut(&mut self) -> Outcome<&mut S, &mut F, &mut E> {
        match *self {
            Success(ref mut x) => Success(x),
            Failure(ref mut x) => Failure(x),
            Error(ref mut x) => Error(x),
        }
    }

    #[inline]
    pub fn map<U, G>(self, op: G) -> Outcome<U, F, E>
    where G: FnOnce(S) -> U {
        match self {
            Success(x) => Success(op(x)),
            Failure(x) => Failure(x),
            Error(x) => Error(x)
        }
    }

    #[inline]
    pub fn map_or<U, G>(self, default: U, op: G) -> U
    where
        G: FnOnce(S) -> U,
    {
        match self {
            Success(t) => op(t),
            _ => default,
        }
    }

    #[inline]
    pub fn map_all<U, I, H, G>(self, success: G, failure: H, error: I) -> U
    where
        G: FnOnce(S) -> U,
        H: FnOnce(F) -> U,
        I: FnOnce(E) -> U,
    {
        match self {
            Success(s) => success(s),
            Failure(f) => failure(f),
            Error(e) => error(e)
        }
    }

    #[inline]
    pub fn map_or_default<U, G>(self, op: G) -> U
    where G: FnOnce(S) -> U,
        U: Default
    {
        match self {
            Success(s) => op(s),
            _ => U::default()
        }
    }

    #[inline]
    pub fn map_failure<U, G>(self, op: G) -> Outcome<S, U, E>
    where G: FnOnce(F) -> U {
        match self {
            Success(x) => Success(x),
            Failure(x) => Failure(op(x)),
            Error(x) => Error(x)
        }
    }

    #[inline]
    pub fn map_failure_or<U, G>(self, default: U, op: G) -> U
    where
        G: FnOnce(F) -> U,
    {
        match self {
            Failure(t) => op(t),
            _ => default,
        }
    }

    #[inline]
    pub fn map_failure_or_default<U, G>(self, op: G) -> U
    where G: FnOnce(F) -> U,
        U: Default
    {
        match self {
            Failure(s) => op(s),
            _ => U::default()
        }
    }

    #[inline]
    pub fn map_error<U, G>(self, op: G) -> Outcome<S, F, U>
    where G: FnOnce(E) -> U {
        match self {
            Success(x) => Success(x),
            Failure(x) => Failure(x),
            Error(x) => Error(op(x))
        }
    }

    #[inline]
    pub fn map_error_or<U, G>(self, default: U, op: G) -> U
    where
        G: FnOnce(E) -> U,
    {
        match self {
            Error(t) => op(t),
            _ => default,
        }
    }

    #[inline]
    pub fn map_error_or_default<U, G>(self, op: G) -> U
    where G: FnOnce(E) -> U,
        U: Default
    {
        match self {
            Error(s) => op(s),
            _ => U::default()
        }
    }

    #[inline]
    pub fn inspect<G>(self, f: G) -> Self
    where G: FnOnce(&S) {
        if let Success(ref s) = self {
            f(s);
        }
        self
    }

    #[inline]
    pub fn inspect_failure<G>(self, f: G) -> Self
    where G: FnOnce(&F) {
        if let Failure(ref s) = self {
            f(s);
        }
        self
    }

    #[inline]
    pub fn inspect_error<G>(self, f: G) -> Self
    where G: FnOnce(&E) {
        if let Error(ref s) = self {
            f(s);
        }
        self
    }

    #[inline]
    pub fn as_deref(&self) -> Outcome<&S::Target, &F, &E>
    where S: Deref {
        self.as_ref().map(Deref::deref)
    }

    #[inline]
    pub fn as_deref_mut(&mut self) -> Outcome<&mut S::Target, &mut F, &mut E>
    where S: DerefMut {
        self.as_mut().map(DerefMut::deref_mut)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, S> {
        Iter { inner: self.as_ref().success() }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, S> {
        IterMut { inner: self.as_mut().success() }
    }

    #[inline]
    pub fn expect(self, fail_msg: &str, err_msg: &str) -> S
    where
        E: Debug,
        F: Debug {
        match self {
            Success(x) => x,
            Failure(x) => panic!("[FAILURE] {fail_msg}: {x:?}"),
            Error(x) => panic!("[ERROR] {err_msg}: {x:?}")
        }
    }

    #[inline]
    pub fn unwrap(self) -> S
    where
        F: Debug,
        E: Debug {
        match self {
            Success(x) => x,
            Failure(failure) => panic!("[FAILURE] Called `Outcome::unwrap()` on a `Failure` value {failure:?}"),
            Error(error) => panic!("[ERROR] Called `Outcome::unwrap()` on an `Error` value {error:?}"),
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> S
        where S: Default,
    {
        match self {
            Success(x) => x,
            _ => Default::default(),
        }
    }
    
    #[inline]
    pub fn expect_failure(self, success_msg: &str, err_msg: &str) -> F
    where
        S: Debug,
        E: Debug
    {
        match self {
            Success(x) => panic!("[SUCCESS] {success_msg}: {x:?}"),
            Failure(x) => x,
            Error(x) => panic!("[ERROR] {err_msg}: {x:?}")
        }
    }
    
    #[inline]
    pub fn unwrap_failure(self) -> F
    where
        S: Debug,
        E: Debug {
        match self {
            Success(success) => panic!("[SUCCESS] Called `Outcome::unwrap_failure()` on a `Success` value {success:?}"),
            Failure(x) => x,
            Error(error) => panic!("[ERROR] Called `Outcome::unwrap_failure()` on an `Error` value {error:?}"),
        }
    }

    #[inline]
    pub fn unwrap_failure_or_default(self) -> F
        where F: Default,
    {
        match self {
            Failure(x) => x,
            _ => Default::default(),
        }
    }

    #[inline]
    pub fn expect_error(self, success_msg: &str, fail_msg: &str) -> E
    where
        S: Debug,
        F: Debug {
        match self {
            Success(x) => panic!("[SUCCESS] {success_msg}: {x:?}"),
            Failure(x) => panic!("[FAILURE] {fail_msg}: {x:?}"),
            Error(x) => x,
        }
    }

    #[inline]
    pub fn unwrap_error(self) -> E
    where
        S: Debug,
        F: Debug {
        match self {
            Success(success) => panic!("[SUCCESS] Called `Outcome::unwrap_error()` on a `Success` value {success:?}"),
            Failure(failure) => panic!("[FAILURE] Called `Outcome::unwrap_error()` on a `Failure` value {failure:?}"),
            Error(error) => error,
        }
    }

    #[inline]
    pub fn unwrap_err_or_default(self) -> E
        where E: Default,
    {
        match self {
            Error(x) => x,
            _ => Default::default(),
        }
    }

    #[inline]
    pub fn and<U>(self, other: Outcome<U, F, E>) -> Outcome<U, F, E> {
        match self {
            Success(_) => other,
            Failure(f) => Failure(f),
            Error(e) => Error(e),
        }
    }

    #[inline]
    pub fn and_then<U, G>(self, op: G) -> Outcome<U, F, E>
    where
        G: FnOnce(S) -> Outcome<U, F, E>,
    {
        match self {
            Success(s) => op(s),
            Failure(f) => Failure(f),
            Error(e) => Error(e)
        }
    }

    #[inline]
    pub fn or<U>(self, other: Outcome<S, U, E>) -> Outcome<S, U, E> {
        match self {
            Success(s) => Success(s),
            Failure(_) => other,
            Error(e) => Error(e),
        }
    }

    #[inline]
    pub fn or_else<U, O>(self, op: O) -> Outcome<S, U, E>
    where O: FnOnce(F) -> Outcome<S, U, E> {
        match self {
            Success(s) => Success(s),
            Failure(f) => op(f),
            Error(e) => Error(e)
        }
    }

    #[inline]
    pub fn or_error<U>(self, other: Outcome<S, F, U>) -> Outcome<S, F, U> {
        match self {
            Success(s) => Success(s),
            Failure(f) => Failure(f),
            Error(_) => other,
        }
    }

    #[inline]
    pub fn or_error_else<U, O>(self, op: O) -> Outcome<S, F, U>
    where O: FnOnce(E) -> Outcome<S, F, U> {
        match self {
            Success(s) => Success(s),
            Failure(f) => Failure(f),
            Error(e) => op(e)
        }
    }

    #[inline]
    pub fn unwrap_or(self, default: S) -> S {
        match self {
            Success(s) => s,
            _ => default
        }
    }
}

impl<S, F, E> Outcome<&S, F, E> {
    #[inline]
    pub fn copied(self) -> Outcome<S, F, E>
    where S: Copy {
        match self {
            Success(&s) => Success(s),
            Failure(f) => Failure(f),
            Error(e) => Error(e)
        }
    }

    #[inline]
    pub fn cloned(self) -> Outcome<S, F, E>
    where S: Clone {
        self.map(|s| s.clone())
    }
}

impl<S, F, E> Outcome<&mut S, F, E> {
    #[inline]
    pub fn copied(self) -> Outcome<S, F, E>
    where S: Copy {
        match self {
            Success(&mut s) => Success(s),
            Failure(f) => Failure(f),
            Error(e) => Error(e)
        }
    }

    #[inline]
    pub fn cloned(self) -> Outcome<S, F, E>
    where S: Clone {
        self.map(|s| s.clone())
    }
}

impl<S, F, E> Outcome<Option<S>, F, E> {
    #[inline]
    pub fn transpose(self) -> Option<Outcome<S, F, E>> {
        match self {
            Success(Some(s)) => Some(Success(s)),
            Success(None) => None,
            Failure(f) => Some(Failure(f)),
            Error(e) => Some(Error(e))
        }
    }
}

impl<S, F, E> Outcome<Outcome<S, F, E>, F, E> {
    #[inline]
    pub fn flatten(self) -> Outcome<S, F, E> {
        match self {
            Success(s) => s,
            Failure(f) => Failure(f),
            Error(e) => Error(e)
        }
    }
}

impl<S, F, E> Clone for Outcome<S, F, E>
where 
    S: Clone,
    F: Clone,
    E: Clone
{
    fn clone(&self) -> Self {
        match self {
            Success(s) => Success(s.clone()),
            Failure(f) => Failure(f.clone()),
            Error(e) => Error(e.clone()),
        }
    }
}


impl <S, F, E> IntoIterator for Outcome<S, F, E> {
    type Item = S;
    type IntoIter = IntoIter<S>;

    fn into_iter(self) -> IntoIter<S> {
        IntoIter { inner: self.success() }
    }
}

impl<'a, S, F, E> IntoIterator for &'a Outcome<S, F, E> {
    type Item = &'a S;
    type IntoIter = Iter<'a, S>;

    fn into_iter(self) -> Iter<'a, S> {
        let opt = match self {
            Success(s) => Some(s),
            Failure(_) => None,
            Error(_) => None,
        };
        Iter { inner: opt }
    }
}

impl<'a, S, F, E> IntoIterator for &'a mut Outcome<S, F, E> {
    type Item = &'a mut S;
    type IntoIter = IterMut<'a, S>;

    fn into_iter(self) -> IterMut<'a, S> {
        let opt = match self {
            Success(s) => Some(s),
            Failure(_) => None,
            Error(_) => None,
        };
        IterMut { inner: opt }
    }
}

impl<S, F, E> From<Result<Result<S, F>, E>> for Outcome<S, F, E> {
    fn from(value: Result<Result<S, F>, E>) -> Self {
        match value {
            Ok(Ok(s)) => Success(s),
            Ok(Err(f)) => Failure(f),
            Err(e) => Error(e),
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    inner: Option<&'a T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.inner.take()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.inner.take()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { inner: self.inner }
    }
}


#[derive(Debug)]
pub struct IntoIter<T> {
    inner: Option<T>
}

impl<T> Iterator for IntoIter<T>  {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.inner.take()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    inner: Option<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        self.inner.take()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        self.inner.take()
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FusedIterator for IterMut<'_, T> {}