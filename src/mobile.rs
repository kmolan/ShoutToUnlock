//! kotlin + swift bindings via UniFFI, so ur phone can get yelled at too. see `mobile/`.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

#[derive(Debug, uniffi::Error)]
pub enum ShoutLockError {
    /// the duration is negative, not finite, or absurdly large. no time travel
    InvalidDuration { seconds: f64 },
}

impl std::fmt::Display for ShoutLockError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDuration { seconds } => write!(
                formatter,
                "invalid duration {seconds} (time only goes forward bestie)"
            ),
        }
    }
}

impl std::error::Error for ShoutLockError {}

fn duration_from_seconds(seconds: f64) -> Result<Duration, ShoutLockError> {
    Duration::try_from_secs_f64(seconds).map_err(|_| ShoutLockError::InvalidDuration { seconds })
}

/// a lock that opens when u are loud enough for long enough.
#[derive(uniffi::Object)]
pub struct ShoutLock(Mutex<crate::ShoutLock>);

impl ShoutLock {
    fn lock(&self) -> MutexGuard<'_, crate::ShoutLock> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[uniffi::export]
impl ShoutLock {
    /// throws `InvalidDuration` if `minimum_duration_seconds` is negative, not finite, or absurdly large. no time travel
    #[uniffi::constructor]
    pub fn new(
        minimum_level_dbfs: f32,
        minimum_duration_seconds: f64,
    ) -> Result<Arc<Self>, ShoutLockError> {
        let minimum_duration = duration_from_seconds(minimum_duration_seconds)?;
        Ok(Arc::new(Self(Mutex::new(crate::ShoutLock::new(
            minimum_level_dbfs,
            minimum_duration,
        )))))
    }

    /// feed it one buffer of mono samples in [-1, 1]. returns `is_unlocked`.
    pub fn process(&self, samples: Vec<f32>, sample_rate: u32) -> bool {
        self.lock().process(&samples, sample_rate)
    }

    /// measured the level urself? overachiever. returns `is_unlocked`.
    pub fn process_level(
        &self,
        level_dbfs: f32,
        duration_seconds: f64,
    ) -> Result<bool, ShoutLockError> {
        let duration = duration_from_seconds(duration_seconds)?;
        Ok(self.lock().process_level(level_dbfs, duration))
    }

    pub fn is_unlocked(&self) -> bool {
        self.lock().is_unlocked()
    }

    /// how long u've been loud enough, in total. ur scream résumé.
    pub fn loud_seconds(&self) -> f64 {
        self.lock().loud_duration().as_secs_f64()
    }

    /// how much more screaming until freedom.
    pub fn remaining_seconds(&self) -> f64 {
        self.lock().remaining_duration().as_secs_f64()
    }

    /// `loud_seconds / minimum_seconds`, clamped to [0, 1]. a progress bar for ur lungs.
    pub fn progress(&self) -> f64 {
        self.lock().progress()
    }

    /// total time of everything u fed it, loud or sad.
    pub fn total_seconds(&self) -> f64 {
        self.lock().total_duration().as_secs_f64()
    }

    /// negative infinity before the first buffer. the calm before the scream.
    pub fn last_level_dbfs(&self) -> f32 {
        self.lock().last_level_dbfs()
    }

    /// negative infinity before the first buffer. the calm before the scream.
    pub fn peak_level_dbfs(&self) -> f32 {
        self.lock().peak_level_dbfs()
    }

    pub fn minimum_level_dbfs(&self) -> f32 {
        self.lock().minimum_level_dbfs()
    }

    pub fn minimum_seconds(&self) -> f64 {
        self.lock().minimum_duration().as_secs_f64()
    }

    /// slams the lock shut and forgets everything. the limits stay. fresh start, same standards.
    pub fn reset(&self) {
        self.lock().reset();
    }
}

/// RMS level in dBFS. silence gives negative infinity. very dramatic of it.
#[uniffi::export]
pub fn level_dbfs(samples: Vec<f32>) -> f32 {
    crate::level_dbfs(&samples)
}
