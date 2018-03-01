//! This module contains the implementation of a service to
//! send a messages when timeout elapsed.

use std::time::Duration;
use stdweb::Value;
use html::Callback;
use super::{Task, to_ms};

/// A handle to cancel a timeout task.
pub struct TimeoutHandle(Option<Value>);

/// An service to set a timeout.
pub struct TimeoutService {
}

impl TimeoutService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self { }
    }

    /// Sets timeout which send a messages from a `converter` after `duration`.
    pub fn spawn(&mut self, duration: Duration, callback: Callback<()>) -> TimeoutHandle {
        let callback = move || {
            callback.emit(());
        };
        let ms = to_ms(duration);
        let handle = js! {
            var callback = @{callback};
            var action = function() {
                callback();
                callback.drop();
            };
            var delay = @{ms};
            return {
                timeout_id: setTimeout(action, delay),
                callback: callback,
            };
        };
        TimeoutHandle(Some(handle))
    }
}

impl Task for TimeoutHandle {
    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to cancel timeout twice");
        js! { @(no_return)
            var handle = @{handle};
            clearTimeout(handle.timeout_id);
            handle.callback.drop();
        }
    }
}
