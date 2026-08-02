use crate::errors::SidecarError;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CircuitState {
    Closed = 0,
    Open = 1,
    HalfOpen = 2,
}

/// Zero-lock, wait-free Circuit Breaker packed into a single AtomicU64.
/// Bits 0-7: State | Bits 8-39: Failure Count | Bits 40-63: Timeout Deadline (Secs)
pub struct CircuitBreaker {
    packed: AtomicU64,
    failure_threshold: u32,
    recovery_timeout_secs: u64,
    boot_time: Instant,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout_secs: u64) -> Self {
        Self {
            packed: AtomicU64::new(0),
            failure_threshold,
            recovery_timeout_secs,
            boot_time: Instant::now(),
        }
    }

    #[inline]
    fn pack(state: CircuitState, failures: u32, deadline_secs: u64) -> u64 {
        ((deadline_secs & 0xFFFFFF) << 40) | ((failures as u64 & 0xFFFFFFFF) << 8) | (state as u64)
    }

    #[inline]
    fn unpack(val: u64) -> (CircuitState, u32, u64) {
        let state = match val & 0xFF {
            0 => CircuitState::Closed,
            1 => CircuitState::Open,
            2 => CircuitState::HalfOpen,
            _ => CircuitState::Closed,
        };
        let failures = ((val >> 8) & 0xFFFFFFFF) as u32;
        let deadline = (val >> 40) & 0xFFFFFF;
        (state, failures, deadline)
    }

    #[inline]
    fn current_secs(&self) -> u64 {
        self.boot_time.elapsed().as_secs()
    }

    pub fn allow_request(&self) -> Result<(), SidecarError> {
        let now = self.current_secs();
        let mut current = self.packed.load(Ordering::Acquire);

        loop {
            let (state, failures, deadline) = Self::unpack(current);

            match state {
                CircuitState::Closed => return Ok(()),
                CircuitState::Open => {
                    if now >= deadline {
                        let new_packed = Self::pack(CircuitState::HalfOpen, failures, deadline);
                        match self.packed.compare_exchange_weak(
                            current,
                            new_packed,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        ) {
                            Ok(_) => return Ok(()),
                            Err(updated) => current = updated,
                        }
                    } else {
                        return Err(SidecarError::CircuitOpen);
                    }
                }
                CircuitState::HalfOpen => return Ok(()),
            }
        }
    }

    pub fn record_success(&self) {
        self.packed
            .store(Self::pack(CircuitState::Closed, 0, 0), Ordering::Release);
    }

    pub fn record_failure(&self) {
        let now = self.current_secs();
        let mut current = self.packed.load(Ordering::Acquire);

        loop {
            let (state, failures, _) = Self::unpack(current);
            let new_failures = failures.saturating_add(1);
            let new_state = if new_failures >= self.failure_threshold {
                CircuitState::Open
            } else {
                state
            };
            let deadline = if new_state == CircuitState::Open {
                now + self.recovery_timeout_secs
            } else {
                0
            };

            let new_packed = Self::pack(new_state, new_failures, deadline);
            match self.packed.compare_exchange_weak(
                current,
                new_packed,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(updated) => current = updated,
            }
        }
    }

    pub async fn call<F, Fut, T>(&self, f: F) -> Result<T, SidecarError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, SidecarError>>,
    {
        self.allow_request()?;
        match f().await {
            Ok(res) => {
                self.record_success();
                Ok(res)
            }
            Err(e) if e.is_retriable() => {
                self.record_failure();
                Err(e)
            }
            Err(e) => Err(e),
        }
    }
}
