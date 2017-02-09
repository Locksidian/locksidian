//! Node specific endpoint: `/node/*`

use iron::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueStruct {
    values: Vec<u64>,
    result: u64
}

pub fn home(req: &mut Request) -> IronResult<Response> {
    match body!(req, ValueStruct) {
        Ok(Some(mut value_struct)) => {
            value_struct.result = value_struct.values.iter().fold(0, |a, b| a + b);
            response!(Ok, value_struct)
        },
        Ok(None) => response!(BadRequest, {"error": "No content"}),
        Err(err) => response!(BadRequest, {"error": err.to_string()})
    }
}

#[cfg(test)]
mod test {
    use iron::{Headers, status};
    use iron_test::{request, response};

    use api::endpoints::node;

    #[test]
    fn home() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            &node::home
        ).unwrap();

        assert_eq!(res.status.unwrap(), status::BadRequest);

        let json_body = response::extract_body_to_string(res);
        assert_eq!(json_body, json!({
                "error": "No content"
            }).to_string()
        );
    }
}