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

## Product Justification
<!-- Here you explain why it is worthwhile to build your system. What other
(similar) products are available? What are the typical (new and innovative)
contributions of your system? -->

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

The DAW is like a drug customer, asking audio data from its dealer, 
*interfere-vst*, which gets its audio from the supplier, *interfere-core*.

The customer and supplier never meet. Instead, they
only ever meet the dealer. If the dealer would sell to another customer, the supplier would never notice.

The benifits of this design show when another standard than
VST must be implemented. Then, just like how the dealer can sell to another
customer without the supplier knowing, another implementation can provide audio to a DAW
using another protocol. This saves time, because the 'dealer' is smaller and simpler than *interfere-core*.

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

# Evaluation
<!-- Here you should evaluate your project, for example: are you satisfied
with your product? What are the unsolved issues? Are you satisfied with your
development process (that is the process which resulted in your product)?

What did you learn? Describe and analyse the factors which determined your
process and product. What are the consequences for future work? How would you
operate in a future project?
-->
