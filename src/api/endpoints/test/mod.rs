//! Test endpoints exposed at `/test`

mod value;

use iron::prelude::*;
use persistence::prelude::*;
use api::client::*;

/// Example structure used for (de)serialization showcase.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueStruct {
    values: Vec<u64>,
    result: u64
}

/// Simple example of how you would use the `body!` and `response!` macros in order to (de)serialize
/// data structures from the HTTP request and to the HTTP response.
pub fn simple_add_values(req: &mut Request) -> IronResult<Response> {
    match body!(req, ValueStruct) {
        Ok(Some(mut value_struct)) => {
            value_struct.result = value_struct.values.iter().fold(0, |a, b| a + b);
            response!(Ok, value_struct)
        },
        Ok(None) => response!(BadRequest, {"error": "No content"}),
        Err(err) => response!(BadRequest, {"error": err.to_string()})
    }
}

/// Simple example of how you would interact with the connection pool in order to retrieve a connection
/// from the HTTP `Request` object, and how to use a `Repository` to interact with persisted entities.
pub fn persisted_add_values(req: &mut Request) -> IronResult<Response> {
    match req.get_connection() {
        Ok(connection) => {
            let repository = value::ValueRepository::new(&*connection);

            match repository.get_all() {
                Some(values) => {
                    let sum = values.iter()
                                    .map(|v| v.value)
                                    .fold(0, |a, b| a + b);

                    response!(Ok, {"sum": sum, "values": values})
                },
                None => response!(NoContent, {})
            }
        },
        Err(msg) => response!(InternalServerError, {"error": msg})
    }
}

/// Simple HTTP client example.
pub fn http_client(req: &mut Request) -> IronResult<Response> {
    match req.get_client() {
        Ok(client) => {
            let mut res = client.get("http://jsonplaceholder.typicode.com/posts/1").send().unwrap();
            let mut body: String = String::new();
            res.read_to_string(&mut body).unwrap();

            response!(Ok, {
                "status": res.status.to_string(),
                "body": body
            })
        },
        Err(msg) => response!(InternalServerError, {"error": msg})
    }
}

#[cfg(test)]
mod test {
    use iron::{Headers, status};
    use iron_test::{request, response};

    use api::endpoints::test;

    #[test]
    fn home() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            &test::simple_add_values
        ).unwrap();

        assert_eq!(res.status.unwrap(), status::BadRequest);

        let json_body = response::extract_body_to_string(res);
        assert_eq!(json_body, json!({
                "error": "No content"
            }).to_string()
        );
    }
}