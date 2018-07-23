macro_rules! add_parse_error {
    ($i:expr, $err:ident, $($t:tt)*) => (
        add_return_error!($i, ::nom::ErrorKind::Custom($crate::parse::Error::$err), $($t)*)
    )
}

// Nom PR: https://github.com/Geal/nom/pull/821
macro_rules! map_res_err_ (
  // Internal parser, do not use directly
  (__impl $i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    {
      use ::nom::lib::std::result::Result::*;
      use ::nom::{Context, Convert, Err};

      let i_ = $i.clone();
      match $submac!(i_, $($args)*) {
        Ok((i,o)) => {
          match $submac2!(o, $($args2)*) {
            Ok(output) => Ok((i, output)),
            Err(e) => {
              let e = Context::convert(Context::Code($i, ::nom::ErrorKind::Custom(e)));
              Err(Err::Error(error_node_position!($i, ::nom::ErrorKind::MapRes, e)))
            },
          }
        },
        Err(e) => Err(e),
      }
    }
  );
  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    map_res_err_!(__impl $i, $submac!($($args)*), call!($g));
  );
  ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    map_res_err_!(__impl $i, $submac!($($args)*), $submac2!($($args2)*));
  );
  ($i:expr, $f:expr, $g:expr) => (
    map_res_err_!(__impl $i, call!($f), call!($g));
  );
  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    map_res_err_!(__impl $i, call!($f), $submac!($($args)*));
  );
);
