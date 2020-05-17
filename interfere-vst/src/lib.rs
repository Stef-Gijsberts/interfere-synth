use std::sync::Arc;

use interfere_core::{Instance, Parameters};

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin, PluginParameters};


// A macro to generate the necessary exposed functions for the library to be recognized as a VST
// plugin.
use vst::plugin_main;
plugin_main!(InterfereVST);


struct InterfereVST {
    instance: Instance,
    parameters: Arc<VSTParameters>,
    sample_rate_hz: f64,
    buffer: [(f64, f64); 1024],
    idx_buffer_head: usize,
}

struct VSTParameters(Parameters);

impl Default for InterfereVST {
    fn default() -> InterfereVST {
        // The samplerate used as long as there is no rate provided yet through
        // the set_sample_rate function
        const DEFAULT_SAMPLE_RATE: f64 = 44100.0;

        let instance: Instance = Instance::default();
        let parameters: Arc<VSTParameters> = Arc::new(VSTParameters(Parameters::default()));

        // 1024 is a value that is sufficiently large for the processor to keep
        // up (processing more at once is more efficient), but not so big that it
        // causes a notable delay
        let buffer = [(0.0f64, 0.0f64); 1024];

        InterfereVST {
            instance,
            parameters,
            sample_rate_hz: DEFAULT_SAMPLE_RATE,
            buffer,
            idx_buffer_head: buffer.len(),
        }
    }
}

impl Plugin for InterfereVST {
    fn get_info(&self) -> Info {
        Info {
            name: "Interfere".to_string(),
            vendor: "Stef Gijsberts".to_string(),
            unique_id: 129747,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: self.parameters.len(),
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate_hz = rate as f64;
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.parameters) as Arc<dyn PluginParameters>
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.instance.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn process(&mut self, in_out_chunk: &mut AudioBuffer<f32>) {
        let (_, outputs) = in_out_chunk.split();

        assert!(outputs.len() == 2, "Two output channels are assumed");

        // Iterate over outputs as (&mut f32, &mut f32)
        // found at: https://github.com/RustAudio/vst-rs/blob/master/examples/dimension_expander.rs
        let (mut l, mut r) = outputs.split_at_mut(1);
        let stereo_out = l[0].iter_mut().zip(r[0].iter_mut());

        while let Some((l_out, r_out)) = stereo_out.next() {
            let frames_available = self.buffer.len() - self.idx_buffer_head > 0;

            if !frames_available {
                self.instance.audio_requested(self.buffer);
                self.idx_buffer_head = 0;
            }

            *l_out = self.buffer[self.idx_buffer_head].0 as f32;
            *r_out = self.buffer[self.idx_buffer_head].1 as f32;
            self.idx_buffer_head += 1;
        }
    }
}

impl PluginParameters for VSTParameters {
    fn get_parameter_label(&self, index: i32) -> String {
        self.get_parameter_label(index)
    }

    fn get_parameter_text(&self, index: i32) -> String {
        self.get_parameter_text(index)
    }

    fn get_parameter_name(&self, index: i32) -> String {
        self.get_parameter_name(index)
    }

    fn get_parameter(&self, index: i32) -> f32 {
        self.get_parameter(index)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        self.set_parameter(index, value)
    }
}

