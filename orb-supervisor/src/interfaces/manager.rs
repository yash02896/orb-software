//! [`Manager`] defines the `org.worldcoin.OrbSupervisor1.Manager` Dbus interface.
//!
//! It currently only supports the `BackgroundDownloadsAllowed` property used by the update to
//! decide whether or not it can download updates.

use tokio::time::{
    Duration,
    Instant,
};
use zbus::{
    dbus_interface,
    SignalContext,
};

/// The duration of time since the last "start signup" event that has to have passed
/// before the update agent is permitted to start a download.
pub const DEFAULT_DURATION_TO_ALLOW_DOWNLOADS: Duration = Duration::from_secs(3600);

pub const BACKGROUND_DOWNLOADS_ALLOWED_PROPERTY_NAME: &str = "BackgroundDownloadsAllowed";
pub const INTERFACE_NAME: &str = "org.worldcoin.OrbSupervisor1.Manager";
pub const OBJECT_PATH: &str = "/org/worldcoin/OrbSupervisor1/Manager";

pub struct Manager {
    last_signup_event: Instant,
    duration_to_allow_downloads: Duration,
}

impl Manager {
    /// Constructs a new `Manager` instance.
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self::with_last_signup_event(Instant::now())
    }

    /// Constructs a new `Manager` instance with `last_signup_event` taken as the instant at
    /// which the last signup event happened.
    ///
    /// This is primarily useful for integration tests.
    ///
    /// **NOTE** do not rely on this function as it will be removed once tokio's time API is
    /// correctly respected inside zbus.
    #[must_use]
    pub fn with_last_signup_event(last_signup_event: Instant) -> Self {
        Self {
            last_signup_event,
            duration_to_allow_downloads: DEFAULT_DURATION_TO_ALLOW_DOWNLOADS,
        }
    }

    #[must_use]
    pub fn duration_to_allow_downloads(self, duration_to_allow_downloads: Duration) -> Self {
        Self {
            duration_to_allow_downloads,
            ..self
        }
    }

    #[allow(clippy::must_use_candidate)]
    pub fn are_downloads_allowed(&self) -> bool {
        self.last_signup_event.elapsed() >= self.duration_to_allow_downloads
    }

    fn reset_last_signup_event(&mut self) {
        self.last_signup_event = Instant::now();
    }

    /// Resets the internal timer tracking the last signup event to the current time and emits a
    /// `PropertyChanged` for the `BackgroundDownloadsAllowed` signal.
    ///
    /// # Errors
    ///
    /// The same as calling [`zbus::fdo::Properties::properties_changed`].
    pub async fn reset_last_signup_event_and_notify(
        &mut self,
        signal_context: &SignalContext<'_>,
    ) -> zbus::Result<()> {
        self.reset_last_signup_event();
        self.background_downloads_allowed_changed(signal_context)
            .await
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

#[dbus_interface(name = "org.worldcoin.OrbSupervisor1.Manager")]
impl Manager {
    #[dbus_interface(property, name = "BackgroundDownloadsAllowed")]
    async fn background_downloads_allowed(&self) -> bool {
        self.are_downloads_allowed()
    }
}

#[cfg(test)]
mod tests {
    use zbus::Interface;

    use super::{
        Manager,
        DEFAULT_DURATION_TO_ALLOW_DOWNLOADS,
    };

    #[test]
    fn manager_interface_name_matches_exported_const() {
        assert_eq!(super::INTERFACE_NAME, &*Manager::name());
    }

    #[tokio::test]
    async fn manager_background_downloads_allowed_property_matched_exported_const() {
        let manager = Manager::new();
        assert!(
            manager
                .get(super::BACKGROUND_DOWNLOADS_ALLOWED_PROPERTY_NAME)
                .await
                .is_some()
        );
    }

    #[tokio::test(start_paused = true)]
    async fn downloads_are_disallowed_if_last_signup_event_is_too_recent() {
        let manager =
            Manager::new().duration_to_allow_downloads(DEFAULT_DURATION_TO_ALLOW_DOWNLOADS);
        tokio::time::advance(DEFAULT_DURATION_TO_ALLOW_DOWNLOADS / 2).await;

        assert!(!manager.are_downloads_allowed());
    }

    #[tokio::test(start_paused = true)]
    async fn downloads_are_allowed_if_last_signup_event_is_old_enough() {
        let manager =
            Manager::new().duration_to_allow_downloads(DEFAULT_DURATION_TO_ALLOW_DOWNLOADS);
        tokio::time::advance(DEFAULT_DURATION_TO_ALLOW_DOWNLOADS * 2).await;
        assert!(manager.are_downloads_allowed());
    }

    #[tokio::test(start_paused = true)]
    async fn downloads_become_disallowed_after_reset() {
        let mut manager =
            Manager::new().duration_to_allow_downloads(DEFAULT_DURATION_TO_ALLOW_DOWNLOADS);
        tokio::time::advance(DEFAULT_DURATION_TO_ALLOW_DOWNLOADS * 2).await;
        assert!(manager.are_downloads_allowed());
        manager.reset_last_signup_event();
        assert!(!manager.are_downloads_allowed());
    }
}
