use audio::{AudioPlayer, BGM, SFX};

pub struct DummyAudioPlayer {
}

impl DummyAudioPlayer {
    pub fn new() -> DummyAudioPlayer {
        DummyAudioPlayer{}
    }
}

impl AudioPlayer for DummyAudioPlayer {
    fn play_bgm(&mut self, bgm: BGM) {}

    fn play_sfx(&mut self, sfx: SFX) {}
}
