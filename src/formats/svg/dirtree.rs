use crate::layouts::dirtree::svg::NodeLayout;
use crate::templates::askama::tree::dirtree::Rendered;
use std::fs::File;
use std::io::Write;
use std::iter;

const SVG_START: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
const SVG_END: &str = "</svg>";

pub fn export_dirtree_svg(
    rendered_node_templates: &[Rendered],
    node_layout: &[NodeLayout],
) -> Result<(), String> {
    let mut file = File::create("dirtree.svg").map_err(|e| e.to_string())?;
    file.write_all(SVG_START.as_bytes())
        .map_err(|e| e.to_string())?;
    for (tmpl, nl) in rendered_node_templates.into_iter().zip(node_layout) {
        match tmpl {
            Rendered::SVG(svg, width, height) => {
                file.write_all(
                    format!("\n<!-- {:?}  {} {} -->\n", nl.left_top, width, height).as_ref(),
                )
                .map_err(|e| e.to_string())?;
                file.write_all(svg.as_ref()).map_err(|e| e.to_string())?;
            }
        }
    }
    file.write_all(SVG_END.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}
