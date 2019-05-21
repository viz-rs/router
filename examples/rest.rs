extern crate futures;
extern crate hyper;
extern crate trek_router;

use futures::Future;
use hyper::server::Server;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, StatusCode};
use std::sync::Arc;
use trek_router::{Resource, ResourceOptions, Resources, Router};

type Params = Vec<(String, String)>;
type Handler = fn(Context) -> Body;

struct Context {
    request: Request<Body>,
    params: Params,
}

struct Geocoder {}

impl Resource for Geocoder {
    type Context = Context;
    type Body = Body;

    fn show(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Show!");
        Body::from(s)
    }

    fn create(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Create!");
        Body::from(s)
    }

    fn update(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Update!");
        Body::from(s)
    }

    fn delete(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Delete!");
        Body::from(s)
    }

    fn edit(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Edit!");
        Body::from(s)
    }

    fn new(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder New!");
        Body::from(s)
    }
}

struct Users {}

impl Resources for Users {
    type Context = Context;
    type Body = Body;

    fn index(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Users Index!");
        Body::from(s)
    }

    fn create(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Create!");
        Body::from(s)
    }

    fn new(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User New!");
        Body::from(s)
    }

    fn show(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Show, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }

    fn update(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Update, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }

    fn delete(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Delete, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }

    fn edit(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Edit, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let mut router = Router::<Handler>::new();

    router.resource("/geocoder", Geocoder::build(ResourceOptions::default()));
    router.resources("/users", Users::build(ResourceOptions::default()));

    let router = Arc::new(router);

    let routing = move || {
        let router = Arc::clone(&router);

        service_fn_ok(move |request| {
            let method = request.method().to_owned();
            let path = request.uri().path().to_owned();

            match router.find(&method, &path) {
                Some((handler, params)) => Response::new(handler(Context {
                    request,
                    params: params
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect(),
                })),
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
