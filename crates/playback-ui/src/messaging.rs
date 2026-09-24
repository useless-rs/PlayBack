//! Bounded messages exchanged between the UI and media threads.

use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError, bounded};
use playback_config::PlaybackConfig;
use playback_core::PlaybackSnapshot;
use thiserror::Error;

use crate::state::UiCommand;

/// Errors returned by the UI message channel.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum UiChannelError {
    /// The receiving side has been dropped.
    #[error("UI channel is disconnected")]
    Disconnected,
    /// A non-blocking send found a full channel.
    #[error("UI channel is full")]
    Full,
    /// A non-blocking receive found no message.
    #[error("UI channel is empty")]
    Empty,
}

/// A message delivered to the UI event loop.
#[derive(Debug, Clone, PartialEq)]
pub enum UiMessage {
    /// Apply a user interaction.
    Command(UiCommand),
    /// Replace the rendered playback snapshot.
    Playback(PlaybackSnapshot),
    /// Replace the active configuration.
    Config(Box<PlaybackConfig>),
    /// Stop the UI event loop.
    Shutdown,
}

/// The sending half of a bounded UI channel.
#[derive(Debug)]
pub struct UiSender {
    inner: Sender<UiMessage>,
}

/// The receiving half of a bounded UI channel.
#[derive(Debug)]
pub struct UiReceiver {
    inner: Receiver<UiMessage>,
}

/// Creates a bounded UI message channel.
pub fn ui_channel(capacity: usize) -> (UiSender, UiReceiver) {
    let (sender, receiver) = bounded(capacity);
    (UiSender { inner: sender }, UiReceiver { inner: receiver })
}

impl UiSender {
    /// Sends a message, waiting for channel capacity when necessary.
    pub fn send(&self, message: UiMessage) -> Result<(), UiChannelError> {
        self.inner
            .send(message)
            .map_err(|_| UiChannelError::Disconnected)
    }

    /// Attempts to send without blocking.
    pub fn try_send(&self, message: UiMessage) -> Result<(), UiChannelError> {
        match self.inner.try_send(message) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(_)) => Err(UiChannelError::Full),
            Err(TrySendError::Disconnected(_)) => Err(UiChannelError::Disconnected),
        }
    }
}

impl UiReceiver {
    /// Receives the next message, blocking the calling thread until one arrives.
    pub fn recv(&self) -> Result<UiMessage, UiChannelError> {
        self.inner.recv().map_err(|_| UiChannelError::Disconnected)
    }

    /// Receives the next message without blocking.
    pub fn try_recv(&self) -> Result<UiMessage, UiChannelError> {
        match self.inner.try_recv() {
            Ok(message) => Ok(message),
            Err(TryRecvError::Empty) => Err(UiChannelError::Empty),
            Err(TryRecvError::Disconnected) => Err(UiChannelError::Disconnected),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UiMessage, ui_channel};
    use crate::state::UiCommand;

    #[test]
    fn bounded_channel_delivers_commands_in_order() {
        let (sender, receiver) = ui_channel(2);
        sender
            .send(UiMessage::Command(UiCommand::TogglePlay))
            .expect("send");
        sender.send(UiMessage::Shutdown).expect("send");
        assert_eq!(
            receiver.recv().expect("receive"),
            UiMessage::Command(UiCommand::TogglePlay)
        );
        assert_eq!(receiver.recv().expect("receive"), UiMessage::Shutdown);
    }

    #[test]
    fn try_send_reports_full_channel() {
        let (sender, _receiver) = ui_channel(1);
        sender.try_send(UiMessage::Shutdown).expect("first send");
        assert!(sender.try_send(UiMessage::Shutdown).is_err());
    }
}
