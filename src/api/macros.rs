//! API specific macros.

macro_rules! body {
    ($req:ident) => {
        $req.get::<::bodyparser::Json>();
    };
    ($req:ident, $target:ty) => {
        $req.get::<::bodyparser::Struct<$target>>();
    };
}