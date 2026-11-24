/* a lock that only opens if u YELL at it. loud enough, long enough. xoxo
 *
 * each buffer gets a vibe check: its RMS level in dBFS, 20*log10(sqrt(sum(x^2)/N)).
 * every buffer at or above the minimum level drops its duration into the loud-time jar.
 * the loud time does not need to be one long scream. breathing is allowed.
 * once the jar hits the minimum duration, the lock opens and stays open until a reset. no take-backs.
 *
 * every function that takes a ShoutLock pointer needs a real one from shout_to_unlock_new.
 * garbage pointers = UB = sad.
 */
#ifndef SHOUT_TO_UNLOCK_H
#define SHOUT_TO_UNLOCK_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ShoutLock ShoutLock;

/* returns NULL if minimum_duration_seconds is negative, not finite, or absurdly large. no time travel. */
ShoutLock *shout_to_unlock_new(float minimum_level_dbfs, double minimum_duration_seconds);
/* NULL does nothing. freeing nothing is free. */
void shout_to_unlock_free(ShoutLock *lock);

/* feed it one buffer of mono samples in [-1, 1]. returns shout_to_unlock_is_unlocked. */
bool shout_to_unlock_process(ShoutLock *lock, const float *samples, size_t sample_count,
                             uint32_t sample_rate);
/* measured the level urself? overachiever. returns shout_to_unlock_is_unlocked. */
bool shout_to_unlock_process_level(ShoutLock *lock, float level_dbfs, double duration_seconds);

bool shout_to_unlock_is_unlocked(const ShoutLock *lock);
/* how long u've been loud enough, in total. ur scream resume. */
double shout_to_unlock_loud_seconds(const ShoutLock *lock);
/* how much more screaming until freedom. */
double shout_to_unlock_remaining_seconds(const ShoutLock *lock);
/* loud_seconds / minimum_seconds, clamped to [0, 1]. a progress bar for ur lungs. */
double shout_to_unlock_progress(const ShoutLock *lock);
/* total time of everything u fed it, loud or sad. */
double shout_to_unlock_total_seconds(const ShoutLock *lock);
/* level of the latest buffer. -INFINITY before the first one. */
float shout_to_unlock_last_level_dbfs(const ShoutLock *lock);
/* ur personal best. -INFINITY before the first buffer. */
float shout_to_unlock_peak_level_dbfs(const ShoutLock *lock);
float shout_to_unlock_minimum_level_dbfs(const ShoutLock *lock);
double shout_to_unlock_minimum_seconds(const ShoutLock *lock);

/* slams the lock shut and forgets everything. the limits stay. fresh start, same standards. */
void shout_to_unlock_reset(ShoutLock *lock);

/* RMS level in dBFS. silence gives -INFINITY. very dramatic of it. */
float shout_to_unlock_level_dbfs(const float *samples, size_t sample_count);

#ifdef __cplusplus
}
#endif

#endif
