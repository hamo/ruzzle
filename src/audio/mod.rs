mod web_audio_player;
mod dummy_audio_player;
pub use self::web_audio_player::WebAudioPlayer;
pub use self::dummy_audio_player::DummyAudioPlayer;

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub enum BGM {
    World0,
    LevelSelection,
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
pub enum SFX {
    RockFall,
    RockMove,
    Dead,
    Fanfare,
    Projecting,
}

pub trait AudioPlayer {
    fn play_bgm(&mut self, bgm: BGM);
    fn play_sfx(&mut self, sfx: SFX);
}