//! the javascript module, so ur browser can hear u scream. build it with `wasm-pack build --features wasm`.

use std::time::Duration;

use wasm_bindgen::prelude::*;

use crate::ShoutLock;

fn duration_from_seconds(seconds: f64) -> Result<Duration, JsError> {
    Duration::try_from_secs_f64(seconds).map_err(|error| {
        JsError::new(&format!(
            "invalid duration {seconds}: {error} (time only goes forward bestie)"
        ))
    })
}

/// a lock that opens when u are loud enough for long enough.
#[wasm_bindgen(js_name = ShoutLock)]
pub struct WasmShoutLock(ShoutLock);

#[wasm_bindgen(js_class = ShoutLock)]
impl WasmShoutLock {
    /// throws if `minimumDurationSeconds` is negative, not finite, or absurdly large. no time travel
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = minimumLevelDbfs)] minimum_level_dbfs: f32,
        #[wasm_bindgen(js_name = minimumDurationSeconds)] minimum_duration_seconds: f64,
    ) -> Result<WasmShoutLock, JsError> {
        let minimum_duration = duration_from_seconds(minimum_duration_seconds)?;
        Ok(Self(ShoutLock::new(minimum_level_dbfs, minimum_duration)))
    }

    /// feed it one buffer of mono samples in [-1, 1]. returns `isUnlocked`.
    pub fn process(
        &mut self,
        samples: &[f32],
        #[wasm_bindgen(js_name = sampleRate)] sample_rate: u32,
    ) -> bool {
        self.0.process(samples, sample_rate)
    }

    /// measured the level urself? overachiever. returns `isUnlocked`.
    #[wasm_bindgen(js_name = processLevel)]
    pub fn process_level(
        &mut self,
        #[wasm_bindgen(js_name = levelDbfs)] level_dbfs: f32,
        #[wasm_bindgen(js_name = durationSeconds)] duration_seconds: f64,
    ) -> Result<bool, JsError> {
        Ok(self
            .0
            .process_level(level_dbfs, duration_from_seconds(duration_seconds)?))
    }

    #[wasm_bindgen(getter = isUnlocked)]
    pub fn is_unlocked(&self) -> bool {
        self.0.is_unlocked()
    }

    /// how long u've been loud enough, in total. ur scream résumé.
    #[wasm_bindgen(getter = loudSeconds)]
    pub fn loud_seconds(&self) -> f64 {
        self.0.loud_duration().as_secs_f64()
    }

    /// how much more screaming until freedom.
    #[wasm_bindgen(getter = remainingSeconds)]
    pub fn remaining_seconds(&self) -> f64 {
        self.0.remaining_duration().as_secs_f64()
    }

    /// `loudSeconds / minimumSeconds`, clamped to [0, 1]. a progress bar for ur lungs.
    #[wasm_bindgen(getter)]
    pub fn progress(&self) -> f64 {
        self.0.progress()
    }

    /// total time of everything u fed it, loud or sad.
    #[wasm_bindgen(getter = totalSeconds)]
    pub fn total_seconds(&self) -> f64 {
        self.0.total_duration().as_secs_f64()
    }

    /// `-Infinity` before the first buffer. the calm before the scream.
    #[wasm_bindgen(getter = lastLevelDbfs)]
    pub fn last_level_dbfs(&self) -> f32 {
        self.0.last_level_dbfs()
    }

    /// `-Infinity` before the first buffer. the calm before the scream.
    #[wasm_bindgen(getter = peakLevelDbfs)]
    pub fn peak_level_dbfs(&self) -> f32 {
        self.0.peak_level_dbfs()
    }

    #[wasm_bindgen(getter = minimumLevelDbfs)]
    pub fn minimum_level_dbfs(&self) -> f32 {
        self.0.minimum_level_dbfs()
    }

    #[wasm_bindgen(getter = minimumSeconds)]
    pub fn minimum_seconds(&self) -> f64 {
        self.0.minimum_duration().as_secs_f64()
    }

    /// slams the lock shut and forgets everything. the limits stay. fresh start, same standards.
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

/// RMS level in dBFS. silence gives `-Infinity`. very dramatic of it.
#[wasm_bindgen(js_name = levelDbfs)]
pub fn level_dbfs(samples: &[f32]) -> f32 {
    crate::level_dbfs(samples)
}
