// C++17 RAII wrapper for shout_to_unlock.h. it frees itself. self care
#ifndef SHOUT_TO_UNLOCK_HPP
#define SHOUT_TO_UNLOCK_HPP

#include <cstddef>
#include <cstdint>
#include <memory>
#include <stdexcept>
#include <vector>

#include "shout_to_unlock.h"

namespace shout_to_unlock {

class ShoutLock {
public:
    // throws std::invalid_argument if minimum_duration_seconds is negative, not finite, or absurdly large.
    ShoutLock(float minimum_level_dbfs, double minimum_duration_seconds)
        : lock_(shout_to_unlock_new(minimum_level_dbfs, minimum_duration_seconds)) {
        if (!lock_) {
            throw std::invalid_argument("shout_to_unlock: invalid minimum duration (time only goes forward bestie)");
        }
    }

    bool process(const float *samples, std::size_t sample_count, std::uint32_t sample_rate) {
        return shout_to_unlock_process(lock_.get(), samples, sample_count, sample_rate);
    }
    bool process(const std::vector<float> &samples, std::uint32_t sample_rate) {
        return process(samples.data(), samples.size(), sample_rate);
    }
    bool process_level(float level_dbfs, double duration_seconds) {
        return shout_to_unlock_process_level(lock_.get(), level_dbfs, duration_seconds);
    }

    [[nodiscard]] bool is_unlocked() const { return shout_to_unlock_is_unlocked(lock_.get()); }
    [[nodiscard]] double loud_seconds() const { return shout_to_unlock_loud_seconds(lock_.get()); }
    [[nodiscard]] double remaining_seconds() const { return shout_to_unlock_remaining_seconds(lock_.get()); }
    [[nodiscard]] double progress() const { return shout_to_unlock_progress(lock_.get()); }
    [[nodiscard]] double total_seconds() const { return shout_to_unlock_total_seconds(lock_.get()); }
    [[nodiscard]] float last_level_dbfs() const { return shout_to_unlock_last_level_dbfs(lock_.get()); }
    [[nodiscard]] float peak_level_dbfs() const { return shout_to_unlock_peak_level_dbfs(lock_.get()); }
    [[nodiscard]] float minimum_level_dbfs() const { return shout_to_unlock_minimum_level_dbfs(lock_.get()); }
    [[nodiscard]] double minimum_seconds() const { return shout_to_unlock_minimum_seconds(lock_.get()); }

    void reset() { shout_to_unlock_reset(lock_.get()); }

private:
    struct Deleter {
        void operator()(::ShoutLock *lock) const { shout_to_unlock_free(lock); }
    };
    std::unique_ptr<::ShoutLock, Deleter> lock_;
};

[[nodiscard]] inline float level_dbfs(const std::vector<float> &samples) {
    return shout_to_unlock_level_dbfs(samples.data(), samples.size());
}

}  // namespace shout_to_unlock

#endif
