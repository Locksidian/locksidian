//! Headers middleware.
//!
//! `AfterMiddleware` allowing us to set various HTTP headers to the API's `Response` object before
//! returning it to the client.
//!
//! The headers that are actually set by this middleware are the following:
//!
//! ```text
//! X-Content-Type-Options: "nosniff"
//! X-XSS-Protection: "1; mode=block"
//! X-Frame-Options: "deny"
//! Cache-Control: "no-cache, no-store, max-age=0, must-revalidate"
//! Pragma: "no-cache"
//! Expires: "0"
//! Content-Security-Policy: "default-src 'none'; frame-ancestors: 'none;'
//! Access-Control-Allow-Origin: "*"
//! ```

use time;

use iron::{Request, Response, IronResult};
use iron::middleware::AfterMiddleware;
use iron::headers::{
    CacheControl, CacheDirective,
    Pragma,
    Expires, HttpDate,
    AccessControlAllowOrigin,
};


pub struct HeadersMiddleware;

impl AfterMiddleware for HeadersMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set_raw("X-Content-Type-Options", vec![Vec::from("nosniff".as_bytes())]);
        res.headers.set_raw("X-XSS-Protection", vec![Vec::from("1; mode=block".as_bytes())]);
        res.headers.set_raw("X-Frame-Options", vec![Vec::from("deny".as_bytes())]);
        res.headers.set(CacheControl(vec![
            CacheDirective::NoCache,
            CacheDirective::NoStore,
            CacheDirective::MaxAge(0),
            CacheDirective::MustRevalidate
        ]));
        res.headers.set(Pragma::NoCache);
        res.headers.set(Expires(HttpDate(time::empty_tm())));
        res.headers.set_raw("Content-Security-Policy", vec![Vec::from(
            "default-src 'none'; frame-ancestors: 'none';".as_bytes()
        )]);
        res.headers.set(AccessControlAllowOrigin::Any);

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use iron::{Request, Response, IronResult, Handler, Headers, Chain, status, headers};
    use iron_test::request;

    use api::middleware::HeadersMiddleware;

    struct TestHandler;
    impl Handler for TestHandler {
        fn handle(&self, _: &mut Request) -> IronResult<Response> {
            Ok(Response::with((status::Ok, "Hello World!")))
        }
    }

    #[test]
    fn response_should_have_all_headers_set() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            Chain::new(TestHandler).link_after(HeadersMiddleware)
        ).unwrap();

        assert!(res.headers.has::<headers::CacheControl>());
        assert!(res.headers.has::<headers::Pragma>());
        assert!(res.headers.has::<headers::Expires>());
        assert!(res.headers.get_raw("X-Content-Type-Options").is_some());
        assert!(res.headers.get_raw("X-XSS-Protection").is_some());
        assert!(res.headers.get_raw("X-Frame-Options").is_some());
        assert!(res.headers.get_raw("Content-Security-Policy").is_some());
        assert!(res.headers.has::<headers::AccessControlAllowOrigin>());
    }
}