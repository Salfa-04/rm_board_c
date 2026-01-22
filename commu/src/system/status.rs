//!
//! # System Status
//!

use super::private::*;

static STATUS: AtomicI8 = AtomicI8::new(SysMode::Boot as _);

///
/// # System Mode Enumeration
///
/// ## Get Current Mode
/// ```rust
/// let mode: SysMode = SysMode::get();
/// ```
///
/// ## Set Current Mode
/// ```rust
/// SysMode::Normal.set();
/// SysMode::set(SysMode::Normal);
/// ```
///
#[repr(i8)]
#[non_exhaustive]
#[derive(FromRepr, PartialEq, defmt::Format, Debug)]
pub enum SysMode {
    Error = -1,
    Boot = 0,
    Normal = 1,
}

impl SysMode {
    ///
    /// # Get System Mode
    ///
    /// Retrieve the current system mode.
    ///
    #[inline]
    pub fn get() -> SysMode {
        match SysMode::from_repr(STATUS.load(Order)) {
            Some(x) => x,
            None => Self::Error,
        }
    }

    ///
    /// # Set System Mode
    ///
    /// Set the current system mode to the specified value.
    ///
    #[inline]
    pub fn set(self) {
        STATUS.store(self as _, Order);
    }

    ///
    /// # Wait for System Mode
    ///
    /// Wait until the system mode matches the specified mode.
    ///
    pub fn wait(&self, t: &mut Ticker) -> impl Future<Output = ()> {
        async {
            while Self::get() != *self {
                t.next().await
            }
        }
    }
}
