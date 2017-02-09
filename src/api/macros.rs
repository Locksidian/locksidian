//! API specific macros.

macro_rules! body {
    ($req:ident) => {
        $req.get::<::bodyparser::Json>();
    };
    ($req:ident, $target:ty) => {
        $req.get::<::bodyparser::Struct<$target>>();
    };
}

macro_rules! response {
    ($status:ident, $payload:tt) => {
        Ok(::iron::Response::with((
            ::iron::status::$status,
            json!($payload).to_string()
        )))
    };

    ($status:ident, $payload:ident) => {
        Ok(::iron::Response::with((
            ::iron::status::$status,
            ::serde_json::to_string(&$payload)
        )))
    };
}