extern crate futures;
extern crate hyper;
extern crate trek_router;

use futures::Future;
use hyper::server::Server;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, StatusCode};
use std::sync::Arc;
use trek_router::Router;

type Params<'a> = Vec<(&'a str, &'a str)>;

type Handler = fn(Request<Body>, Params) -> Body;

fn v1_login(_: Request<Body>, _: Params) -> Body {
    Body::from("v1 login")
}

fn v1_submit(_req: Request<Body>, _: Params) -> Body {
    Body::from("v1 submit")
}

fn v1_read(_req: Request<Body>, _: Params) -> Body {
    Body::from("v1 read")
}

fn v2_login(_: Request<Body>, _: Params) -> Body {
    Body::from("v2 login")
}

fn v2_submit(_req: Request<Body>, _: Params) -> Body {
    Body::from("v2 submit")
}

fn v2_read(_req: Request<Body>, _: Params) -> Body {
    Body::from("v2 read")
}

fn users(_req: Request<Body>, _: Params) -> Body {
    Body::from("users")
}

fn foo(_: Request<Body>, _: Params) -> Body {
    Body::from("foo")
}

fn bar(_req: Request<Body>, _: Params) -> Body {
    Body::from("bar")
}

fn baz(_req: Request<Body>, _: Params) -> Body {
    Body::from("baz")
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let mut router = Router::<Handler>::new();

    router
        // scope v1
        .scope("/v1", |v1| {
            v1.get("/login", v1_login)
                .post("/submit", v1_submit)
                .delete("/read", v1_read);
        })
        // scope v2
        .scope("/v2", |v2| {
            v2.get("/login", v2_login)
                .post("/submit", v2_submit)
                .delete("/read", v2_read)
                // scope users
                .scope("users", |u| {
                    u.any("", users);
                });
        })
        .get("/foo", foo)
        .post("/bar", bar)
        .delete("/baz", baz);

    let router = Arc::new(router);

    let routing = move || {
        let router = Arc::clone(&router);

        service_fn_ok(move |req| {
            let method = req.method().to_owned();
            let path = req.uri().path().to_owned();

            match router.find(&method, &path) {
                Some((handler, params)) => Response::new(handler(req, params)),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .unwrap(),
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(routing)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
