#[derive(Default)]
pub struct Metadata
{
    pub title:Option<String>,
    pub artist:Option<String>,
    pub album:Option<String>,
    pub duration:Option<u64>,
    pub art_url:Option<String>
}