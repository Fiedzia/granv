use std::io::BufReader;

use ego_tree::{self, NodeId};
use indicatif::ProgressBar;
use xml::reader::{EventReader, XmlEvent};

use crate::model::tree::Tree;
use crate::templates::askama::tree::dirtree::Rendered;

pub struct NodeLayout {
    pub node_id: NodeId,
    pub left_top: (u32, u32), //x, y
    pub width_height: (u32, u32),
}

pub enum LineLayout {
    Horizontal((NodeId, (u32, u32), (u32, u32))), //node to, (x,y), (width, height) (in mm)
    Vertical((NodeId, (u32, u32), (u32, u32))),   //node from, (x, y), (width, height) (in mm)
}

///Parse enough svg to extract width and height
pub fn get_svg_width_height(svg: &str) -> Result<(u32, u32), String> {
    let parser = EventReader::new(BufReader::new(svg.as_bytes()));
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let mut w: Option<u32> = None;
                let mut h: Option<u32> = None;
                for attr in &attributes {
                    if attr.name.local_name == "width" {
                        if attr.value.ends_with("mm") {
                            w = Some(
                                attr.value[0..attr.value.len() - 2]
                                    .parse::<u32>()
                                    .map_err(|e| e.to_string())?,
                            );
                        }
                    };
                    if attr.name.local_name == "height" {
                        if attr.value.ends_with("mm") {
                            h = Some(
                                attr.value[0..attr.value.len() - 2]
                                    .parse::<u32>()
                                    .map_err(|e| e.to_string())?,
                            );
                        }
                    };
                }
                if let (Some(width), Some(height)) = (w, h) {
                    return Ok((width, height));
                }
            }
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
    }
    Err(format!(
        "SVG parser faild to extract image width and height: {}",
        svg
    ))
}

pub fn layout_dirtree_nodes_and_lines_svg(
    tree: &Tree,
    rendered_templates: &[Rendered],
    progress_bar: Option<&ProgressBar>,
) -> Result<(Vec<NodeLayout>, Vec<LineLayout>, (u32, u32)), String> {
    //                                        width, height
    let spacing = 1u32; //mm, space between 
    let horizontal_step = 20u32; //mm

    let mut x: u32 = spacing; //outer image border
    let mut y: u32 = spacing; //outer image border
    let mut max_width = spacing;
    let mut add_spacing = false; // add spacing between nodes starting from second one

    let root = tree.tree.root();
    let mut vertical_lines: Vec<Option<LineLayout>> =
        vec![Some(LineLayout::Vertical((root.id(), (x, y), (0, 0))))];
    let mut level = 0u32;

    let mut result_node_layouts = vec![];

    for (idx, node) in tree.tree.nodes().enumerate() {
        if node == root {
            continue;
        };
        let prev_level = level;

        level = node.ancestors().count().try_into().unwrap(); //includes root
        if add_spacing {
            y += spacing;
        } else {
            add_spacing = true;
        };
        x = spacing + (level * horizontal_step) + ((level - 1) / 2) * spacing + spacing;

        //println!("Node: {:?}, {}, {}, {}", node.value(), x, y, level);

        let (width, height) = match rendered_templates[idx - 1] {
            Rendered::SVG(ref svg) => {
                let (width, height) = get_svg_width_height(&svg)?;
                result_node_layouts.push(NodeLayout {
                    node_id: node.id(),
                    left_top: (x, y),
                    //width_height: (width, height),
                    width_height: (width, height),
                });
                (width, height)
            } //_ => return Err(format!("SVG rendered did not expect non-svg template {:?} for node {}",rendered_templates[idx-1], node.value() ))
        };
        y += height;
        max_width = std::cmp::max(max_width, x + width + spacing);
        if let Some(pb) = progress_bar
            && idx % 1000 == 0
        {
            pb.inc(1000);
            pb.tick();
        }
    }

    Ok((result_node_layouts, vec![], (max_width, y + spacing)))
}
