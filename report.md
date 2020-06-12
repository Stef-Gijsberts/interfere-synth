# Introduction
<!-- Give a general description of your system. For example, what are the
goals of the system? Who are the users of the system? Why are they interested
in using this system? -->
This project report describes *Interfere*, a musical instrument.

Musicians can use Interfere in a Digital Audio Workstation (DAW). Such a
computer progam is a digital studio for music, where instruments and effects
are organized and used.

# Description

## Properties
<!-- Global description of the product and its properties. Although this is a
global description, it is important that you try to describe the properties
explicitly. So do not accept any implicit assumptions. The description can be
illustrated with information about the user interface, for example
screenshots or other illustrative information.-->
Interfere implements the VST API. This means that any VST Host (probably a DAW)
can use it as an instrument.

There are four important dataflows:

- plugin information, going from the plugin to the host;
- MIDI messages, going from the host to the plugin;
- parameter updates, going from the host to the plugin;
- audio samples, going from the plugin to the host.


### Plugin Information
If the host asks Interfere for information, Interfere provides it. Information
the host can ask for includes:

- The plugin's name, creator, number of inputs and number of outputs;
- The number of parameters;
- The name and value of a parameter as a specific index.

### MIDI messages
A MIDI message is a message describing an instruction that has to to with notes being
played on a keyboard.

If the host sends Interfere a MIDI message, Interfere should handle it correctly.

For example, between the moment that a MIDI message is received to start a
certain note and the moment that a message is received to stop it, Interfere
will make sure that a sine wave with the frequency corresponding to that note
is audible.

### Parameter updates
If the host orders Interfere to update a specific parameter, Interfere must update
its state accordingly.

### Audio samples
The host can order Interfere to fill a buffer with audio samples. Interfere will
then generate sound, based on the current parameter values and past MIDI messages.


## Product Justification
<!-- Here you explain why it is worthwhile to build your system. What other
(similar) products are available? What are the typical (new and innovative)
contributions of your system? -->
Many digital synthesizers exist, of which some are nearly impossible to match:
Sylenth1, Serum, Massive and Ableton Live's Operator are great digital
synthesizers.

Apart from being great synthesizers, they share another trait: they are all
released for just Windows and OSX. And, most of these synthesizers allow for
just one interface (VST). This limits the choice of operating system and DAW.

But Interfere is written in platform independent code. It can be easily adapted
to be used with other standards than VST, and it can be compiled for any
platform supported by the Rust programming language.

By being written in Rust, Interfere also gets safety and efficiency for free.

Also, the simple and flexible parameter model of Interfere allows for more
customization than on other synthesizers.

## Specifications
<!-- A more detailed description of the properties mentioned in section 2.1.
It would be good to give some underlying models, for example Use Case
Diagrams with an explanatory description. -->


# Design

## Global Design
<!-- Describe the components (modules) of your system and the
interconnections between those components. You should explicitly describe the
role of each component. Explain why the components together actually do what
they are supposed to do. Make sure that this distinction between the
components is in line with the way in which you implemented the system. -->
Interfere consists of two parts: *interfere-core* and *interfere-vst*.

## Detailed Design
<!-- Give a detailed design in terms of data structures and algorithms, for
example the classes, methods and attributes. Explain the idea behind the most
important methods and attributes. Make sure that your descriptions are clear
and consistent, such that a future programmer would be able to further
improve or extend the system. -->

## Design Justification
<!-- Explain why your design is a good design. Here you should focus on your
design decisions including technical details. Give possible design
alternatives and describe how you chose between these alternatives. -->
The DAW is like a drug customer, getting audio data from its dealer,
*interfere-vst*, which gets its audio from the supplier, *interfere-core*.

The customer and supplier never meet. Instead, they only ever meet the dealer.
If the dealer would sell to another customer, the supplier would never notice.

The advantages of this design show when another standard than VST must be
implemented. Then, just like how the dealer can sell to another customer
without the supplier knowing, another implementation can provide audio to a DAW
using another protocol. This saves time, because the 'dealer' is smaller and
simpler than *interfere-core*.


# Evaluation
<!-- Here you should evaluate your project, for example: are you satisfied
with your product? What are the unsolved issues? Are you satisfied with your
development process (that is the process which resulted in your product)?

What did you learn? Describe and analyse the factors which determined your
process and product. What are the consequences for future work? How would you
operate in a future project?
-->
