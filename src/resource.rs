use http::Method;

lazy_static! {
    #[derive(Debug)]
    pub static ref RESOURCE_ACTIONS: &'static [(&'static str, &'static str, &'static Method)] = &[
        ("show", "", &Method::GET),
        ("create", "", &Method::POST),
        ("update", "", &Method::PATCH),
        ("update", "", &Method::PUT),
        ("delete", "", &Method::DELETE),
        ("edit", "edit", &Method::GET),
        ("new", "new", &Method::GET),
    ];
    pub static ref RESOURCES_ACTIONS: &'static [(&'static str, &'static str, &'static Method)] = &[
        ("index", "", &Method::GET),
        ("create", "", &Method::POST),
        ("new", "new", &Method::GET),
        ("show", ":id", &Method::GET),
        ("update", ":id", &Method::PATCH),
        ("update", ":id", &Method::PUT),
        ("delete", ":id", &Method::DELETE),
        ("edit", ":id/edit", &Method::GET)
    ];
}

#[derive(Default)]
pub struct ResourceOptions {
    only: Vec<&'static str>,
    except: Vec<&'static str>,
}

impl ResourceOptions {
    pub fn only(only: Vec<&'static str>) -> Self {
        ResourceOptions {
            only,
            except: vec![],
        }
    }

    pub fn except(except: Vec<&'static str>) -> Self {
        ResourceOptions {
            only: vec![],
            except,
        }
    }
}

pub trait Resource {
    type Context;
    type Body;

    fn show(ctx: Self::Context) -> Self::Body;
    fn create(ctx: Self::Context) -> Self::Body;
    fn update(ctx: Self::Context) -> Self::Body;
    fn delete(ctx: Self::Context) -> Self::Body;
    fn edit(ctx: Self::Context) -> Self::Body;
    fn new(ctx: Self::Context) -> Self::Body;

    fn build<'a>(
        opts: ResourceOptions,
    ) -> Vec<(
        (&'a str, &'a str, &'a Method),
        fn(Self::Context) -> Self::Body,
    )> {
        let mut ra: Vec<_> = RESOURCE_ACTIONS.to_vec();
        if !opts.only.is_empty() {
            ra = ra
                .into_iter()
                .take_while(|t| opts.only.contains(&t.0))
                .collect();
        }
        if !opts.except.is_empty() {
            ra = ra
                .into_iter()
                .skip_while(|t| opts.except.contains(&t.0))
                .collect();
        }
        let mut r: Vec<(
            (&'a str, &'a str, &'a Method),
            fn(Self::Context) -> Self::Body,
        )> = Vec::new();
        for t in ra {
            match t.0 {
                "show" => r.push((t, Self::show)),
                "create" => r.push((t, Self::create)),
                "update" => r.push((t, Self::update)),
                "delete" => r.push((t, Self::delete)),
                "edit" => r.push((t, Self::edit)),
                "new" => r.push((t, Self::new)),
                _ => unimplemented!(),
            }
        }
        r
    }
}

pub trait Resources {
    type Context;
    type Body;

    fn index(ctx: Self::Context) -> Self::Body;
    fn create(ctx: Self::Context) -> Self::Body;
    fn new(ctx: Self::Context) -> Self::Body;
    fn show(ctx: Self::Context) -> Self::Body;
    fn update(ctx: Self::Context) -> Self::Body;
    fn delete(ctx: Self::Context) -> Self::Body;
    fn edit(ctx: Self::Context) -> Self::Body;

    fn build<'a>(
        opts: ResourceOptions,
    ) -> Vec<(
        (&'a str, &'a str, &'a Method),
        fn(Self::Context) -> Self::Body,
    )> {
        let mut ra: Vec<_> = RESOURCES_ACTIONS.to_vec();
        if !opts.only.is_empty() {
            ra = ra
                .into_iter()
                .take_while(|t| opts.only.contains(&t.0))
                .collect();
        }
        if !opts.except.is_empty() {
            ra = ra
                .into_iter()
                .skip_while(|t| opts.except.contains(&t.0))
                .collect();
        }
        let mut r: Vec<(
            (&'a str, &'a str, &'a Method),
            fn(Self::Context) -> Self::Body,
        )> = Vec::new();
        for t in ra {
            match t.0 {
                "index" => r.push((t, Self::index)),
                "create" => r.push((t, Self::create)),
                "new" => r.push((t, Self::new)),
                "show" => r.push((t, Self::show)),
                "update" => r.push((t, Self::update)),
                "delete" => r.push((t, Self::delete)),
                "edit" => r.push((t, Self::edit)),
                _ => unimplemented!(),
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Router;

    #[test]
    fn resource() {
        type F = fn(ctx: ()) -> usize;
        let mut router = Router::<F>::new();

        struct Geocoder {}

        impl Resource for Geocoder {
            type Context = ();
            type Body = usize;

            fn show(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resource Show");
                0
            }

            fn create(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resource Create");
                1
            }

            fn update(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resource Update");
                2
            }

            fn delete(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resource Delete");
                3
            }

            fn edit(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resource Edit");
                4
            }

            fn new(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resource New");
                5
            }
        }

        router.resource("/geocoder", Geocoder::build(ResourceOptions::default()));

        struct Users {}

        impl Resources for Users {
            type Context = ();
            type Body = usize;

            fn index(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resources Index");
                0
            }

            fn create(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resources Create");
                1
            }

            fn new(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resources New");
                2
            }

            fn show(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resources Show");
                3
            }

            fn update(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resources Update");
                4
            }

            fn delete(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resources Delete");
                5
            }

            fn edit(ctx: Self::Context) -> Self::Body {
                println!("{}", "Resources Edit");
                6
            }
        }

        router.resources("/users", Users::build(ResourceOptions::default()));

        dbg!(&router);

        let r = router.find(&Method::GET, "/geocoder");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 0);
        assert_eq!(p, []);

        let r = router.find(&Method::POST, "/geocoder");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 1);
        assert_eq!(p, []);

        let r = router.find(&Method::PATCH, "/geocoder");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 2);
        assert_eq!(p, []);

        let r = router.find(&Method::PUT, "/geocoder");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 2);
        assert_eq!(p, []);

        let r = router.find(&Method::DELETE, "/geocoder");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 3);
        assert_eq!(p, []);

        let r = router.find(&Method::GET, "/geocoder/edit");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 4);
        assert_eq!(p, []);

        let r = router.find(&Method::GET, "/geocoder/new");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 5);
        assert_eq!(p, []);

        let r = router.find(&Method::GET, "/users");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 0);
        assert_eq!(p, []);

        let r = router.find(&Method::POST, "/users");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 1);
        assert_eq!(p, []);

        let r = router.find(&Method::GET, "/users/new");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 2);
        assert_eq!(p, []);

        let r = router.find(&Method::GET, "/users/1");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 3);
        assert_eq!(p, [("user_id", "1")]);

        let r = router.find(&Method::PATCH, "/users/1");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 4);
        assert_eq!(p, [("user_id", "1")]);

        let r = router.find(&Method::PUT, "/users/1");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 4);
        assert_eq!(p, [("user_id", "1")]);

        let r = router.find(&Method::DELETE, "/users/1");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 5);
        assert_eq!(p, [("user_id", "1")]);

        let r = router.find(&Method::GET, "/users/1/edit");
        assert!(r.is_some());
        let (h, p) = r.unwrap();
        assert_eq!(h(()), 6);
        assert_eq!(p, [("user_id", "1")]);
    }
}
