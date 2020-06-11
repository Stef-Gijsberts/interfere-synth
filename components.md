# Components

- interfere
	- Goal: musical instrument
	- Sub-components: interfere-vst

- interfere-vst:
	- Goal: being the intermediary of interfere core and a VST host
	- Sub-components: interfere-core

- interfere-core:
	- Goal: being a platform and API independent core of the synthesizer
	- Sub-components: Instance

- Instance:
	- Goal: being a single instance of the plugin
	- Sub-components: IndependentUpdater, DependentDeriver, SoundGenerator

- IndependentUpdater:
	- Purpose: updating the independent values (also keeps track of elapsed time)
	- Sub-components: MidiProcessor, Envelope, LFO

- MidiProcessor:
	- Purpose: use midi messages to update the internal state on notes being played
	- Sub-components:

- Envelope:
	- Purpose: generate values based on time, note info, and an internal ADSR curve (uses global independents)
	- Sub-components:

- LFO:
	- Purpose: generate values based on time and an internal wave (uses global independents)
	- Sub-components:

- DependentDeriver:
	- Purpose: deriving dependent variables based on independent variables
	- Sub-components: 

- SoundGenerator:
	- Purpose: generating sound
	- Sub-components: Oscillator, Filter, Mixer

- Oscillator:
	- Purpose: synthesizing audio waves (based on certain dependent variables)
	- Sub-components: 

- Filter:
	- Purpose: filtering out a specific part of the audio spectrum (based on certain dependent variables)
	- Sub-components: 

- Mixer:
	- Purpose: combining audio of voices (16x mono) into one stereo stream (based on certain dependent variables)
	- Sub-components:

