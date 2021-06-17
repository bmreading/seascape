/// Count items in a list of items within a macro, taken from here:
/// https://danielkeep.github.io/tlborm/book/blk-counting.html
#[doc(hidden)]
#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! count_items {
    ($($item:expr),*) => {<[()]>::len(&[$($crate::replace_expr!($item ())),*])};
}

#[doc(hidden)]
#[macro_export]
macro_rules! internal_build_map {
    (/* required */, $map:ident, $key:expr, $val:expr) => {
        $map.insert($key, $val);
    };
    (optional, $map:ident, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $map.insert($key, val);
        }
    };
}
/// Constructs a HashMap<&str, &str>
/// The syntax is the following:
///
///   [optional] "key": value
/// https://github.com/ramsayleung/rspotify
#[doc(hidden)]
#[macro_export]
macro_rules! build_map {
    (
        $(
            $( $kind:ident )? $key:literal : $val:expr
        ),+ $(,)?
    ) => {{
        let mut params = $crate::http::QueryParamMap::with_capacity(
            $crate::count_items!($( $key ),*)
        );
        $(
            $crate::internal_build_map!(
                $( $kind )?,
                params,
                $key,
                $val
            );
        )+
        params
    }};
}
