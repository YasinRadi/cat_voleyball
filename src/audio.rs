use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{
        prelude::{World, WorldExt},
    },
    audio::{
        Source,
        AudioSink,
        OggFormat,
        SourceHandle,
        output::Output,
    }
};

use std::iter;
use std::vec;

/// Background music tracks
pub struct Music {
    pub music: iter::Cycle<vec::IntoIter<SourceHandle>>,
}

/// Sound effects
pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub collision_sfx: SourceHandle,
}

/// Loads an ogg audio track
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(
        file, 
        OggFormat, 
        (), 
        &world.read_resource()
    )
}

fn play_sound(storage: &AssetStorage<Source>, output: Option<&Output>, sound: SourceHandle) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sound) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_collision(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    play_sound(storage, output, sounds.collision_sfx)
}

pub fn play_score(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    play_sound(storage, output, sounds.score_sfx)
}

/// Initializes the audio system
pub fn initialize_audio(world: &World) {
    use crate::catvolleyball::{AUDIO_COLLISION, AUDIO_MUSIC, AUDIO_SCORE};

    let (sound_effects, music) = {
        let mut sink = world.write_resource::<AudioSink>();
        // Reduce starting volume as it starts too high
        sink.set_volume(0.25);

        let loader = world.read_resource::<Loader>();
        let music = AUDIO_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        
        let music = Music { music };
        let sound = Sounds {
            collision_sfx: load_audio_track(&loader, &world, AUDIO_COLLISION),
            score_sfx: load_audio_track(&loader, &world, AUDIO_SCORE),
        };

        (sound, music)
    };

    // Add sound effects to world. 
    // Done here because they cannot be added to world while Loader is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}