extern crate iron;
extern crate time;

// iron
use iron::prelude::*;
// middleware
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
// mime types
// use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
// headers
use iron::modifiers::Header;
use iron::headers::{ContentType};
 
struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(req: &mut Request) -> IronResult<Response> {    
    // delta to check time spent
    let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
    let delta_ms = (delta as f64) / 1000000.0;
    
    // prepare an html string
    let mut msg_text: String = "<html><head><title></title></head><body>It's a pretty fast webserver.<br>\
                                And templating isn't TOO bad.<br>\
                                It took this long to get here (ms): ".to_owned();
    let msg_delta: &String = &delta_ms.to_string();
    msg_text.push_str(msg_delta);
    msg_text.push_str("</body></html>");
    
    println!("Request middle: {} ms", delta_ms);
    
    // set up response
    let html_res = Response::with((
        iron::status::Ok,
        msg_text,
        Header(ContentType::html())
    ));
    
    // execute response
    Ok(html_res)
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}
