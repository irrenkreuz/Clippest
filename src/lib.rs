use nih_plug::prelude::*;
use std::num::NonZeroU32;
use std::sync::Arc;

const ONE_MINUS_EPS: f32 = 1.0 - f32::EPSILON / 2.0;

#[derive(Params)]
struct ClippestParams {}

struct Clippest {
    params: Arc<ClippestParams>,
}

impl Default for Clippest {
    fn default() -> Self {
        Self {
            params: Arc::new(ClippestParams {}),
        }
    }
}

impl Plugin for Clippest {
    const NAME: &'static str = "Clippest";
    const VENDOR: &'static str = "";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                *sample = sample.tanh().clamp(-ONE_MINUS_EPS, ONE_MINUS_EPS);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Clippest {
    const CLAP_ID: &'static str = "com.clippest.clippest";
    const CLAP_DESCRIPTION: Option<&'static str> = None;
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Distortion,
    ];
}

nih_export_clap!(Clippest);
