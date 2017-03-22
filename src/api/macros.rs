//! API specific macros.
//!
//! # Body
//!
//! The `body!` macro allows you to automagically deserialize the content of the HTTP request body,
//! either in a JSON object or as an initialized structure (whose type was provided to the macro).
//!
//! Example 1: deserialize as a JSON object
//!
//! ```rust
//! let json = body!(req);
//! ```
//!
//! Example 2: deserialize a structure
//!
//! ```rust
//! let my_struct: MyStruct = body!(req, MyStruct);
//! ```
//!
//! # Response
//!
//! The `response!` macro allows you to easily create an HTTP response by specifying the HTTP status
//! code and either a JSON object, a serializable structure, or a mix of these two.
//!
//! Example 1: return a simple JSON object
//!
//! ```rust
//! response!(Ok, {"msg": "Hello World!"});
//! ```
//!
//! Example 2: return a serialized structure
//!
//! ```rust
//! response!(Ok, my_struct);
//! ```
//!
//! Example 3: return a JSON object containing serialized structures.
//!
//! ```rust
//! response!(Ok, {
//!     "msg": "Hello World!",
//!     "struct": my_struct
//! });
//! ```

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