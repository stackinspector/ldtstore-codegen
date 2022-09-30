use serde::Serialize;
use crate::Map;

pub type ToolIndex = Map<ToolIndexItem>;

#[derive(Clone, Debug, Serialize)]
pub struct ToolIndexItem {
    pub title: String,
    pub list: Vec<String>,
    pub cross_list: Vec<String>,
}

pub type ToolCategory = Map<ToolCategoryItem>;

#[derive(Clone, Debug, Serialize)]
pub struct ToolCategoryItem {
    pub title: String,
    pub list: Vec<String>,
}

pub type ToolAll = Map<String>;

pub type ToolCross = Map<Map<String>>;

#[derive(Clone, Debug, Serialize)]
pub struct ToolData {
    pub index: ToolIndex,
    pub category: ToolCategory,
    pub all: ToolAll,
    pub cross: ToolCross,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "page_type")]
#[serde(rename_all = "snake_case")]
pub enum GlobalData {
    Home,
    Tool { tool: ToolData }
}
