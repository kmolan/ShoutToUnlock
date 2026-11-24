//! the C ABI, for the classics. see `include/shout_to_unlock.h`.
//!
//! every function that takes a `ShoutLock` pointer needs a real one from `shout_to_unlock_new`. garbage pointers = UB = sad.

use std::time::Duration;

use crate::ShoutLock;

/// returns null if `minimum_duration_seconds` is negative, not finite, or absurdly large. no time travel
#[unsafe(no_mangle)]
pub extern "C" fn shout_to_unlock_new(
    minimum_level_dbfs: f32,
    minimum_duration_seconds: f64,
) -> *mut ShoutLock {
    match Duration::try_from_secs_f64(minimum_duration_seconds) {
        Ok(minimum_duration) => Box::into_raw(Box::new(ShoutLock::new(
            minimum_level_dbfs,
            minimum_duration,
        ))),
        Err(_) => std::ptr::null_mut(),
    }
}

/// null does nothing. freeing nothing is free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_free(lock: *mut ShoutLock) {
    if !lock.is_null() {
        drop(unsafe { Box::from_raw(lock) });
    }
}

/// `samples` can be null only if `sample_count` is 0. no samples, no scream.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_process(
    lock: *mut ShoutLock,
    samples: *const f32,
    sample_count: usize,
    sample_rate: u32,
) -> bool {
    let lock = unsafe { &mut *lock };
    if samples.is_null() || sample_count == 0 {
        return lock.is_unlocked();
    }
    let samples = unsafe { std::slice::from_raw_parts(samples, sample_count) };
    lock.process(samples, sample_rate)
}

/// a negative, not finite, or absurdly large `duration_seconds` gets ignored. like ur texts.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_process_level(
    lock: *mut ShoutLock,
    level_dbfs: f32,
    duration_seconds: f64,
) -> bool {
    let lock = unsafe { &mut *lock };
    match Duration::try_from_secs_f64(duration_seconds) {
        Ok(duration) => lock.process_level(level_dbfs, duration),
        Err(_) => lock.is_unlocked(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_is_unlocked(lock: *const ShoutLock) -> bool {
    unsafe { &*lock }.is_unlocked()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_loud_seconds(lock: *const ShoutLock) -> f64 {
    unsafe { &*lock }.loud_duration().as_secs_f64()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_remaining_seconds(lock: *const ShoutLock) -> f64 {
    unsafe { &*lock }.remaining_duration().as_secs_f64()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_progress(lock: *const ShoutLock) -> f64 {
    unsafe { &*lock }.progress()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_total_seconds(lock: *const ShoutLock) -> f64 {
    unsafe { &*lock }.total_duration().as_secs_f64()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_last_level_dbfs(lock: *const ShoutLock) -> f32 {
    unsafe { &*lock }.last_level_dbfs()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_peak_level_dbfs(lock: *const ShoutLock) -> f32 {
    unsafe { &*lock }.peak_level_dbfs()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_minimum_level_dbfs(lock: *const ShoutLock) -> f32 {
    unsafe { &*lock }.minimum_level_dbfs()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_minimum_seconds(lock: *const ShoutLock) -> f64 {
    unsafe { &*lock }.minimum_duration().as_secs_f64()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_reset(lock: *mut ShoutLock) {
    unsafe { &mut *lock }.reset();
}

/// `samples` can be null only if `sample_count` is 0. no samples, no scream.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shout_to_unlock_level_dbfs(
    samples: *const f32,
    sample_count: usize,
) -> f32 {
    if samples.is_null() || sample_count == 0 {
        return f32::NEG_INFINITY;
    }
    crate::level_dbfs(unsafe { std::slice::from_raw_parts(samples, sample_count) })
}
