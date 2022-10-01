use std::borrow::Cow;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::Map;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PageType {
    Home,
    Tool,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputType {
    Major,
    Sides,
    Tools,
    Category,
}

pub type TileColumns<'a> = Vec<Vec<Tile<'a>>>;

#[derive(Clone, Debug, Deserialize)]
pub struct TileGrids<'a> {
    pub left: Vec<Tile<'a>>,
    pub middle: Vec<TileGridMiddle<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileGridMiddle<'a> {
    pub title: Cow<'a, str>,
    pub content: Vec<Tile<'a>>,
}

// TODO Vec<CategoryTab>
#[derive(Clone, Debug, Deserialize)]
pub struct Category<'a> {
    pub tool: CategoryTab<'a>,
    pub link: CategoryTab<'a>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CategoryTab<'a> {
    pub title: Cow<'a, str>,
    pub content: Vec<CategoryGroup<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CategoryGroup<'a> {
    pub title: Cow<'a, str>,
    pub content: Vec<Tile<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tile<'a> {
    pub tile: Option<Cow<'a, str>>, // prev no option
    pub font: Option<TileFont>,
    pub action: TileAction,
    pub icon_type: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    pub title: Option<Cow<'a, str>>,
    pub icon: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TileFont {
    H1,
    H2,
    H3,
    H4,
    H5,
}

impl TileFont {
    pub fn into_tag(&self) -> lighthtml::ElementTag {
        use TileFont::*;
        use lighthtml::ElementTag::*;
        match self {
            H1 => h1,
            H2 => h2,
            H3 => h3,
            H4 => h4,
            H5 => h5,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TileAction {
    Side,
    Tool,
    Category,
    Copy,
    Href,
    R,
    R2,
    Home,
    None,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileTemplate<'a> {
    pub template: TileTemplateInner<'a>,
    pub tiles: TileTemplateTiles<'a>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum TileTemplateTiles<'a> {
    WithoutTitle(Vec<Cow<'a, str>>),
    WithTitle(Map<'a, Cow<'a, str>>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileTemplateInner<'a> {
    pub tile: Cow<'a, str>,
    pub font: Option<TileFont>,
    pub action: TileAction,
    pub icon_type: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Side<'a> {
    pub name: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub text: Option<Cow<'a, str>>,
    pub text_small: Option<bool>,
    pub tiles: Option<Vec<Tile<'a>>>,
    pub templated: Option<TileTemplate<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToolGroup<'a> {
    pub name: Option<Cow<'a, str>>,
    pub title: Option<Cow<'a, str>>,
    pub cross_notice: Option<Cow<'a, str>>,
    pub list: Vec<Tool<'a>>,
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

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolLinkType {
    R2,
    Mirror,
}

impl ToolLinkType {
    pub fn as_str(&self) -> &'static str {
        use ToolLinkType::*;
        match self {
            R2 => "r2",
            Mirror => "mirror",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolLinkIcon {
    Link,
    Download,
}

impl ToolLinkIcon {
    pub fn as_str(&self) -> &'static str {
        use ToolLinkIcon::*;
        match self {
            Link => "link",
            Download => "download",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]

pub enum MirrorType {
    Active,
    Locked,
    Synced,
}

impl MirrorType {
    pub fn as_str(&self) -> &'static str {
        use MirrorType::*;
        match self {
            Active => "active",
            Locked => "locked",
            Synced => "synced",
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tool<'a> {
    pub name: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub icon: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>, // prev no option
    pub notice: Option<Cow<'a, str>>,
    pub category: Option<Vec<Cow<'a, str>>>,
    pub cross: Option<Vec<Cow<'a, str>>>,
    pub keywords: Option<Cow<'a, str>>,
    pub cross_notice: Option<Map<'a, Cow<'a, str>>>,
    #[serde(flatten)]
    pub links: ToolLinks<'a>,
}

// #[derive(Clone, Debug, Deserialize)]
// pub struct ToolInner<'a> {
//     #[serde(flatten)]
//     pub inner: ToolInner<'a>,
// }

// #[derive(Clone, Debug, Deserialize)]
// pub struct ToolCross<'a> {
//     #[serde(flatten)]
//     pub cross: ToolCross<'a>,
// }

#[derive(Clone, Debug, Deserialize)]
pub struct ToolLinks<'a> {
    pub website: Option<ToolLinkTitle<'a>>,
    pub websites: Option<Map<'a, ToolLinkTitle<'a>>>,
    pub downloads: Option<Map<'a, Cow<'a, str>>>,
    pub mirror: Option<MirrorType>,
    pub mirrors: Option<Map<'a, Cow<'a, str>>>,
    pub columns: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToolLink<'a> {
    pub title: ToolLinkTitle<'a>,
    pub link_type: ToolLinkType,
    pub link: Cow<'a, str>,
    pub icon: ToolLinkIcon,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ToolLinkTitle<'a> {
    Type(ToolLinkTitleType),
    Text(Cow<'a, str>),
}

pub const fn tool_website_type(t: ToolLinkTitleType) -> &'static str {
    use ToolLinkTitleType::*;
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
//     pub tools: Map<'a, Tool>,
//     pub tool_data: ToolData,
// }

pub const fn tool_link_prefix(t: ToolLinkType) -> &'static str {
    use ToolLinkType::*;
    match t {
        R2 => "//r.ldtstore.com.cn/r2/",
        Mirror => "{{MIRROR}}",
    }
}

pub const fn tool_icon_emoji(t: ToolLinkIcon) -> &'static str {
    use ToolLinkIcon::*;
    match t {
        Link => "🔗",
        Download => "💾",
    }
}
