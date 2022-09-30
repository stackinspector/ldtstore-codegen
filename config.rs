use serde::Deserialize;
use serde_repr::Deserialize_repr;
use serde_enum_str::Deserialize_enum_str;

use crate::Map;

#[derive(Clone, Copy, Debug, Deserialize_enum_str)]
pub enum PageType {
    home,
    tool,
}

#[derive(Clone, Copy, Debug, Deserialize_enum_str)]
pub enum InputType {
    major,
    sides,
    tools,
    category,
}

pub type TileColumns = Vec<Vec<Tile>>;

#[derive(Clone, Debug, Deserialize)]
pub struct TileGrids {
    pub left: Vec<Tile>,
    pub middle: Vec<TileGridMiddle>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileGridMiddle {
    pub title: String,
    pub content: Vec<Tile>,
}

// TODO Vec<CategoryTab>
#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub tool: CategoryTab,
    pub link: CategoryTab,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CategoryTab {
    pub title: String,
    pub content: Vec<CategoryGroup>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CategoryGroup {
    pub title: String,
    pub content: Vec<Tile>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tile {
    pub tile: Option<String>, // prev no option
    pub font: Option<String>,
    pub action: String,
    pub icon_type: Option<String>,
    pub name: String,
    pub title: Option<String>,
    pub icon: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileTemplate {
    pub template: TileTemplateInner,
    pub tiles: TileTemplateTiles,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum TileTemplateTiles {
    WithoutTitle(Vec<String>),
    WithTitle(Map<String>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileTemplateInner {
    pub tile: String,
    pub font: Option<String>,
    pub action: String,
    pub icon_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Side {
    pub name: String,
    pub title: String,
    pub text: Option<String>,
    pub text_small: Option<bool>,
    pub tiles: Option<Vec<Tile>>,
    pub templated: Option<TileTemplate>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToolGroup {
    pub name: Option<String>,
    pub title: Option<String>,
    pub cross_notice: Option<String>,
    pub list: Vec<Tool>,
}

#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ToolLinkTitleType {
    Official = 1,
    Link = 2,
    PageLink = 3,
    Unofficial = 4,
    OfficialLimited = 5,
}

#[derive(Clone, Copy, Debug, Deserialize_enum_str)]
pub enum ToolLinkType {
    r2,
    mirror,
}

#[derive(Clone, Copy, Debug, Deserialize_enum_str)]
pub enum ToolLinkIcon {
    link,
    download,
}

#[derive(Clone, Copy, Debug, Deserialize_enum_str)]

pub enum MirrorType {
    active,
    locked,
    synced,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tool {
    pub name: String,
    pub title: String,
    pub category: Option<Vec<String>>,
    pub cross: Option<Vec<String>>,
    pub keywords: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>, // prev no option
    pub website: Option<ToolLinkTitle>,
    pub websites: Option<Map<ToolLinkTitle>>,
    pub downloads: Option<Map<String>>,
    pub mirror: Option<MirrorType>,
    pub mirrors: Option<Map<String>>,
    pub columns: Option<bool>,
    pub notice: Option<String>,
    pub cross_notice: Option<Map<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToolLink {
    pub title: ToolLinkTitle,
    pub link_type: ToolLinkType,
    pub link: String,
    pub icon: ToolLinkIcon,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ToolLinkTitle {
    Type(ToolLinkTitleType),
    Text(String),
}

pub use PageType::*;
pub use InputType::*;
pub use ToolLinkTitleType::*;
pub use ToolLinkType::*;
pub use ToolLinkIcon::*;
pub use MirrorType::*;

pub const fn tool_website_type(t: ToolLinkTitleType) -> &'static str {
    match t {
        Official => "官方网站",
        Link => "首发链接",
        PageLink => "网页链接",
        Unofficial => "<b>非官方</b>页面",
        OfficialLimited => "官方网站（国内无法访问）",
    }
}

// #[derive(Clone, Debug)]
// pub struct ProcessedToolGroups {
//     pub tools: Map<Tool>,
//     pub tool_data: ToolData,
// }

pub const fn tool_link_prefix(t: ToolLinkType) -> &'static str {
    match t {
        r2 => "//r.ldtstore.com.cn/r2/",
        mirror => "{{MIRROR}}",
    }
}

pub const fn tool_icon_emoji(t: ToolLinkIcon) -> &'static str {
    match t {
        link => "🔗",
        download => "💾",
    }
}
