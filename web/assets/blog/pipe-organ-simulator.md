# Building a Physical Pipe Organ Interface with Arduino and a Shift Register

I've been playing pipe organ for a while, and at some point the gap between "I want to practice at home" and "I can afford a real pipe organ" became a software problem. This post is about building a physical manual (keyboard) interface for a pipe organ simulator — connecting real organ keys to software using an Arduino Pro Micro and a 74HC165 shift register.

## The Software Side: GrandOrgue and RustyPipes

Before touching hardware, you need something to receive MIDI and produce organ sounds.

**GrandOrgue** is the old reliable. It's an open-source pipe organ simulator that's been around for years, runs on Linux/Windows/macOS, and supports sample sets — recordings of real pipe organs that get mapped to MIDI notes. You can download free sample sets or buy professional ones. GrandOrgue receives MIDI input and plays back the samples with proper attack, sustain, and release envelopes. It's not glamorous software, but it works extremely well.

**RustyPipes** is a newer project — a pipe organ simulator written in Rust. It's still evolving, but the architecture is interesting: it treats each pipe as an independent audio source with its own convolution reverb and wind model. The Rust codebase is clean and modular, which makes it a good project to study if you want to understand real-time audio programming. It also accepts MIDI input, so any interface that speaks MIDI will work with it.

For this project, either works. I used GrandOrgue for testing because the sample set support is mature.

## The Hardware: One Manual, 61 Keys

A standard organ manual has 61 keys (5 octaves). Each key is essentially a switch — pressed or released. You need to read all 61 switch states as fast as possible and send MIDI note-on/note-off messages.

**The problem:** the Arduino Pro Micro (ATmega32u4) has only ~20 I/O pins. 61 keys won't fit directly. You need a way to expand inputs.

**The solution:** the **74HC165** parallel-in, serial-out shift register. Each chip reads 8 inputs in parallel and shifts them out serially on a clock signal. Chain 8 of them together and you can read 64 inputs using just 3 microcontroller pins: `CLOCK`, `LOAD` (latch), and `DATA`.

```
MCU pin --- LOAD  --> [74HC165 #1] --> [74HC165 #2] --> ... --> [74HC165 #8]
MCU pin --- CLOCK --> all chips (daisy-chained)
MCU pin <-- DATA  <-- Q7 of last chip (serial output chain)
```

Each chip's `Q7` (serial output) connects to the `SER` (serial input) of the next chip in the chain. You shift out 64 bits total to read all 8 chips at once.

## Wiring the 74HC165

The 74HC165 pinout:

| Pin | Name | Function |
|-----|------|----------|
| 1  | SH/LD | Shift (HIGH) or Load (LOW) — latch parallel inputs |
| 2  | CLK | Clock input |
| 3–6 | D3–D6 | Parallel data inputs |
| 7  | D7 | Parallel data input |
| 9  | Q7 | Serial output |
| 10 | SER | Serial input (from previous chip) |
| 11–14 | D0–D3 | Parallel data inputs |
| 15 | CLK INH | Clock inhibit (tie LOW to enable) |
| 16 | VCC | 3.3V or 5V |
| 8  | GND | Ground |

For each key, connect one side of the switch to a data input pin (D0–D7) and the other side to GND. Enable the internal pull-up in software, or add 10kΩ pull-up resistors to VCC on each input line.

Chain 8 chips for 64 inputs (61 keys + 3 spares):

```
Pro Micro pin 6  --> SH/LD  (all chips share this line)
Pro Micro pin 7  --> CLK    (all chips share this line)
Pro Micro pin 8  <-- Q7     (only the last chip's serial output)
First chip SER   --> GND    (no previous chip)
Chip N Q7        --> Chip N+1 SER
```

## The Arduino Code

