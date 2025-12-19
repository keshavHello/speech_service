from gtts import gTTS
import os

def text_to_speech(input_file='input.txt', output_file='output.mp3'):
    if not os.path.exists(input_file):
        print(f"⚠️ File '{input_file}' not found.")
        return

    with open(input_file, 'r', encoding='utf-8') as f:
        text = f.read().strip()

    if not text:
        print("⚠️ Input file is empty.")
        return

    # Convert text to speech using online Google TTS
    tts = gTTS(text=text, lang='en', slow=False, tld='co.in')
    tts.save(output_file)
    print(f"✅ Audio saved as '{output_file}'")

if __name__ == "__main__":
    text_to_speech('input.txt', 'output.mp3')
