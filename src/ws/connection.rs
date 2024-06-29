use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
    layer::SocketIoLayer,
    socket::DisconnectReason,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

use log::{debug, error, info, trace, warn};

struct AppState {
    listener_count: Arc<Mutex<i32>>,
}

pub fn return_layer() -> SocketIoLayer {
    let listener_count = Arc::new(Mutex::new(0));
    let state = AppState {
        listener_count: listener_count.clone(),
    };

    let (layer, io) = SocketIo::new_layer();


    io.ns("/ws/connection", move |socket: SocketRef| {

        // Handler for getting the amount of listeners... very hacky.

        let listener_count_clone = state.listener_count.clone(); // Clone the Arc for the closure
        let mut listener_count = listener_count_clone.lock().unwrap();
        *listener_count += 1;
        debug!("Listener count: {}", *listener_count);
        let emit = socket.broadcast().emit("listener_count", Value::Number((*listener_count).into()));
        let local_emit = socket.emit("listener_count", Value::Number((*listener_count).into()));
        match emit {
            Ok(_) => debug!("Emitted listener count: {}", *listener_count),
            Err(e) => error!("Failed to emit listener count: {}", e),
        }
        match local_emit {
            Ok(_) => debug!("Emitted listener count locally: {}", *listener_count),
            Err(e) => error!("Failed to emit listener count locally: {}", e),
        }

        let listener_count_clone = state.listener_count.clone(); // Clone the Arc again for the on_disconnect closure
        socket.on_disconnect(move |s: SocketRef| async move {
            let mut listener_count = listener_count_clone.lock().unwrap();
            *listener_count -= 1;
            s.broadcast().emit("listener_count", Value::Number((*listener_count).into())).unwrap();
            debug!("Listener count: {}", *listener_count);
        });

        // 
        
    });

    layer
}
