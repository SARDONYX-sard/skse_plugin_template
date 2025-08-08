use commonlibsse_ng::skse;
use commonlibsse_ng::skse::interfaces::messaging::{Message, MessageType};

/// Early return error & log
macro_rules! bail {
    ($expr:expr) => {
        if let Err(err) = $expr {
            #[cfg(feature = "tracing")]
            tracing::error!("{err}");
            return;
        };
    };
}

// #[commonlibsse_ng::skse_plugin_main(log_level = "info")] // You can this.
//
// You can this too.(However, changing this alone will not change the DLL name.
// The only thing that can be changed is the information read by SKSE.)
// #[commonlibsse_ng::skse_plugin_main(plugin_name= "my_rust_plugin")]

#[commonlibsse_ng::skse_plugin_main] // <- simple pattern
fn plugin_main() {
    let messaging = match skse::api::get_messaging_interface() {
        Ok(messaging) => messaging,
        Err(_err) => {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed to skse::init: {_err}");
            return;
        }
    };

    bail!(messaging.register_skse_listener(skse_event_listener));
}

fn skse_event_listener(message: &Message) {
    if let Some(msg_type) = message.msg_type.to_enum()
        && msg_type == MessageType::PostLoadGame
    {
        commonlibsse_ng::debug_message_box!("Hello world from Rust SKSE plugin!");

        tracing::info!("Logging test!");

        unsafe {
            use commonlibsse_ng::re::ConsoleLog::ConsoleLog;
            ConsoleLog::get_singleton_mut().map(|c| c.print(c"Hello World! from Rust".as_ptr()))
        };
    }
}

// This is because CI will treat it as an error if there is at least one test missing.
#[test]
fn dummy_test() {
    assert_eq!(1 + 1, 2);
}
