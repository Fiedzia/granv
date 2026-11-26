use ego_tree::{self, NodeId};

use crate::model::tree::Tree;
use crate::templates::askama::tree::dirtree::Rendered;

pub struct NodeLayout {
    pub node_id: NodeId,
    pub left_top: (usize, usize), //x, y
    pub width_height: (usize, usize),
}

pub enum LineLayout {
    Horizontal((NodeId, (usize, usize), (usize, usize))), //node to, (x,y), (width, height) (in mm)
    Vertical((NodeId, (usize, usize), (usize, usize))), //node from, (x, y), (width, height) (in mm)
}

pub fn layout_dirtree_nodes_and_lines_svg(
    tree: &Tree,
    rendered_templates: &[Rendered],
) -> Result<(Vec<NodeLayout>, Vec<LineLayout>), String> {
    let spacing = 1; //mm, space between 
    let horizontal_step = 20; //mm

    let mut x = spacing; //outer image border
    let mut y = spacing; //outer image border
    let mut add_spacing = false; // add spacing between nodes starting from second one

    let root = tree.tree.root();
    let mut vertical_lines: Vec<Option<LineLayout>> =
        vec![Some(LineLayout::Vertical((root.id(), (x, y), (0, 0))))];
    let mut level = 0;

    let mut result_node_layouts = vec![];

    for (idx, node) in tree.tree.nodes().enumerate() {
        if node == root {
            continue;
        };
        let prev_level = level;

        level = node.ancestors().count(); //includes root
        if add_spacing {
            y += spacing;
        } else {
            add_spacing = true;
        };
        x = spacing + (level * horizontal_step) + ((level - 1) / 2) * spacing + spacing;

        //println!("Node: {:?}, {}, {}, {}", node.value(), x, y, level);

        y += match rendered_templates[idx - 1] {
            Rendered::SVG(_, width, height) => {
                result_node_layouts.push(NodeLayout {
                    node_id: node.id(),
                    left_top: (x, y),
                    width_height: (width, height),
                });
                height
            } //_ => return Err(format!("SVG rendered did not expect non-svg template {:?} for node {}",rendered_templates[idx-1], node.value() ))
        };
    }

    Ok((result_node_layouts, vec![]))
}
