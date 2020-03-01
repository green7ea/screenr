use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Rect
{
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Mode
{
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
}

impl ToString for Mode
{
    fn to_string(&self) -> String
    {
        format!("{}x{}", self.width, self.height)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Display
{
    pub id: Option<u8>,
    pub name: String,
    pub rect: Rect,
    pub focus: Option<Vec<u32>>,
    pub active: bool,
    pub primary: bool,
    pub make: String,
    pub model: String,
    pub modes: Vec<Mode>,
    pub current_mode: Option<Mode>,
    pub focused: Option<bool>,
}

impl std::string::ToString for Display
{
    /**
     * Returns a string that represents the resolution - "1920x1080".
     */
    fn to_string(&self) -> std::string::String
    {
        format!("{} - {}", self.name, self.model)
    }
}

impl std::cmp::PartialEq for Display
{
    fn eq(&self, other: &Self) -> bool
    {
        self.name.eq(&other.name)
    }
}

impl std::cmp::PartialOrd for Display
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        Some(self.cmp(&other))
    }
}

impl std::cmp::Eq for Display {}

impl std::cmp::Ord for Display
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.rect.x.cmp(&other.rect.x)
    }
}
