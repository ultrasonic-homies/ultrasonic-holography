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
            freq = note_to_freq(msg.note)

            # print(f"Note On: Channel={msg.channel}, Note={msg.note}, Velocity={msg.velocity}")
            # print(note_to_freq(msg.note), midi_note_to_name(msg.note))
            # we can't play e 5, so bring anything higher down to below e5
            r.publish("music", f"{freq},true")
        elif msg.type == 'note_off':
            freq = note_to_freq(msg.note)

            r.publish("music", f"{freq},false")

if __name__ == '__main__':
    
    file = "pokemon2.mid"
    midi_file = Path(__file__).resolve().parent / file  # Replace 'your_midi_file.mid' with the path to your MIDI file
    # midi_file = Path(__file__).resolve().parent / 'river-flows.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file
    # midi_file = Path(__file__).resolve().parent / 'underground.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file
    # midi_file = Path(__file__).resolve().parent / 'green-hill-zone.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file

    # midi_file = Path(__file__).resolve().parent / 'badapple.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file
    r = redis.Redis(host='localhost', port=6379, db=0, decode_responses=True)
    # turn it off
    print_notes(midi_file, r)
    r.publish("music", f"{300},false")
    r.publish("music", f"{200},false")