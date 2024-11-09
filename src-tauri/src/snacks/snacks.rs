use tauri::{AppHandle, Emitter};
use crate::structs::snack::{Snack, SnackHorizontal, SnackSeverity, SnackVertical};

pub fn send_snacks(message: String, snack_severity: SnackSeverity, snack_vertical: SnackVertical, snack_horizontal: SnackHorizontal, app: &AppHandle) {
    let snack = Snack {
        severity: snack_severity.to_string().to_lowercase(),
        message,
        open: true,
        vertical: snack_vertical.to_string().to_lowercase(),
        horizontal: snack_horizontal.to_string().to_lowercase(),
    };
    app.emit("show_snack", snack)
        .expect("Could not emit snack");
}
