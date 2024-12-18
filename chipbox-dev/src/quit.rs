use std::time::Duration;
use watchexec::action::ActionHandler;
use watchexec_signals::Signal;

fn signal_terminates(signal: Signal) -> bool {
    signal == Signal::Interrupt || signal == Signal::Terminate || signal == Signal::Quit
}

pub(super) fn handle(action: &mut ActionHandler) {
    if action.signals().any(signal_terminates) {
        tracing::info!("quitting on signal");
        action.quit_gracefully(Signal::Terminate, Duration::from_secs(1));
    }
}
