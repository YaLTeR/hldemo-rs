macro_rules! add_parse_error {
    ($i:expr, $err:ident, $($t:tt)*) => (
        add_return_error!($i, ::nom::ErrorKind::Custom($crate::parse::Error::$err), $($t)*)
    )
}
