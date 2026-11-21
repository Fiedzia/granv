use std::collections::HashMap;

use ego_tree;
use serde_json;

pub mod load;

pub struct Tree {
    pub tree: ego_tree::Tree<String>,
    pub node_properties: HashMap<String, Option<serde_json::Map<String, serde_json::Value>>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            tree: ego_tree::Tree::new("root".to_string()),
            node_properties: HashMap::new(),
        }
    }

    pub fn get_by_path_mut(
        &mut self,
        path_seq: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Option<ego_tree::NodeMut<'_, String>> {
        let mut parent = self.tree.root();
        for item in path_seq {
            parent = match &parent
                .children()
                .filter(|n| n.value() == item.as_ref())
                .next()
            {
                Some(node) => self.tree.get(node.id()).unwrap(),
                None => return None,
            }
        }
        Some(self.tree.get_mut(parent.id()).unwrap())
    }
}
