use http::Method;
// use inflector::string::pluralize::to_plural;
use inflector::string::singularize::to_singular;

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
        ("show", ":{id}", &Method::GET),
        ("update", ":{id}", &Method::PATCH),
        ("update", ":{id}", &Method::PUT),
        ("delete", ":{id}", &Method::DELETE),
        ("edit", " :{id}/edit", &Method::GET)
    ];
}

pub trait Resource: Send + Sync + 'static {
    type Handler;
    type Context;
    type Body;

    fn build<'a>() -> Vec<(
        (&'a str, &'a str, &'a Method),
        fn(Self::Context) -> Self::Body,
    )> {
        let mut r: Vec<(
            (&'a str, &'a str, &'a Method),
            fn(Self::Context) -> Self::Body,
        )> = Vec::new();
        for t in RESOURCE_ACTIONS.iter() {
            match t.0 {
                "show" => r.push((*t, Self::show)),
                "create" => r.push((*t, Self::create)),
                "update" => r.push((*t, Self::update)),
                "delete" => r.push((*t, Self::delete)),
                "edit" => r.push((*t, Self::edit)),
                "new" => r.push((*t, Self::new)),
                _ => unimplemented!(),
            }
        }
        r
    }

    fn show(ctx: Self::Context) -> Self::Body;
    fn create(ctx: Self::Context) -> Self::Body;
    fn update(ctx: Self::Context) -> Self::Body;
    fn delete(ctx: Self::Context) -> Self::Body;
    fn edit(ctx: Self::Context) -> Self::Body;
    fn new(ctx: Self::Context) -> Self::Body;
}

pub trait Resources {
    type Context;
    type Body;

    fn build<'a>() -> Vec<(
        (&'a str, &'a str, &'a Method),
        fn(Self::Context) -> Self::Body,
    )> {
        let mut r: Vec<(
            (&'a str, &'a str, &'a Method),
            fn(Self::Context) -> Self::Body,
        )> = Vec::new();
        for t in RESOURCE_ACTIONS.iter() {
            match t.0 {
                "index" => r.push((*t, Self::index)),
                "create" => r.push((*t, Self::create)),
                "new" => r.push((*t, Self::new)),
                "show" => r.push((*t, Self::show)),
                "update" => r.push((*t, Self::update)),
                "delete" => r.push((*t, Self::delete)),
                "edit" => r.push((*t, Self::edit)),
                _ => unimplemented!(),
            }
        }
        r
    }

    fn index(ctx: Self::Context) -> Self::Body;
    fn create(ctx: Self::Context) -> Self::Body;
    fn new(ctx: Self::Context) -> Self::Body;
    fn show(ctx: Self::Context) -> Self::Body;
    fn update(ctx: Self::Context) -> Self::Body;
    fn delete(ctx: Self::Context) -> Self::Body;
    fn edit(ctx: Self::Context) -> Self::Body;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Router;

    #[test]
    fn resource() {
        type F = fn(ctx: ()) -> usize;
        let mut router = Router::<F>::new();

        struct Users {}

        impl Resource for Users {
            type Handler = F;
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

        router.resource("/geocoder", Users::build());

        // dbg!(&router);

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
    }
}
