//! Node specific endpoint: `/node/*`

use iron::prelude::*;
use iron::status;
use bodyparser::Struct;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueStruct {
    values: Vec<u64>,
    result: u64
}

pub fn home(req: &mut Request) -> IronResult<Response> {
    match req.get::<Struct<ValueStruct>>() {
        Ok(Some(value_struct)) => {
            let mut sum = 0;

            for i in value_struct.values {
                sum = sum + i
            }

            println!("Struct sum: {:?}", sum);
        },
        Ok(None) => println!("No structure"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with((status::Ok, json!({
            "message": "Hello Node!"
        }).to_string()
    )))
}

#[cfg(test)]
mod test {
    use iron::Headers;
    use iron_test::{request, response};

    use api::endpoints::node;

    #[test]
    fn home() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            &node::home
        ).unwrap();

        let json_body = response::extract_body_to_string(res);
        assert_eq!(json_body, json!({
                "message": "Hello Node!"
            }).to_string()
        );
    }
}