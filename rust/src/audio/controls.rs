//use std::sync::{atomic::AtomicBool, RwLock};

#[derive(Default)]
pub struct Controls
{
    pub is_playing:bool,
    pub volume:f32
}

// impl Controls
// {
//     pub fn default() -> Self
//     {
//         Controls
//         {
//             is_playing: AtomicBool::new(true),
//             volume: RwLock::new(0.5)
//         }
//     }
// }