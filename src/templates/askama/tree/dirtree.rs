use askama::Template;
use lasso::Rodeo;

use crate::model::tree::TemplateNode;

#[derive(Debug)]
pub enum Rendered {
    //Text(String), //text
    SVG(String), //svg
}

pub trait RenderSVG<'a>
where
    Self: ToString + Sized,
{
    fn new(node: TemplateNode<'a>) -> Self;

    fn render(node: TemplateNode<'a>) -> Rendered {
        let instance = Self::new(node);
        let result = instance.to_string();
        Rendered::SVG(result)
    }
}

#[derive(Template)]
#[template(path = "askama/tree/dirtree.svg")]
pub struct AskamaDirTreeTemplateSVGPlain<'a> {
    pub node: TemplateNode<'a>,
}

impl<'a> RenderSVG<'a> for AskamaDirTreeTemplateSVGPlain<'a> {
    fn new(node: TemplateNode<'a>) -> Self {
        Self { node }
    }
}
