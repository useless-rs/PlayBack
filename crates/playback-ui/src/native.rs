//! Optional native-window integration helpers.
//!
//! The default build keeps this boundary platform-neutral. Native frontends
//! can enable the `native` feature and pass a `HasWindowHandle` implementation
//! to [`native_window_handle`] without exposing raw pointers in PlayBack's
//! public model.

#![cfg(feature = "native")]

use raw_window_handle::{HasWindowHandle, RawWindowHandle};

/// Returns the raw handle supplied by a platform window.
///
/// # Errors
///
/// Returns `None` when the platform window does not currently expose a valid
/// native handle.
pub fn native_window_handle(window: &impl HasWindowHandle) -> Option<RawWindowHandle> {
    window.window_handle().ok().map(|handle| handle.as_raw())
}

#[cfg(target_os = "macos")]
/// Returns the macOS HIG design-system rules used by a native renderer.
pub fn apple_hig_design_system() -> gpui_design::DesignSystem {
    gpui_design::DesignSystem::apple_hig()
}

#[cfg(target_os = "macos")]
/// Returns the Lucide icon used for the initial play action.
pub fn default_player_icon() -> lucide_icons::Icon {
    lucide_icons::Icon::Play
}

#[cfg(target_os = "macos")]
/// Confirms that the GPUI UI-kit button type is linked into a native build.
pub fn ui_kit_button_type_name() -> &'static str {
    std::any::type_name::<gpui_ui_kit::Button>()
}

#[cfg(test)]
mod tests {
    use super::native_window_handle;

    struct NoHandle;

    impl raw_window_handle::HasWindowHandle for NoHandle {
        fn window_handle(
            &self,
        ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
            Err(raw_window_handle::HandleError::NotSupported)
        }
    }

    #[test]
    fn unsupported_window_has_no_handle() {
        assert!(native_window_handle(&NoHandle).is_none());
    }
}
