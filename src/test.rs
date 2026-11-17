
#[cfg(test)]
mod tests {
    
    use crate::Outcome::{self, *};

    // -------------------------
    // Variant and query tests
    // -------------------------

    #[test]
    fn identifies_variants() {
        assert!(Success::<i32, (), ()>(1).is_success());
        assert!(Failure::<i32, (), ()>(()).is_failure());
        assert!(Error::<i32, (), ()>(()).is_error());

        assert!(!Failure::<i32, (), ()>(()).is_success());
        assert!(!Error::<i32, (), ()>(()).is_failure());
        assert!(!Success::<i32, (), ()>(1).is_error());
    }

    #[test]
    fn variant_predicates_work() {
        assert!(Success::<i32, (), ()>(42).is_success_and(|x| x == 42));
        assert!(!Success::<i32, (), ()>(42).is_success_and(|x| x == 0));

        assert!(Failure::<i32, i32, ()>(42).is_failure_and(|x| x == 42));
        assert!(!Failure::<i32, i32, ()>(42).is_failure_and(|x| x == 0));

        assert!(Error::<i32, (), i32>(42).is_error_and(|x| x == 42));
        assert!(!Error::<i32, (), i32>(42).is_error_and(|x| x == 0));
    }

    // -------------------------
    // Unwrapping tests
    // -------------------------

    #[test]
    fn unwrap_returns_success() {
        let outcome: Outcome<i32, &str, &str> = Success(42);
        assert_eq!(outcome.unwrap(), 42);
    }

    #[test]
    fn unwrap_failure_returns_failure() {
        let outcome: Outcome<i32, &str, &str> = Failure("invalid input");
        assert_eq!(outcome.unwrap_failure(), "invalid input");
    }

    #[test]
    fn unwrap_error_returns_error() {
        let outcome: Outcome<i32, &str, &str> = Error("io error");
        assert_eq!(outcome.unwrap_error(), "io error");
    }

    #[test]
    fn unwrap_or_uses_default_for_non_success() {
        assert_eq!(Success::<i32, (), ()>(42).unwrap_or(0), 42);
        assert_eq!(Failure::<i32, (), ()>(()).unwrap_or(0), 0);
        assert_eq!(Error::<i32, (), ()>(()).unwrap_or(0), 0);
    }

    #[test]
    fn unwrap_or_default_works() {
        assert_eq!(Success::<i32, (), ()>(42).unwrap_or_default(), 42);
        assert_eq!(Failure::<i32, (), ()>(()).unwrap_or_default(), 0);
        assert_eq!(Error::<i32, (), ()>(()).unwrap_or_default(), 0);
    }

    #[test]
    fn other_unwrap_defaults_work() {
        assert_eq!(Success::<i32, i32, ()>(42).unwrap_failure_or(0), 0);
        assert_eq!(Failure::<i32, i32, ()>(42).unwrap_failure_or(0), 42);
    }

    // -------------------------
    // Conversion tests
    // -------------------------

    #[test]
    fn converts_result_into_outcome() {
        let success: Outcome<i32, &str, u8> = Ok(42).into();
        let error: Outcome<i32, &str, u8> = Err::<i32, u8>(1).into();

        assert_eq!(success, Success(42));
        assert_eq!(error, Error(1));
    }

    #[test]
    fn converts_nested_result_into_outcome() {
        let success: Outcome<i32, &str, u8> = Ok(Ok(42)).into();
        let failure: Outcome<i32, &str, u8> = Ok(Err("invalid")).into();
        let error: Outcome<i32, &str, u8> = Err::<i32, u8>(1).into();

        assert_eq!(success, Success(42));
        assert_eq!(failure, Failure("invalid"));
        assert_eq!(error, Error(1));
    }

    #[test]
    fn converts_outcome_into_nested_result() {
        assert_eq!(
            Success::<i32, &str, u8>(42).into_result(),
            Ok(Ok(42))
        );

        assert_eq!(
            Failure::<i32, &str, u8>("invalid").into_result(),
            Ok(Err("invalid"))
        );

        assert_eq!(
            Error::<i32, &str, u8>(1).into_result(),
            Err(1)
        );
    }

    // -------------------------
    // Option conversion tests
    // -------------------------

    #[test]
    fn converts_variants_into_options() {
        let success: Outcome<i32, &str, u8> = Success(42);
        let failure: Outcome<i32, &str, u8> = Failure("invalid");
        let error: Outcome<i32, &str, u8> = Error(1);

        assert_eq!(success.success(), Some(42));
        assert_eq!(failure.success(), None);
        assert_eq!(error.success(), None);

        assert_eq!(Success::<i32, &str, u8>(42).failure(), None);
        assert_eq!(Failure::<i32, &str, u8>("invalid").failure(), Some("invalid"));

        assert_eq!(Success::<i32, &str, u8>(42).error(), None);
        assert_eq!(Error::<i32, &str, u8>(1).error(), Some(1));
    }

