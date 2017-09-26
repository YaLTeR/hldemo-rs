//! An error type wrapping nom's parsing errors and some glue between the two.

use nom::{self, error_to_list, IError, IResult, Needed};

use parse;

error_chain! {
    foreign_links {
        ParseError(parse::Error);
    }

    errors {
        NeedMoreBytes(count: Option<usize>) {
            description("need more bytes"),
            display("need {} more bytes", count.map(|c| format!("{}", c))
                                               .unwrap_or_else(|| "(unknown)".to_string())),
        }
    }
}

impl<I> From<nom::Err<I, parse::Error>> for Error {
    fn from(err: nom::Err<I, parse::Error>) -> Self {
        let v = error_to_list(&err);
        let mut iter = v.into_iter()
                        .rev()
                        .filter_map(|x| if let nom::ErrorKind::Custom(inner) = x {
                                        Some(inner)
                                    } else {
                                        None
                                    });

        let mut err = Error::from(iter.next().unwrap());
        for parse_error in iter {
            err = Error::with_chain(err, Error::from(parse_error));
        }

        err
    }
}

impl<I> From<IError<I, parse::Error>> for Error {
    fn from(err: IError<I, parse::Error>) -> Self {
        match err {
            IError::Incomplete(Needed::Size(count)) => ErrorKind::NeedMoreBytes(Some(count)).into(),
            IError::Incomplete(Needed::Unknown) => ErrorKind::NeedMoreBytes(None).into(),
            IError::Error(err) => err.into(),
        }
    }
}

/// Converts `nom::IResult<I, O, parse::Error>` into a `Result<O>`, preserving the parse error
/// chain.
///
/// # Panics
///
/// Panics if `result` is an `IError` and there's no `nom::ErrorKind::Custom(parse::Error)` in its
/// error chain.
// Can't have this as a From due to orphan rules.
pub fn iresult_into_result<I, O>(result: IResult<I, O, parse::Error>) -> Result<O> {
    result.to_full_result().map_err(|err| err.into())
}
