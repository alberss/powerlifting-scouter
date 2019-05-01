extern crate actix_web;
use actix_web::{App, HttpRequest, http::Method};

fn make_app() {
    fn index(_req: &HttpRequest) -> &'static str {
        "Hello world!"
    }

    let _app = App::new()
                .prefix("/app")
                .resource("/index.html", |r| r.method(Method::GET).f(index))
                .finish();
}

fn main() {
    make_app();
    // server::new(|| App::new().resource("/", |r| r.f(index)))
    //     .bind("127.0.0.1:8088")
    //     .unwrap()
    //     .run();
}

