"""scream into ur default microphone to unlock. needs `pip install . sounddevice numpy`. warn ur roommates."""

import sounddevice

from shout_to_unlock import ShoutLock

SAMPLE_RATE = 48_000
BLOCK_SIZE = 1024

lock = ShoutLock(minimum_level_dbfs=-15.0, minimum_duration_seconds=2.0)

with sounddevice.InputStream(samplerate=SAMPLE_RATE, channels=1, dtype="float32", blocksize=BLOCK_SIZE) as stream:
    print("SHOUT TO UNLOCK. go on. louder.")
    while not lock:
        block, _overflowed = stream.read(BLOCK_SIZE)
        lock.process(block[:, 0], SAMPLE_RATE)
        bar = "#" * int(lock.progress * 40)
        print(f"\r{lock.last_level_dbfs:6.1f} dBFS [{bar:<40}] {lock.loud_seconds:4.1f} s", end="", flush=True)
print("\nUNLOCKED. ur throat hurts but ur free.")
