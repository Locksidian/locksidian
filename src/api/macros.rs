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
//!
//! # Route parameter
//!
//! The `route_param!` macro allows you to easily access the value of a dynamic parameter of your
//! Iron route.
//!
//! Usage:
//!
//! ```rust
//! match route_param!(req, "my_param") {
//! 	Some(my_param) => ...,
//! 	None => ...
//! }
//! ```

macro_rules! body {
    ($req:ident) => {
        $req.get::<::bodyparser::Json>();
    };

    ($req:ident, $target:ty) => {
        $req.get::<::bodyparser::Struct<$target>>();
    };
}

macro_rules! body_raw {
    ($req:ident) => {
        $req.get::<::bodyparser::Raw>();
    };
}

macro_rules! response {
    ($status:ident, $payload:tt) => {
        if ::iron::status::$status == ::iron::status::Ok {
            Ok(::iron::Response::with((
                ::iron::status::$status,
                json!($payload).to_string()
            )))
        }
        else {
            error!($status, $payload)
        }
    };

    ($status:ident, $payload:ident) => {
        if ::iron::status::$status == ::iron::status::Ok {
            Ok(::iron::Response::with((
                ::iron::status::$status,
                ::serde_json::to_string(&$payload)
            )))
        }
        else {
            error!($status, $payload)
        }
    };
}

macro_rules! error {
    ($status:ident, $payload:tt) => {
        Err(IronError::new(
            ::error::LocksidianError::new(
                json!($payload).to_string()
            ),
            ::iron::status::$status
        ))
    };

    ($status:ident, $payload:ident) => {
        Err(IronError::new(
            ::error::LocksidianError::new(
                ::serde_json::to_string(&$payload)
            ),
            ::iron::status::$status
        ))
    };
}

macro_rules! route_param {
	($req:ident, $param:tt) => {
		$req.extensions.get::<::router::Router>().unwrap().find($param);
	}
}