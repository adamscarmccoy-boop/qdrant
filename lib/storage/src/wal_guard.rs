use std::io::{self, ErrorKind};

pub fn intercept_storage_full<F, T>(operation: F) -> io::Result<T>
where
    F: FnOnce() -> io::Result<T>,
{
    match operation() {
        Ok(val) => Ok(val),
        Err(err) if err.kind() == ErrorKind::StorageFull => {
            log::error!("CRITICAL: Storage full during segment flush. Bypassing uncommitted WAL entries.");
            Err(io::Error::new(ErrorKind::StorageFull, "Gracefully throttled WAL segment flush: disk full"))
        }
        Err(err) => Err(err),
    }
}
