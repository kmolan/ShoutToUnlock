// g++ -std=c++17 -Iinclude examples/cpp_example.cpp -Ltarget/release -lshout_to_unlock -o examples/cpp_example
#include <cmath>
#include <cstdio>
#include <vector>

#include "shout_to_unlock.hpp"

int main() {
    constexpr std::uint32_t sample_rate = 48'000;
    shout_to_unlock::ShoutLock lock(-20.0F, 0.5);

    std::vector<float> shout(4800);  // 100 ms of A4 at the top of ur lungs
    for (std::size_t index = 0; index < shout.size(); ++index) {
        shout[index] = 0.8F * std::sin(2.0F * 3.14159265F * 440.0F * index / sample_rate);
    }
    const std::vector<float> whisper(4800, 0.01F);

    for (int buffer = 0; !lock.is_unlocked(); ++buffer) {
        lock.process(buffer % 2 == 0 ? shout : whisper, sample_rate);
        std::printf("buffer %2d: level %6.1f dBFS, loud %.2f s, remaining %.2f s, progress %3.0f%%\n", buffer,
                    lock.last_level_dbfs(), lock.loud_seconds(), lock.remaining_seconds(), lock.progress() * 100.0);
    }
    std::printf("UNLOCKED after %.2f s of input, peak %.1f dBFS. the neighbours have been notified.\n", lock.total_seconds(), lock.peak_level_dbfs());
    return 0;
}
