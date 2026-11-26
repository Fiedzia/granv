use askama::Template;
use ego_tree::NodeRef;

#[derive(Debug)]
pub enum Rendered {
    //Text(String, usize, usize), //text, width, height (in monospaced )
    SVG(String, usize, usize), //svg, width, height
}

pub trait RenderSVG<'a>
where
    Self: ToString + Sized,
{
    fn new(node: NodeRef<'a, String>) -> Self;

    fn render(node: NodeRef<'a, String>) -> Rendered {
        let instance = Self::new(node);
        let result = instance.to_string();
        Rendered::SVG(result, 50, 20)
    }
}

#[derive(Template)]
#[template(path = "askama/tree/dirtree.svg")]
pub struct AskamaDirTreeTemplateSVGPlain<'a> {
    node: NodeRef<'a, String>,
}

impl<'a> RenderSVG<'a> for AskamaDirTreeTemplateSVGPlain<'a> {
    fn new(node: NodeRef<'a, String>) -> Self {
        Self { node }
    }
}
