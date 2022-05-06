# Midiline

Midiline is a commandline tool that sends MIDI messages by reading
textual representations of the messages. For example, the command
`noteon 64 20 1` sends a `noteon` message for the note 64 with a
velocity of 20 on channel 1; the hex representation of the outgoing
MIDI message is `91 40 01`.

## Usage

The program can be run by giving it a single argument with the name of
the MIDI output to be used. If no argument is given then i prints a
list of the available MIDI outputs and exits.

Once running, the following commands are supported:
* `noteon NOTE VELOCITY CHANNEL`: Send a `noteon` message. All values are in decimal.
* `cc CONTROL VALUE CHANNEL`: Send a `cc` message. All values are in decimal.

## Building

Midiline is written in Rust and can be built with cargo using the
command `cargo build`.