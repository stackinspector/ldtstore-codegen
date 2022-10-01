use std::borrow::Cow;
use serde::Serialize;
use crate::Map;

pub type ToolIndex<'a> = Map<'a, ToolIndexItem<'a>>;

#[derive(Clone, Debug, Serialize)]
pub struct ToolIndexItem<'a> {
    pub title: Cow<'a, str>,
    pub list: Vec<Cow<'a, str>>,
    pub cross_list: Vec<Cow<'a, str>>,
}

pub type ToolCategory<'a> = Map<'a, ToolCategoryItem<'a>>;

#[derive(Clone, Debug, Serialize)]
pub struct ToolCategoryItem<'a> {
    pub title: Cow<'a, str>,
    pub list: Vec<Cow<'a, str>>,
}

pub type ToolAll<'a> = Map<'a, Cow<'a, str>>;

pub type ToolCross<'a> = Map<'a, Map<'a, Cow<'a, str>>>;

#[derive(Clone, Debug, Serialize)]
pub struct ToolData<'a> {
    pub index: ToolIndex<'a>,
    pub category: ToolCategory<'a>,
    pub all: ToolAll<'a>,
    pub cross: ToolCross<'a>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "page_type")]
#[serde(rename_all = "snake_case")]
pub enum GlobalData<'a> {
    Home,
    Tool { tool: ToolData<'a> }
}