    // -------------------------
    // Mapping tests
    // -------------------------

    #[test]
    fn maps_success() {
        let outcome: Outcome<i32, &str, &str> = Success(21);

        assert_eq!(outcome.map(|x| x * 2), Success(42));
    }

    #[test]
    fn maps_failure() {
        let outcome: Outcome<i32, i32, &str> = Failure(21);

        assert_eq!(outcome.map_failure(|x| x * 2), Failure(42));
    }

    #[test]
    fn maps_error() {
        let outcome: Outcome<i32, &str, i32> = Error(21);

        assert_eq!(outcome.map_error(|x| x * 2), Error(42));
    }

    #[test]
    fn map_preserves_other_variants() {
        let failure: Outcome<i32, &str, &str> = Failure("invalid");
        let error: Outcome<i32, &str, &str> = Error("runtime");

        assert_eq!(failure.map(|x| x * 2), Failure("invalid"));
        assert_eq!(error.map(|x| x * 2), Error("runtime"));
    }

    #[test]
    fn map_all_handles_every_variant() {
        let success: Outcome<i32, &str, &str> = Success(10);
        let failure: Outcome<i32, &str, &str> = Failure("bad");
        let error: Outcome<i32, &str, &str> = Error("crash");

        assert_eq!(
            success.map_all(|x| x * 2, |_| 0, |_| 0),
            20
        );

        assert_eq!(
            failure.map_all(|_| 0, |_| 30, |_| 0),
            30
        );

        assert_eq!(
            error.map_all(|_| 0, |_| 0, |_| 40),
            40
        );
    }

    // -------------------------
    // Boolean operation tests
    // -------------------------

    #[test]
    fn and_uses_other_only_on_success() {
        assert_eq!(
            Success::<i32, &str, &str>(1).and(Success(2)),
            Success(2)
        );

        assert_eq!(
            Failure::<i32, &str, &str>("bad").and(Success(2)),
            Failure("bad")
        );

        assert_eq!(
            Error::<i32, &str, &str>("crash").and(Success(2)),
            Error("crash")
        );
    }

    #[test]
    fn and_then_runs_only_on_success() {
        let success = Success::<i32, &str, &str>(21)
            .and_then(|x| Success(x * 2));

        assert_eq!(success, Success(42));

        let failure = Failure::<i32, &str, &str>("bad")
            .and_then(|x| Success(x * 2));

        assert_eq!(failure, Failure("bad"));
    }

    #[test]
    fn or_uses_other_only_on_failure() {
        assert_eq!(
            Success::<i32, &str, &str>(1).or(Success::<i32, u8, &str>(2)),
            Success(1)
        );

        assert_eq!(
            Failure::<i32, &str, &str>("bad").or(Success::<i32, u8, &str>(2)),
            Success(2)
        );

        assert_eq!(
            Error::<i32, &str, &str>("crash").or(Success::<i32, u8, &str>(2)),
            Error("crash")
        );
    }

    #[test]
    fn or_else_runs_only_on_failure() {
        let outcome = Failure::<i32, &str, &str>("bad")
            .or_else(|_| Success::<i32, u8, &str>(42));

        assert_eq!(outcome, Success(42));
    }

    #[test]
    fn or_error_uses_other_only_on_error() {
        assert_eq!(
            Success::<i32, &str, u8>(1).or_error(Success::<i32, &str, i32>(2)),
            Success(1)
        );

        assert_eq!(
            Failure::<i32, &str, u8>("bad").or_error(Success::<i32, &str, i32>(2)),
            Failure("bad")
        );

        assert_eq!(
            Error::<i32, &str, u8>(1).or_error(Success::<i32, &str, i32>(2)),
            Success(2)
        );
    }

    #[test]
    fn or_error_else_runs_only_on_error() {
        let outcome = Error::<i32, &str, u8>(1)
            .or_error_else(|_| Success::<i32, &str, i32>(42));

        assert_eq!(outcome, Success(42));
    }

    // -------------------------
    // Reference tests
    // -------------------------

    #[test]
    fn as_ref_preserves_variant() {
        let outcome: Outcome<String, String, String> =
            Success("hello".to_owned());

        assert_eq!(outcome.as_ref(), Success(&"hello".to_owned()));
    }

    #[test]
    fn as_mut_allows_mutation() {
        let mut outcome: Outcome<i32, (), ()> = Success(10);

        if let Success(value) = outcome.as_mut() {
            *value = 42;
        }

        assert_eq!(outcome, Success(42));
    }

    #[test]
    fn as_deref_works() {
        let outcome: Outcome<Box<i32>, (), ()> = Success(Box::new(42));

        assert_eq!(outcome.as_deref(), Success(&42));
    }

