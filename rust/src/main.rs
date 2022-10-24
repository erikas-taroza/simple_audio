mod lib;

use crate::lib::Player;

fn main()
{
    let mut player = Player::new();
    player.open("/home/erikas/Music/test.mp3").expect("Error");
}