use ego_tree::NodeRef;
use indicatif::ProgressBar;
use lasso::{Rodeo, Spur};

use crate::model::tree::{TemplateNode, Tree};
use crate::templates::askama::tree::dirtree::Rendered;

pub fn render_svg_templates<'a>(
    tree: &'a Tree,
    render_fn: fn(TemplateNode<'a>) -> Rendered,
    progress_bar: Option<&ProgressBar>,
) -> Vec<Rendered> {
    let root = tree.tree.root();
    let mut results: Vec<Rendered> = Vec::with_capacity(tree.tree.nodes().len());
    for (idx, node) in tree.tree.nodes().enumerate() {
        if node == root {
            continue;
        };

        let rendered = {
            let tn = TemplateNode::new(node, &tree.rodeo);
            render_fn(tn)
        };

        results.push(rendered);
        let cnt = 1000usize;
        if let Some(pb) = progress_bar
            && idx % cnt == 0
        {
            pb.tick();
            pb.inc(cnt as u64);
        }
    }
    results
}
