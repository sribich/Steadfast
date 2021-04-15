use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG: AtomicBool = AtomicBool::new(false);
