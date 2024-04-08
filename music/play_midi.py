import mido
import redis
from pathlib import Path


def note_to_freq(note_number):
    return 440 * (2 ** ((note_number - 69) / 12))

def midi_note_to_name(midi_note):
    notes = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']
    octave = (midi_note // 12) - 1
    note_name = notes[midi_note % 12]
    return f"{note_name}{octave}"

def print_notes(filename, r):
    mid = mido.MidiFile(filename)

    for msg in mid.play():
        if msg.type == 'note_on':
            # print(f"Note On: Channel={msg.channel}, Note={msg.note}, Velocity={msg.velocity}")
            print(note_to_freq(msg.note), midi_note_to_name(msg.note))
            freq = note_to_freq(msg.note)
            # only play notes above c4
            if freq > 261.63:
                r.publish("music", freq)
        elif msg.type == 'note_off':
            r.publish("music", 0)

if __name__ == '__main__':
    # midi_file = Path(__file__).resolve().parent / 'bad-apple-melody.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file
    midi_file = Path(__file__).resolve().parent / 'river-flows.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file
    r = redis.Redis(host='localhost', port=6379, db=0, decode_responses=True)
    print_notes(midi_file, r)