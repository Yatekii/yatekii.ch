extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::{status, AfterMiddleware};
use router::{Router};
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource};
use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("{}", err);
        Err(err)
    }
}

fn main() {

    fn index(_: &mut Request) -> IronResult<Response> {
        let mut resp = Response::new();

        let mut data = BTreeMap::new();
        data.insert("year".to_string(), "2015".to_json());
        println!("can work data");
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
        println!("can create response");
        Ok(resp)
    }

    let mut router = Router::new();
    router.get("/", index, "index");
    let mut chain = Chain::new(router);
    /// HandlebarsEngine will look up all files with "./examples/templates/**/*.hbs"
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("/Users/yatekii/repos/yatekii.ch/target/debug/templates", ".html")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }
    chain.link_after(hbse);
    chain.link_after(ErrorReporter);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
