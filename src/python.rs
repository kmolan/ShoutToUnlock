//! the python module `shout_to_unlock`. build it with `maturin`.

use std::time::Duration;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::ShoutLock;

fn duration_from_seconds(seconds: f64) -> PyResult<Duration> {
    Duration::try_from_secs_f64(seconds).map_err(|error| {
        PyValueError::new_err(format!(
            "invalid duration {seconds}: {error} (time only goes forward bestie)"
        ))
    })
}

/// a lock that opens when u are loud enough for long enough.
#[pyclass(name = "ShoutLock", module = "shout_to_unlock")]
struct PythonShoutLock(ShoutLock);

#[pymethods]
impl PythonShoutLock {
    /// raises `ValueError` if `minimum_duration_seconds` is negative, not finite, or absurdly large. no time travel
    #[new]
    fn new(minimum_level_dbfs: f32, minimum_duration_seconds: f64) -> PyResult<Self> {
        let minimum_duration = duration_from_seconds(minimum_duration_seconds)?;
        Ok(Self(ShoutLock::new(minimum_level_dbfs, minimum_duration)))
    }

    /// feed it one buffer of mono samples in [-1, 1]. returns `is_unlocked`.
    fn process(&mut self, samples: Vec<f32>, sample_rate: u32) -> bool {
        self.0.process(&samples, sample_rate)
    }

    /// measured the level urself? overachiever. returns `is_unlocked`.
    fn process_level(&mut self, level_dbfs: f32, duration_seconds: f64) -> PyResult<bool> {
        Ok(self
            .0
            .process_level(level_dbfs, duration_from_seconds(duration_seconds)?))
    }

    #[getter]
    fn is_unlocked(&self) -> bool {
        self.0.is_unlocked()
    }

    fn __bool__(&self) -> bool {
        self.0.is_unlocked()
    }

    /// how long u've been loud enough, in total. ur scream résumé.
    #[getter]
    fn loud_seconds(&self) -> f64 {
        self.0.loud_duration().as_secs_f64()
    }

    /// how much more screaming until freedom.
    #[getter]
    fn remaining_seconds(&self) -> f64 {
        self.0.remaining_duration().as_secs_f64()
    }

    /// `loud_seconds / minimum_seconds`, clamped to [0, 1]. a progress bar for ur lungs.
    #[getter]
    fn progress(&self) -> f64 {
        self.0.progress()
    }

    /// total time of everything u fed it, loud or sad.
    #[getter]
    fn total_seconds(&self) -> f64 {
        self.0.total_duration().as_secs_f64()
    }

    /// `-inf` before the first buffer. the calm before the scream.
    #[getter]
    fn last_level_dbfs(&self) -> f32 {
        self.0.last_level_dbfs()
    }

    /// `-inf` before the first buffer. the calm before the scream.
    #[getter]
    fn peak_level_dbfs(&self) -> f32 {
        self.0.peak_level_dbfs()
    }

    #[getter]
    fn minimum_level_dbfs(&self) -> f32 {
        self.0.minimum_level_dbfs()
    }

    #[getter]
    fn minimum_seconds(&self) -> f64 {
        self.0.minimum_duration().as_secs_f64()
    }

    /// slams the lock shut and forgets everything. the limits stay. fresh start, same standards.
    fn reset(&mut self) {
        self.0.reset();
    }

    fn __repr__(&self) -> String {
        format!(
            "ShoutLock(unlocked={}, loud_seconds={:.3}/{:.3}, last_level_dbfs={:.1})",
            if self.0.is_unlocked() {
                "True"
            } else {
                "False"
            },
            self.0.loud_duration().as_secs_f64(),
            self.0.minimum_duration().as_secs_f64(),
            self.0.last_level_dbfs(),
        )
    }
}

/// RMS level in dBFS. silence gives `-inf`. very dramatic of it.
#[pyfunction]
fn level_dbfs(samples: Vec<f32>) -> f32 {
    crate::level_dbfs(&samples)
}

#[pymodule]
fn shout_to_unlock(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonShoutLock>()?;
    module.add_function(wrap_pyfunction!(level_dbfs, module)?)?;
    Ok(())
}
