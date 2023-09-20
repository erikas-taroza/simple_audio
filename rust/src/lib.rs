// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

mod api;
mod audio;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod media_controllers;
mod utils;

// https://github.com/RustAudio/cpal/issues/720#issuecomment-1311813294
#[cfg(target_os = "android")]
#[no_mangle]
extern "C" fn JNI_OnLoad(vm: jni::JavaVM, res: *mut std::os::raw::c_void) -> jni::sys::jint
{
    use std::ffi::c_void;

    let vm = vm.get_java_vm_pointer() as *mut c_void;
    unsafe {
        ndk_context::initialize_android_context(vm, res);
    }
    jni::JNIVersion::V6.into()
}

#[cfg(test)]
mod tests
{
    use std::{thread, time::Duration};

    use crate::api::*;
    use crate::media_controllers::types::{MediaControlAction, Metadata};

    #[test]
    fn open_and_play() -> anyhow::Result<()>
    {
        let player = Player::default();
        player.set_volume(0.1);
        player.open("/home/erikas/Downloads/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(100));
        Ok(())
    }

    #[test]
    fn open_network_and_play() -> anyhow::Result<()>
    {
        let player = Player::default();
        // You can change the file extension here for different formats ------v
        // https://docs.espressif.com/projects/esp-adf/en/latest/design-guide/audio-samples.html
        player.open(
            "https://dl.espressif.com/dl/audio/ff-16b-2c-44100hz.mp3".to_string(),
            true,
        )?;
        player.set_volume(0.1);
        thread::sleep(Duration::from_secs(10));
        player.seek(90);
        thread::sleep(Duration::from_secs(10));
        player.seek(60);
        thread::sleep(Duration::from_secs(187));
        Ok(())
    }

    #[test]
    fn open_hls_and_play() -> anyhow::Result<()>
    {
        let player = Player::default();
        player.open("https://cf-hls-media.sndcdn.com/playlist/x7uSGJp4rku7.128.mp3/playlist.m3u8?Policy=eyJTdGF0ZW1lbnQiOlt7IlJlc291cmNlIjoiKjovL2NmLWhscy1tZWRpYS5zbmRjZG4uY29tL3BsYXlsaXN0L3g3dVNHSnA0cmt1Ny4xMjgubXAzL3BsYXlsaXN0Lm0zdTgqIiwiQ29uZGl0aW9uIjp7IkRhdGVMZXNzVGhhbiI6eyJBV1M6RXBvY2hUaW1lIjoxNjc1ODA1NTM2fX19XX0_&Signature=Cd6o8KT6AEoLaIHok~438sourFeoHywCDdG09MS38qxmWLsKyJU-eFHOdh8jccvfPaWfjYkEEqfnpp6EMINXP3f99GAwWFPGMrp43lqz2JAL5MBUAc1plLLm1KV~t5Vy5ON6M1X~Fj6nFV7vdD7mGR84lfeafFmXBP4U4oZATI9GoPrUkEgVtCViDg6kBMVKk77e144LFwzZtkiSHj-S7umU5Qf9r2lDCqYaHVVoWSMtJBWMXoKQZCjdR5e6pqINcRQA-348wX8C9bonQGeoCZ3xRQWPq0ZtznmDKdZ-p91YJL8o4LNSPOMreu-ELsXhoftd7iKpZoG7~YwX2Oxg5A__&Key-Pair-Id=APKAI6TU7MMXM5DG6EPQ".to_string(), true)?;
        player.set_volume(0.1);
        thread::sleep(Duration::from_secs(10));
        player.seek(90);
        thread::sleep(Duration::from_secs(10));
        player.seek(60);
        thread::sleep(Duration::from_secs(187));
        Ok(())
    }

    // The following tests are to check the responsiveness.
    #[test]
    fn play_pause() -> anyhow::Result<()>
    {
        let player = Player::default();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(1));
        println!("Pausing now");
        player.pause();
        thread::sleep(Duration::from_secs(5));
        println!("Playing now");
        player.play();
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn volume() -> anyhow::Result<()>
    {
        let player = Player::default();
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(1));
        println!("Changing volume now");
        player.set_volume(0.2);
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn seeking() -> anyhow::Result<()>
    {
        let player = Player::default();
        player.set_volume(0.5);
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(1));
        println!("Seeking now");
        player.seek(50);
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn stop() -> anyhow::Result<()>
    {
        let player = Player::default();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        player.seek(10);
        thread::sleep(Duration::from_secs(5));
        println!("Stopping now");
        player.stop();
        println!("Playing now");
        player.open("/home/erikas/Music/2.mp3".to_string(), true)?;
        player.stop();
        thread::sleep(Duration::from_millis(50));
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        player.play();
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn mpris() -> anyhow::Result<()>
    {
        let player = Player::new(
            vec![MediaControlAction::PlayPause],
            "SimpleAudio".to_string(),
            None,
        );
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        player.set_metadata(Metadata {
            title: Some("My Title".to_string()),
            artist: Some("My Artist".to_string()),
            album: Some("My Album".to_string()),
            ..Default::default()
        });

        thread::sleep(Duration::from_secs(2));

        player.set_metadata(Metadata {
            title: Some("My Title2".to_string()),
            artist: Some("My Artist2".to_string()),
            album: Some("My Album2".to_string()),
            ..Default::default()
        });

        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn preload() -> anyhow::Result<()>
    {
        let path = "https://dl.espressif.com/dl/audio/ff-16b-2c-44100hz.mp3".to_string();

        let player = Player::default();
        player.open(path.clone(), true)?;
        player.set_volume(0.5);
        println!("Preloading");
        player.preload(path.clone())?;
        thread::sleep(Duration::from_secs(5));
        println!("Playing preloaded file.");
        player.stop();
        player.play_preload();
        thread::sleep(Duration::from_secs(10));
        player.stop();
        player.open(path, true)?;
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    /// SEE: https://github.com/erikas-taroza/simple_audio/issues/19
    #[test]
    fn silent_adts_file() -> anyhow::Result<()>
    {
        let player = Player::default();
        player.set_volume(0.1);
        player.open("/home/erikas/Downloads/silent.aac".to_string(), true)?;
        thread::sleep(Duration::from_secs(3));
        Ok(())
    }
}
