use St;

use bpal::thread::spawn as pspawn;

pub use bpal::thread::JoinHandle;
pub use bpal::thread::Thread;

pub fn spawn<F, T>(_: &St, f: F) -> JoinHandle<T>
    where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
    pspawn(move || {
        f()
    })
}