```c
#include <MIDIUSB.h>

#define PIN_LOAD  6   /* SH/LD: LOW to latch, HIGH to shift */
#define PIN_CLOCK 7   /* Clock */
#define PIN_DATA  8   /* Serial data from last chip */

#define NUM_CHIPS 8
#define NUM_KEYS  61
#define MIDI_CH   0   /* MIDI channel 1 (zero-indexed) */
#define BASE_NOTE 36  /* C2 — lowest key maps to MIDI note 36 */

/* Store previous key states to detect changes */
uint8_t prev_state[NUM_CHIPS] = {0};

/* Read all shift registers and return 64 bits in an 8-byte array.
   Returns 1 if a key is pressed (input pulled LOW), 0 if released. */
void read_keys(uint8_t *out) {
    /* Pulse LOAD low to latch all parallel inputs simultaneously */
    digitalWrite(PIN_LOAD, LOW);
    delayMicroseconds(5);
    digitalWrite(PIN_LOAD, HIGH);

    /* Shift out 8 bytes (64 bits) from the chain */
    for (int chip = 0; chip < NUM_CHIPS; chip++) {
        uint8_t byte = 0;
        for (int bit = 7; bit >= 0; bit--) {
            /* Read the current bit from DATA pin */
            int val = digitalRead(PIN_DATA);
            /* Keys are active LOW (pulled to GND when pressed) */
            if (val == LOW)
                byte |= (1 << bit);

            /* Pulse clock to advance the shift register */
            digitalWrite(PIN_CLOCK, HIGH);
            delayMicroseconds(2);
            digitalWrite(PIN_CLOCK, LOW);
        }
        out[chip] = byte;
    }
}

void setup() {
    pinMode(PIN_LOAD,  OUTPUT);
    pinMode(PIN_CLOCK, OUTPUT);
    pinMode(PIN_DATA,  INPUT);

    digitalWrite(PIN_LOAD,  HIGH);
    digitalWrite(PIN_CLOCK, LOW);
}

void loop() {
    uint8_t current[NUM_CHIPS] = {0};
    read_keys(current);

    /* Compare current state to previous; send MIDI on any change */
    for (int chip = 0; chip < NUM_CHIPS; chip++) {
        uint8_t changed = current[chip] ^ prev_state[chip];
        if (!changed) continue; /* Nothing changed on this chip */

        for (int bit = 7; bit >= 0; bit--) {
            if (!(changed & (1 << bit))) continue; /* This bit didn't change */

            int key_index = chip * 8 + (7 - bit);
            if (key_index >= NUM_KEYS) continue; /* Ignore spare inputs */

            uint8_t note = BASE_NOTE + key_index;
            bool pressed = (current[chip] >> bit) & 1;

            if (pressed) {
                /* Key pressed: send Note On, velocity 100 */
                midiEventPacket_t on = {0x09, 0x90 | MIDI_CH, note, 100};
                MidiUSB.sendMIDI(on);
            } else {
                /* Key released: send Note Off */
                midiEventPacket_t off = {0x08, 0x80 | MIDI_CH, note, 0};
                MidiUSB.sendMIDI(off);
            }
        }
        prev_state[chip] = current[chip];
    }

    MidiUSB.flush(); /* Send buffered MIDI packets */
    delayMicroseconds(500); /* ~2000 scans/sec, more than enough */
}
```

The ATmega32u4 has native USB and the `MIDIUSB` library exposes it as a USB MIDI device — plug it into your computer and it shows up as a MIDI input. No drivers needed on Linux.

## Connecting to GrandOrgue

Once the Arduino is recognized as a MIDI device:

1. Open GrandOrgue and load a sample set (the [Sonus Paradisi](https://www.sonusparadisi.cz/) free samples are a good start)
2. Go to **Audio/MIDI Settings → MIDI Devices** and enable your Arduino MIDI device
3. In the organ's MIDI configuration, assign the manual to the MIDI channel you set in the sketch (channel 1 in our case)
4. Play a key — you should hear a pipe

For RustyPipes, the setup is similar — it reads from any MIDI input device via the system MIDI API (`alsa_midi` on Linux, `coremidi` on macOS).

## Latency

The scan loop runs at ~2000 Hz, so the worst-case key detection delay is 0.5ms. USB MIDI adds another ~1ms of latency. GrandOrgue's audio buffer adds a few more milliseconds depending on your JACK/ALSA settings. Total round-trip latency is typically 5–15ms with a tuned audio setup — perfectly playable.

## What's Next

A single manual is a start. A real organ has two or three manuals plus a pedalboard. Each manual is an independent shift register chain on a separate MIDI channel. The pedalboard is 32 keys — just 4 more 74HC165 chips.

Stops (the register controls) are toggle switches — same circuit, different MIDI messages (typically program change or control change). Expression pedals are potentiometers read through the ADC.

The whole thing ends up being a fairly compact PCB, a handful of chips, and a few hundred lines of C. And then you load a Gothic cathedral organ sample set and suddenly your living room sounds very different.
