//! HTTP client specific macros.
//!
//! # Client body
//!
//! The `client_body!` macro allows you to automagically deserialize the content of the HTTP response
//! object as an initialized structure (whose type was provided to the macro).
//!
//! Example: deserialize the HTTP response.
//!
//! ```rust
//! let my_struct = client_body!(res, MyStruct);
//! ```

macro_rules! client_body {
    ($res:ident, $target:ty) => {
        {
            let mut client_body: String = String::new();
            match $res.read_to_string(&mut client_body) {
                Ok(_) => match ::serde_json::from_str::<$target>(&client_body) {
                    Ok(result) => Ok(result),
                    Err(err) => Err(err.to_string())
                },
                Err(err) => Err(err.to_string())
            }
        }
    }
}