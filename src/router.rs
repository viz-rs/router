use std::iter::FromIterator;

use radix_tree::Node;

#[derive(Clone, Debug)]
pub struct Route {}

#[derive(Clone, Debug)]
pub enum NodeKind {
    Root = 0,
    Static,
    Parameter,
    CatchAll,
}

impl Default for NodeKind {
    fn default() -> Self {
        NodeKind::Static
    }
}

#[derive(Clone, Debug, Default)]
pub struct NodeMetadata {
    kind: NodeKind,
    is_key: bool,
    params: Option<Vec<&'static str>>,
}

impl NodeMetadata {
    pub fn new() -> Self {
        NodeMetadata {
            ..Default::default()
        }
    }
}

impl Route {
    pub fn new() -> Self {
        Route {}
    }
}

#[derive(Debug)]
pub struct Router {
    tree: Node<char, NodeMetadata>,
}

impl Router {
    pub fn new(path: &'static str, data: NodeMetadata) -> Self {
        Router {
            tree: Node::new(path, data),
        }
    }

    pub fn insert(&mut self, path: &'static str, data: NodeMetadata) -> &mut Self {
        let mut node = &mut self.tree;
        let mut params: Option<Vec<Vec<char>>> = None;
        let mut buf: Vec<char> = path.trim_start_matches('/').chars().collect();

        // Root "/"
        if 0 == buf.len() {
            match &mut node.data {
                Some(ref mut d) => {
                    d.is_key = true;
                }
                None => {
                    node.data = Some(NodeMetadata::default());
                }
            }
            return self;
        }

        while 0 < buf.len() {
            let mut i: usize = 0;
            let mut next: Vec<char>;
            let mut meta = NodeMetadata::default();

            match buf[i] {
                '*' => {
                    next = buf.split_off(buf.len());
                    match params.as_mut() {
                        Some(p) => {
                            p.push(buf.split_off(1));
                        }
                        None => {
                            params.replace(vec![buf.split_off(1)]);
                        }
                    }
                    meta.kind = NodeKind::CatchAll;
                }
                ':' => {
                    next = buf.split_off(loop {
                        if i == buf.len() {
                            break i;
                        }
                        if '*' == buf[i] || '/' == buf[i] {
                            break i;
                        }
                        i += 1;
                        // match buf[i] {
                        //     '*' | '/' => {
                        //         break i;
                        //     }
                        //     // pattern open
                        //     // '(' => {}
                        //     // pattern close
                        //     // ')' => {}
                        //     _ => {
                        //         i += 1;
                        //     }
                        // }
                    });
                    match params.as_mut() {
                        Some(p) => {
                            p.push(buf.split_off(1));
                        }
                        None => {
                            params.replace(vec![buf.split_off(1)]);
                        }
                    }
                    meta.kind = NodeKind::Parameter;
                }
                _ => {
                    next = buf.split_off(loop {
                        if i == buf.len() {
                            break i;
                        }
                        if '*' == buf[i] || ':' == buf[i] {
                            break i;
                        }
                        i += 1;
                        // match buf[i] {
                        //     '*' | ':' => {
                        //         break i;
                        //     }
                        //     _ => {
                        //         i += 1;
                        //     }
                        // }
                    });
                    meta.kind = NodeKind::Static;
                }
            }

            let ended = 0 == next.len();

            // end
            if 0 == next.len() {
                if let Some(ref p) = params {
                    meta.params = Some(
                        p.iter()
                            .map(|x| {
                                &*(Box::leak(String::from_iter(x.into_iter()).into_boxed_str()))
                            })
                            .collect(),
                    );
                }
                meta.is_key = true;
            }

            node = node.add_node_with(&mut buf, Some(meta), 0, ended, |&l, &c, indices| {
                let mut j = l;
                if 0 == j {
                    return j;
                }

                if '*' == c {
                    return j;
                }

                if '*' == indices[j - 1] {
                    j -= 1;
                }

                if ':' == c {
                    return j;
                }

                if ':' == indices[j - 1] {
                    j -= 1;
                }

                j
            });

            buf = next;
        }

        self
    }

    pub fn find(
        &mut self,
        path: &'static str,
    ) -> Option<(&Node<char, NodeMetadata>, Option<Vec<Vec<char>>>)> {
        let node = &self.tree;
        let buf: &Vec<char> = &path.chars().collect();

        recognize(&buf, &node)
    }
}

