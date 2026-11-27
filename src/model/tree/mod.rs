use std::collections::HashMap;

use ego_tree;
use lasso::{Rodeo, Spur};
use serde_json;

pub mod load;

pub struct Tree {
    pub tree: ego_tree::Tree<Spur>,
    pub rodeo: Rodeo, // string interner
    pub node_properties: HashMap<Spur, Option<serde_json::Map<String, serde_json::Value>>>,
}

impl Tree {
    pub fn new() -> Self {
        let mut rodeo = Rodeo::default();
        let root_key = rodeo.get_or_intern("root");
        Tree {
            tree: ego_tree::Tree::new(root_key),
            node_properties: HashMap::new(),
            rodeo,
        }
    }

    pub fn get_by_path_mut(
        &mut self,
        path_seq: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Option<ego_tree::NodeMut<'_, Spur>> {
        let mut parent = self.tree.root();
        for item in path_seq {
            parent = match &parent
                .children()
                .filter(|n| self.rodeo.resolve(n.value()) == item.as_ref())
                .next()
            {
                Some(node) => self.tree.get(node.id()).unwrap(),
                None => return None,
            }
        }
        Some(self.tree.get_mut(parent.id()).unwrap())
    }
}

///Tree node trait that will be passed to templates
///provides any data templates may use

pub struct TemplateNode<'a> {
    pub node: ego_tree::NodeRef<'a, Spur>,
    pub rodeo: &'a Rodeo,
}

impl<'a> TemplateNode<'a> {
    pub fn new(node: ego_tree::NodeRef<'a, Spur>, rodeo: &'a Rodeo) -> Self {
        Self { node, rodeo }
    }

    pub fn name(&self) -> &str {
        self.rodeo.resolve(self.node.value())
    }

    pub fn id(&self) -> String {
        //annoyingly, NodeId index is private. But it implements Debug, so..
        let formatted = format!("{:?}", self.node.id());
        let value = &formatted[7..formatted.len() - 1];
        format!("node_{}", value)
    }
}
