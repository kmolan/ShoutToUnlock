//! a lock that only opens if u YELL at it. loud enough, long enough. that's the whole personality
//!
//! each buffer gets a vibe check: its RMS level in dBFS, `20·log₁₀(√(Σx²/N))`.
//! a full-scale sine wave is ~−3 dBFS, for reference, in case ur a sine wave.
//! every buffer at or above the minimum level drops its duration into the loud-time jar.
//! the loud time does not need to be one long scream. breathe. hydrate. resume screaming.
//! once the jar hits the minimum duration, the lock opens and STAYS open until [`ShoutLock::reset`]. no take-backs.
//!
//! ```
//! use std::time::Duration;
//! use shout_to_unlock::ShoutLock;
//!
//! let mut lock = ShoutLock::new(-20.0, Duration::from_millis(500));
//! let shout = vec![0.5_f32; 4800]; // 100 ms of pure rage at 48 kHz, −6 dBFS
//! for _ in 0..4 {
//!     assert!(!lock.process(&shout, 48_000));
//! }
//! assert!(lock.process(&shout, 48_000));
//! ```

#[cfg(not(target_family = "wasm"))]
mod ffi;
#[cfg(feature = "mobile")]
mod mobile;
#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();
#[cfg(feature = "python")]
mod python;
#[cfg(feature = "wasm")]
mod wasm;

use std::time::Duration;

/// the lock. opens after enough total time at or above the minimum level. not complicated, just loud.
#[derive(Debug, Clone)]
pub struct ShoutLock {
    minimum_level_dbfs: f32,
    minimum_duration: Duration,
    loud_duration: Duration,
    total_duration: Duration,
    last_level_dbfs: f32,
    peak_level_dbfs: f32,
}

impl ShoutLock {
    /// a fresh lock. closed. silent. waiting.
    #[must_use]
    pub fn new(minimum_level_dbfs: f32, minimum_duration: Duration) -> Self {
        Self {
            minimum_level_dbfs,
            minimum_duration,
            loud_duration: Duration::ZERO,
            total_duration: Duration::ZERO,
            last_level_dbfs: f32::NEG_INFINITY,
            peak_level_dbfs: f32::NEG_INFINITY,
        }
    }

    /// feed it one buffer of mono samples in `[-1, 1]`. returns [`Self::is_unlocked`].
    ///
    /// the whole buffer gets ONE level, so buffer length = time resolution. big buffer, blurry vibes.
    /// an empty buffer or a `sample_rate` of 0 does nothing. u cannot scream at zero hertz babe.
    pub fn process(&mut self, samples: &[f32], sample_rate: u32) -> bool {
        if samples.is_empty() || sample_rate == 0 {
            return self.is_unlocked();
        }
        let duration =
            Duration::from_nanos(samples.len() as u64 * 1_000_000_000 / u64::from(sample_rate));
        self.process_level(level_dbfs(samples), duration)
    }

    /// measured the level urself? overachiever. hand it over with a duration. returns [`Self::is_unlocked`].
    pub fn process_level(&mut self, level_dbfs: f32, duration: Duration) -> bool {
        self.last_level_dbfs = level_dbfs;
        self.peak_level_dbfs = self.peak_level_dbfs.max(level_dbfs);
        self.total_duration += duration;
        if level_dbfs >= self.minimum_level_dbfs {
            self.loud_duration += duration;
        }
        self.is_unlocked()
    }

    /// `true` if the loud time is nonzero AND at or above the minimum. the big question. the only question.
    #[must_use]
    pub fn is_unlocked(&self) -> bool {
        !self.loud_duration.is_zero() && self.loud_duration >= self.minimum_duration
    }

    /// how long u've been loud enough, in total. ur scream résumé.
    #[must_use]
    pub fn loud_duration(&self) -> Duration {
        self.loud_duration
    }

    /// how much more screaming until freedom.
    #[must_use]
    pub fn remaining_duration(&self) -> Duration {
        self.minimum_duration.saturating_sub(self.loud_duration)
    }

    /// `loud_duration / minimum_duration`, clamped to `[0, 1]`. a progress bar for ur lungs.
    #[must_use]
    pub fn progress(&self) -> f64 {
        if self.minimum_duration.is_zero() {
            return if self.is_unlocked() { 1.0 } else { 0.0 };
        }
        (self.loud_duration.as_secs_f64() / self.minimum_duration.as_secs_f64()).min(1.0)
    }