    #[test]
    fn as_deref_mut_works() {
        let mut outcome: Outcome<Box<i32>, (), ()> = Success(Box::new(10));

        if let Success(value) = outcome.as_deref_mut() {
            *value = 42;
        }

        assert_eq!(outcome, Success(Box::new(42)));
    }

    #[test]
    fn copied_and_cloned_work() {
        let value = 42;

        assert_eq!(Success::<&i32, (), ()>(&value).copied(), Success(42));

        let value = String::from("hello");

        assert_eq!(
            Success::<&String, (), ()>(&value).cloned(),
            Success(String::from("hello"))
        );
    }

    // -------------------------
    // Iterator tests
    // -------------------------

    #[test]
    fn iterates_only_success_values() {
        let outcome: Outcome<i32, (), ()> = Success(42);

        assert_eq!(outcome.iter().collect::<Vec<_>>(), vec![&42]);

        let outcome: Outcome<i32, (), ()> = Failure(());

        assert_eq!(outcome.iter().count(), 0);
        
        let outcome: Outcome<i32, (), ()> = Error(());

        assert_eq!(outcome.iter().count(), 0);
    }

    #[test]
    fn iter_mut_allows_mutation() {
        let mut outcome: Outcome<i32, (), ()> = Success(10);

        for value in outcome.iter_mut() {
            *value = 42;
        }

        assert_eq!(outcome, Success(42));
    }

    #[test]
    fn into_iterator_yields_only_success() {
        let outcome: Outcome<i32, (), ()> = Success(42);

        assert_eq!(outcome.into_iter().collect::<Vec<_>>(), vec![42]);

        let outcome: Outcome<i32, (), ()> = Failure(());

        assert!(outcome.into_iter().next().is_none());
    }

    #[test]
    fn iterator_has_exact_size() {
        let outcome: Outcome<i32, (), ()> = Success(42);
        let mut iter = outcome.iter();

        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(&42));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
    }

    // -------------------------
    // Transpose and flatten tests
    // -------------------------

    #[test]
    fn transpose_works() {
        let success: Outcome<Option<i32>, &str, &str> = Success(Some(42));
        let empty: Outcome<Option<i32>, &str, &str> = Success(None);
        let failure: Outcome<Option<i32>, &str, &str> = Failure("bad");
        let error: Outcome<Option<i32>, &str, &str> = Error("crash");

        assert_eq!(success.transpose(), Some(Success(42)));
        assert_eq!(empty.transpose(), None);
        assert_eq!(failure.transpose(), Some(Failure("bad")));
        assert_eq!(error.transpose(), Some(Error("crash")));
    }

    #[test]
    fn flatten_works() {
        let success: Outcome<Outcome<i32, &str, &str>, &str, &str> =
            Success(Success(42));

        let failure: Outcome<Outcome<i32, &str, &str>, &str, &str> =
            Success(Failure("bad"));

        let error: Outcome<Outcome<i32, &str, &str>, &str, &str> =
            Success(Error("crash"));

        assert_eq!(success.flatten(), Success(42));
        assert_eq!(failure.flatten(), Failure("bad"));
        assert_eq!(error.flatten(), Error("crash"));
    }

    // -------------------------
    // Collection tests
    // -------------------------

    #[test]
    fn collects_successes() {
        let values: Outcome<Vec<i32>, &str, &str> =
            [Success(1), Success(2), Success(3)].into_iter().collect();

        assert_eq!(values, Success(vec![1, 2, 3]));
    }

    #[test]
    fn collection_stops_on_failure() {
        let values: Outcome<Vec<i32>, &str, &str> =
            [Success(1), Failure("bad"), Success(3)].into_iter().collect();

        assert_eq!(values, Failure("bad"));
    }

    #[test]
    fn collection_stops_on_error() {
        let values: Outcome<Vec<i32>, &str, &str> =
            [Success(1), Error("crash"), Success(3)].into_iter().collect();

        assert_eq!(values, Error("crash"));
    }

    #[test]
    fn sums_successes() {
        let result: Outcome<i32, &str, &str> =
            [Success(1), Success(2), Success(3)].into_iter().sum();

        assert_eq!(result, Success(6));
    }

    #[test]
    fn products_successes() {
        let result: Outcome<i32, &str, &str> =
            [Success(2), Success(3), Success(4)].into_iter().product();

        assert_eq!(result, Success(24));
    }

    // -------------------------
    // Clone tests
    // -------------------------

    #[test]
    fn clones_all_variants() {
        let success: Outcome<String, String, String> =
            Success("success".into());

        let failure: Outcome<String, String, String> =
            Failure("failure".into());

        let error: Outcome<String, String, String> =
            Error("error".into());

        assert_eq!(success.clone(), success);
        assert_eq!(failure.clone(), failure);
        assert_eq!(error.clone(), error);
    }
}
