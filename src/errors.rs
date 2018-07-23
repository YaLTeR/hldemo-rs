//! An error type wrapping nom's parsing errors and some glue between the two.

use nom::{self, Context, Needed};

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

impl<I> From<Context<I, parse::Error>> for Error {
    fn from(err: Context<I, parse::Error>) -> Self {
        let v = match err {
            Context::Code(i, kind) => vec![(i, kind)],
            Context::List(vec) => vec,
        };

        let mut iter = v.into_iter().filter_map(|(_, x)| {
                                                    if let nom::ErrorKind::Custom(inner) = x {
                                                        Some(inner)
                                                    } else {
                                                        None
                                                    }
                                                });

        let mut err = Error::from(iter.next().unwrap());
        for parse_error in iter {
            err = Error::with_chain(err, Error::from(parse_error));
        }

        err
    }
}

impl<I> From<nom::Err<I, parse::Error>> for Error {
    fn from(err: nom::Err<I, parse::Error>) -> Self {
        match err {
            nom::Err::Incomplete(Needed::Size(count)) => {
                ErrorKind::NeedMoreBytes(Some(count)).into()
            }
            nom::Err::Incomplete(Needed::Unknown) => ErrorKind::NeedMoreBytes(None).into(),
            nom::Err::Error(err) | nom::Err::Failure(err) => err.into(),
        }
    }
}
