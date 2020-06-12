# Components

- interfere
	- Purpose: being a musical instrument
	- Sub-components: interfere-vst

- interfere-vst:
	- Purpose: being the intermediary of interfere core and a VST host
	- Sub-components: interfere-core

- interfere-core:
	- Purpose: being a platform and API independent core of the synthesizer
	- Sub-components: Instance

- Instance:
	- Purpose: being a single instance of the plugin
	- Sub-components: IndependentUpdater, DependentDeriver, AudioGenerator

- IndependentUpdater:
	- Purpose: updating the independent values (also keeps track of elapsed
	  time)
	- Sub-components: MidiProcessor, Envelope, LFO, Pitch

- MidiProcessor:
	- Purpose: use midi messages to update the internal state on notes
	  being played
	- Sub-components:

- Envelope:
	- Purpose: generate values based on time, note info, and an internal
	  ADSR curve (uses global independents)
	- Sub-components:

- LFO:
	- Purpose: generate values based on time and an internal wave (uses
	  global independents)
	- Sub-components:

- Pitch:
	- Purpose: generate pitch based on the notes currently being played
	- Sub-components:

- DependentDeriver:
	- Purpose: deriving dependent variables based on independent variables
	- Sub-components: 

- AudioGenerator:
	- Purpose: generating audio for all voices, and mixing it into one
	  buffer
	- Sub-components: Oscillator, Filter

- Oscillator:
	- Purpose: synthesizing audio waves (based on dependent variables)
	- Sub-components: 

- Filter:
	- Purpose: filtering out a specific part of the audio spectrum (based
	  on dependent variables)
	- Sub-components: 

