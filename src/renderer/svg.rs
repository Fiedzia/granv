use ego_tree::NodeRef;
use indicatif::ProgressBar;

use crate::model::tree::Tree;
use crate::templates::askama::tree::dirtree::Rendered;

pub fn render_svg_templates<'a>(
    tree: &'a Tree,
    render_fn: fn(NodeRef<'a, std::string::String>) -> Rendered,
    progress_bar: Option<&ProgressBar>,
) -> Vec<Rendered> {
    let root = tree.tree.root();
    let mut results: Vec<Rendered> = Vec::with_capacity(tree.tree.nodes().len());
    for node in tree.tree.nodes() {
        if node == root {
            continue;
        };
        let rendered = render_fn(node);
        results.push(rendered);
        if let Some(pb) = progress_bar {
            pb.tick();
            pb.inc(1);
        }
    }
    results
}