pub fn recognize<'a>(
    path: &Vec<char>,
    node: &'a Node<char, NodeMetadata>,
) -> Option<(&'a Node<char, NodeMetadata>, Option<Vec<Vec<char>>>)> {
    if path.len() == 0 {
        return None;
    }

    let mut buf: Vec<char> = path.clone();
    let mut next: Vec<char>;
    let mut values: Option<Vec<Vec<char>>> = None;

    match node.path[0] {
        '*' => {
            match values.as_mut() {
                Some(v) => {
                    v.push(buf);
                }
                None => {
                    values.replace(vec![buf]);
                }
            }
            return Some((&node, values));
        }
        ':' => {
            let mut i = 0;
            next = buf.split_off(loop {
                if i == buf.len() {
                    break i;
                }
                if '/' == buf[i] {
                    break i;
                }
                i += 1;
            });

            match values.as_mut() {
                Some(v) => {
                    v.push(buf);
                }
                None => {
                    values.replace(vec![buf]);
                }
            }

            if 0 == next.len() {
                return Some((&node, values));
            }

            if 0 == node.indices.len() {
                return None;
            }

            if let Some((n, v)) = recognize(&next, &node.nodes[0]) {
                if let Some(mut d) = v {
                    values.as_mut().unwrap().append(&mut d);
                }
                return Some((&n, values));
            }

            return None;
        }
        _ => {
            let mut m = buf.len();
            let mut n = m;
            let mut o = node.path.len();

            if m >= o {
                m = 0;
                while m < o && buf[m] == node.path[m] {
                    m += 1;
                }
            }

            if m < o {
                return None;
            }

            if m == o && m == n {
                return Some((&node, values));
            }

            let mut l = node.indices.len();
            if 0 == l {
                return None;
            }

            next = buf.split_off(m);

            o = 0;
            let mut has_star = false;
            if '*' == node.indices[l - 1] {
                l -= 1;
                o = l;
                has_star = true;
            }

            n = 0;
            let mut has_colon = false;
            if l > 0 && ':' == node.indices[l - 1] {
                l -= 1;
                n = l;
                has_colon = true;
            }

            m = 0;
            let c = next[m];
            let mut has_node = false;
            while m < l {
                if c == node.indices[m] {
                    has_node = true;
                    break;
                }
                m += 1;
            }

            // Static Node
            if has_node {
                if let Some((n, v)) = recognize(&next, &node.nodes[m]) {
                    if let Some(mut d) = v {
                        match values.as_mut() {
                            Some(v) => {
                                v.append(&mut d);
                            }
                            None => {
                                values.replace(d);
                            }
                        }
                    }
                    return Some((&n, values));
                }
            }

            // Parameter
            if has_colon {
                if let Some((n, v)) = recognize(&next, &node.nodes[n]) {
                    if let Some(mut d) = v {
                        match values.as_mut() {
                            Some(v) => {
                                v.append(&mut d);
                            }
                            None => {
                                values.replace(d);
                            }
                        }
                    }
                    return Some((&n, values));
                }
            }

            // CatchAll
            if has_star {
                if let Some((n, v)) = recognize(&next, &node.nodes[o]) {
                    if let Some(mut d) = v {
                        match values.as_mut() {
                            Some(v) => {
                                v.append(&mut d);
                            }
                            None => {
                                values.replace(d);
                            }
                        }
                    }
                    return Some((&n, values));
                }
            }

            // dbg!(buf = next);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_router() {
        let mut router = Router::new(
            "/",
            NodeMetadata {
                kind: NodeKind::Root,
                ..Default::default()
            },
        );
        // let mut router = Router::<u32, Route>::new("/", Route::new());
        // let mut router = Router::<char, Route>::new("/", Route::new());
        assert_eq!(router.tree.path.len(), 1);

        router.insert("/", NodeMetadata::new());
        router.insert("/users", NodeMetadata::new());
        router.insert("/users/:id", NodeMetadata::new());
        router.insert("/users/:id/:org", NodeMetadata::new());
        router.insert("/users/:user_id/repos", NodeMetadata::new());
        router.insert("/users/:user_id/repos/:id", NodeMetadata::new());
        router.insert("/users/:user_id/repos/:id/*any", NodeMetadata::new());
        router.insert("/:username", NodeMetadata::new());
        router.insert("/*any", NodeMetadata::new());
        router.insert("/about", NodeMetadata::new());
        router.insert("/about/", NodeMetadata::new());
        router.insert("/about/us", NodeMetadata::new());
        router.insert("/users/repos/*any", NodeMetadata::new());

        println!("{:#?}", router);

        let route = router.find("/");
        // println!("/ {:#?}", route);
        assert_eq!(route.is_some(), true);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['/']);
        assert_eq!(res.1, None);

        let route = router.find("/users");
        // println!("/users: {:#?}", route);
        assert_eq!(route.is_some(), true);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['u', 's', 'e', 'r', 's']);
        assert_eq!(res.1, None);

        let route = router.find("/about");
        // println!("/about {:#?}", route);
        assert_eq!(route.is_some(), true);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['a', 'b', 'o', 'u', 't']);
        assert_eq!(res.1, None);

        let route = router.find("/about/");
        // println!("/about/ {:#?}", route);
        assert_eq!(route.is_some(), true);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['/']);
        assert_eq!(res.1, None);

        let route = router.find("/about/us");
        // println!("/about/us {:#?}", route);
        assert_eq!(route.is_some(), true);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['u', 's']);
        assert_eq!(res.1, None);

        let route = router.find("/username");
        // println!("/username {:#?}", route);
        assert_eq!(route.is_some(), true);
        let res = route.unwrap();
        assert_eq!(res.0.path, [':']);
        assert_eq!(res.1.unwrap(), [['u', 's', 'e', 'r', 'n', 'a', 'm', 'e']]);

        let route = router.find("/user/s");
        // println!("/user/s {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['*']);
        assert_eq!(res.1.unwrap(), [['u', 's', 'e', 'r', '/', 's']]);

        let route = router.find("/users/fundon/repo");
        // println!("/users/fundon/repo {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, [':']);
        assert_eq!(
            res.1.unwrap(),
            vec![vec!['f', 'u', 'n', 'd', 'o', 'n'], vec!['r', 'e', 'p', 'o']]
        );

        let route = router.find("/users/fundon/repos");
        // println!("/users/fundon/repos {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, "repos".chars().collect::<Vec<char>>());
        assert_eq!(res.1.unwrap(), [['f', 'u', 'n', 'd', 'o', 'n']]);

        let route = router.find("/users/fundon/repos/trek-rs");
        // println!("/users/fundon/repos/233 {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, [':']);
        assert_eq!(
            res.1.unwrap(),
            vec![
                vec!['f', 'u', 'n', 'd', 'o', 'n'],
                vec!['t', 'r', 'e', 'k', '-', 'r', 's'],
            ]
        );

        let route = router.find("/users/fundon/repos/trek-rs/");
        // println!("/users/fundon/repos/233/ {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['/']);
        assert_eq!(
            res.1.unwrap(),
            vec![
                vec!['f', 'u', 'n', 'd', 'o', 'n'],
                vec!['t', 'r', 'e', 'k', '-', 'r', 's'],
            ]
        );

        let route = router.find("/users/fundon/repos/trek-rs/router");
        // println!("/users/fundon/repos/trek-rs/router {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['*']);
        assert_eq!(
            res.1.unwrap(),
            vec![
                vec!['f', 'u', 'n', 'd', 'o', 'n'],
                vec!['t', 'r', 'e', 'k', '-', 'r', 's'],
                vec!['r', 'o', 'u', 't', 'e', 'r']
            ]
        );

        let route = router.find("/users/fundon/repos/trek-rs/router/issues");
        // println!("/users/fundon/repos/trek-rs/router/issues {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['*']);
        assert_eq!(
            res.1.unwrap(),
            vec![
                vec!['f', 'u', 'n', 'd', 'o', 'n'],
                vec!['t', 'r', 'e', 'k', '-', 'r', 's'],
                vec!['r', 'o', 'u', 't', 'e', 'r', '/', 'i', 's', 's', 'u', 'e', 's']
            ]
        );

        let route = router.find("/users/repos/");
        // println!("/users/repos/ {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, "repos/".chars().collect::<Vec<char>>());
        assert_eq!(res.1.is_some(), false);

        let route = router.find("/about/as");
        // println!("/about/as {:#?}", route);
        let res = route.unwrap();
        assert_eq!(res.0.path, ['*']);
        assert_eq!(res.1.unwrap(), [['a', 'b', 'o', 'u', 't', '/', 'a', 's']]);
    }
}