    /// total time of everything u fed it, loud or sad.
    #[must_use]
    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    /// level of the latest buffer. `-∞` before the first one. the calm before the scream.
    #[must_use]
    pub fn last_level_dbfs(&self) -> f32 {
        self.last_level_dbfs
    }

    /// loudest level since creation or the last reset. ur personal best. `-∞` before the first buffer.
    #[must_use]
    pub fn peak_level_dbfs(&self) -> f32 {
        self.peak_level_dbfs
    }

    #[must_use]
    pub fn minimum_level_dbfs(&self) -> f32 {
        self.minimum_level_dbfs
    }

    #[must_use]
    pub fn minimum_duration(&self) -> Duration {
        self.minimum_duration
    }

    /// slams the lock shut and forgets everything. the limits stay. fresh start, same standards.
    pub fn reset(&mut self) {
        *self = Self::new(self.minimum_level_dbfs, self.minimum_duration);
    }
}

/// RMS level in dBFS: `20·log₁₀(√(Σx²/N))`. silence gives `-∞`. very dramatic of it.
#[must_use]
pub fn level_dbfs(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return f32::NEG_INFINITY;
    }
    let sum_of_squares: f64 = samples
        .iter()
        .map(|&sample| f64::from(sample).powi(2))
        .sum();
    let root_mean_square = (sum_of_squares / samples.len() as f64).sqrt();
    (20.0 * root_mean_square.log10()) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: u32 = 48_000;

    fn buffer(amplitude: f32, milliseconds: u32) -> Vec<f32> {
        vec![amplitude; (SAMPLE_RATE / 1000 * milliseconds) as usize]
    }

    #[test]
    fn closed_by_default() {
        let lock = ShoutLock::new(-20.0, Duration::from_secs(1));
        assert!(!lock.is_unlocked());
        assert_eq!(lock.progress(), 0.0);
        assert_eq!(lock.last_level_dbfs(), f32::NEG_INFINITY);
    }

    #[test]
    fn zero_minimum_duration_needs_one_loud_buffer() {
        let mut lock = ShoutLock::new(-20.0, Duration::ZERO);
        assert!(!lock.is_unlocked());
        assert!(!lock.process(&buffer(0.01, 10), SAMPLE_RATE));
        assert!(lock.process(&buffer(0.5, 10), SAMPLE_RATE));
    }

    #[test]
    fn loud_time_does_not_need_to_be_continuous() {
        let mut lock = ShoutLock::new(-20.0, Duration::from_millis(300));
        let loud = buffer(0.5, 100);
        let quiet = buffer(0.01, 100);
        for _ in 0..2 {
            assert!(!lock.process(&loud, SAMPLE_RATE));
            assert!(!lock.process(&quiet, SAMPLE_RATE));
        }
        assert_eq!(lock.loud_duration(), Duration::from_millis(200));
        assert_eq!(lock.remaining_duration(), Duration::from_millis(100));
        assert_eq!(lock.total_duration(), Duration::from_millis(400));
        assert!(lock.process(&loud, SAMPLE_RATE));
        assert_eq!(lock.progress(), 1.0);
        assert!(lock.process(&quiet, SAMPLE_RATE));
    }

    #[test]
    fn reset_keeps_limits() {
        let mut lock = ShoutLock::new(-20.0, Duration::from_millis(100));
        assert!(lock.process(&buffer(0.5, 100), SAMPLE_RATE));
        lock.reset();
        assert!(!lock.is_unlocked());
        assert_eq!(lock.loud_duration(), Duration::ZERO);
        assert_eq!(lock.peak_level_dbfs(), f32::NEG_INFINITY);
        assert_eq!(lock.minimum_level_dbfs(), -20.0);
        assert_eq!(lock.minimum_duration(), Duration::from_millis(100));
    }

    #[test]
    fn levels() {
        assert_eq!(level_dbfs(&[0.0; 16]), f32::NEG_INFINITY);
        assert!((level_dbfs(&[1.0, -1.0]) - 0.0).abs() < 1e-6);
        assert!((level_dbfs(&[0.1; 16]) + 20.0).abs() < 1e-4);
        let mut lock = ShoutLock::new(-20.0, Duration::from_secs(1));
        lock.process(&buffer(0.5, 10), SAMPLE_RATE);
        lock.process(&buffer(0.1, 10), SAMPLE_RATE);
        assert!((lock.last_level_dbfs() + 20.0).abs() < 1e-4);
        assert!((lock.peak_level_dbfs() + 6.0206).abs() < 1e-3);
    }
}
