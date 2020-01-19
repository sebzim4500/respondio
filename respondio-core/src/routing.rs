use std::collections::HashMap;

enum RouteComponent {
    Literal(String),
    Variable(String)
}

pub struct Route {
    components: Vec<RouteComponent>
}

impl Route {
    pub fn new(mut route: &str) -> Self {
        route = route.trim_start_matches('/');

        for c in route.chars() {
            if !c.is_ascii() {
                panic!("Routes can not contain unicode symbols");
            }
            if c.is_ascii_control() {
                panic!("Invalid character in route")
            }
        }

        let mut components: Vec<RouteComponent> = Vec::new();
        for component in route.split('/') {
            if component.starts_with("{") && component.ends_with("}") {
                components.push(RouteComponent::Variable(component[1 .. component.len() - 1].to_string()))
            } else {
                components.push(RouteComponent::Literal(component.to_string()))
            }
        }
        Route { components }
    }
}

pub struct RouteTree<T> {
    root: RouteTreeNode<T>,
}

impl<T> Default for RouteTree<T> {
    fn default() -> Self {
        RouteTree { root: Default::default() }
    }
}

struct RouteTreeNode<T> {
    handler: Option<T>,
    literals: HashMap<String, RouteTreeNode<T>>,
    variable: Option<Box<RouteTreeNode<T>>>,
}

impl<T> Default for RouteTreeNode<T> {
    fn default() -> Self {
        RouteTreeNode {
            handler: Default::default(),
            literals: Default::default(),
            variable: Default::default(),
        }
    }
}

impl<T> RouteTree<T> {
    pub fn add_route(&mut self, route: Route, handler: T) {
        let mut node = &mut self.root;
        for component in route.components {
            match component {
                RouteComponent::Literal(literal) => {
                    node = node.literals.entry(literal).or_default();
                },
                RouteComponent::Variable(_) => {
                    node = node.variable.get_or_insert_with(|| Default::default());
                },
            }
        }
        if node.handler.is_some() {
            panic!("Two routes for same path");
        }
        node.handler = Some(handler);
    }

    pub fn match_path<'p>(&self, path: &'p str) -> Option<(&T, Vec<String>)> {
        let mut node = &self.root;
        let mut var_matches = Vec::new();
        for component in path.trim_start_matches('/').split('/') {
            if let Some(next_node) = node.literals.get(component) {
                node = next_node
            } else if let Some(next_node) = node.variable.as_ref() {
                var_matches.push(component.to_string());
                node = &*next_node;
            } else {
                return None;
            }
        }
        node.handler.as_ref().map(|x| (x, var_matches))
    }
}

#[cfg(test)]
mod tests {
    use crate::routing::{RouteTree, Route};

    #[test]
    fn test_root_path() {
        let mut routes = RouteTree::default();
        routes.add_route(Route::new("/"), "root");
        assert_eq!(routes.match_path(""), Some((&"root", vec!())));
        assert_eq!(routes.match_path("/"), Some((&"root", vec!())));
    }

    #[test]
    fn test_single_var() {
        let mut routes = RouteTree::default();
        routes.add_route(Route::new("{}"), "variable");
        routes.add_route(Route::new("hello"), "literal");
        assert_eq!(routes.match_path("hello"), Some((&"literal", vec!())));
        assert_eq!(routes.match_path("world"), Some((&"variable", vec!("world".to_string()))));
    }

    #[test]
    fn test_beginning_var() {
        let mut routes = RouteTree::default();
        routes.add_route(Route::new("{}/asdf"), "variable");
        routes.add_route(Route::new("hello/asdf"), "literal");
        assert_eq!(routes.match_path("hello/asdf"), Some((&"literal", vec!())));
        assert_eq!(routes.match_path("world/asdf"), Some((&"variable", vec!("world".to_string()))));
        assert_eq!(routes.match_path("world"), None);
    }
}