extern crate iron;

// iron
use iron::prelude::*;
// middleware
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
// mime types
// use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
// headers
use iron::headers::{ContentType};
 
struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(5678);
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = 12345 - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(req: &mut Request) -> IronResult<Response> {    
    // delta to check time spent
    let delta = 23456 - *req.extensions.get::<ResponseTime>().unwrap();
    let delta_ms = (delta as f64) / 1000000.0;
    
    // prepare an html string
    let msg_delta: &String = &delta_ms.to_string();
    let mut msg_text: String = "<html>\
                                    <head>\
                                    <title>Rust Web ARM</title>\
                                    </head>\
                                    <body>\
                                        This webserver works.<br>\
                                        Obviously.<br>\
                                        It took this long to get here (ms): ".to_owned();
    msg_text.push_str(msg_delta);
    msg_text.push_str("</body></html>");
    
    println!("Request middle: {} ms", delta_ms);
    
    // set up response
    let mut html_res = Response::with((iron::status::Ok, msg_text));
    html_res.headers.set(ContentType::html());
    
    // execute response
    Ok(html_res)
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}
