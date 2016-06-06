extern crate iron;
extern crate router;
extern crate urlencoded;

use self::iron::{Request, Response, IronResult, Url};
use self::iron::Plugin;
use self::iron::headers;
use self::iron::modifiers::{Redirect, Header};
use self::iron::status;
use self::router::Router;
use self::urlencoded::UrlEncodedQuery;


const URL_QUERY_PARAM: &'static str = "url";


pub fn to(req: &mut Request) -> IronResult<Response> {
    let url = iexpect!(req.get_ref::<UrlEncodedQuery>()
        .ok()
        .and_then(|hashmap| hashmap.get(URL_QUERY_PARAM))
        .and_then(|vals| vals.first())
        .and_then(|url| Url::parse(url).ok()));

    Ok(Response::with((status::Found, Redirect(url))))
}

pub fn relative(req: &mut Request) -> IronResult<Response> {

    let mut code = itry!(req.extensions
        .get::<Router>()
        .unwrap()
        .find("n")
        .unwrap_or("1")
        .parse::<u16>());

    code = code - 1;

    let url = if code <= 0 {
        String::from("/")
    } else {
        format!("/relative-redirect/{}", code)
    };

    Ok(Response::with((status::Found, Header(headers::Location(url)))))
}
