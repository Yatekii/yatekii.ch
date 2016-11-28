extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::{Router};
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource};
use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

fn main() {

    /// HandlebarsEngine will look up all files with "./examples/templates/**/*.hbs"
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".html")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

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
    chain.link_after(hbse);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
