use Cap;

use pal::thread::spawn as pspawn;

pub use pal::thread::JoinHandle;
pub use pal::thread::Thread;

pub fn spawn<F, T>(_: &Cap, f: F) -> JoinHandle<T>
    where F: FnOnce(Cap) -> T, F: Send + 'static, T: Send + 'static
{
    pspawn(move || {
        f(Cap(()))
    })
}

