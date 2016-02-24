use libc;
use ralloc::boxed::{Box, FnBox};
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use core::cmp;

pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
    let stack_size = 2 * 1024 * 1024;

    let main = move || {
        f();
    };

    let t = unsafe { Thread::new(stack_size, Box::new(main)).unwrap() };

    JoinHandle {
        _d: PhantomData,
        _thread: t
    }
}

pub struct JoinHandle<T> {
    _d: PhantomData<T>,
    _thread: Thread,
}

impl<T> JoinHandle<T> {
    pub fn thread(&self) -> &Thread {
        unimplemented!()
    } 

    pub fn join(self) -> Result<T, ()> {
        unimplemented!()
    }
}

pub struct Thread {
    id: libc::pthread_t,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>)
                          -> Result<Thread, ()> {
        let p = Box::new(p);
        let mut native: libc::pthread_t = mem::zeroed();
        let mut attr: libc::pthread_attr_t = mem::zeroed();
        assert_eq!(libc::pthread_attr_init(&mut attr), 0);

        let stack_size = cmp::max(stack, /*min_stack_size(&attr)*/ 0);
        match libc::pthread_attr_setstacksize(&mut attr,
                                              stack_size as libc::size_t) {
            0 => {}
            n => {
                assert_eq!(n, libc::EINVAL);
                // EINVAL means |stack_size| is either too small or not a
                // multiple of the system page size.  Because it's definitely
                // >= PTHREAD_STACK_MIN, it must be an alignment issue.
                // Round up to the nearest page and try again.
                /*let page_size = os::page_size();
                let stack_size = (stack_size + page_size - 1) &
                                 (-(page_size as isize - 1) as usize - 1);
                let stack_size = stack_size as libc::size_t;
                assert_eq!(libc::pthread_attr_setstacksize(&mut attr,
                                                           stack_size), 0);
                 */
                unimplemented!()
            }
        };

        let ret = libc::pthread_create(&mut native, &attr, thread_start,
                                       &*p as *const _ as *mut _);
        assert_eq!(libc::pthread_attr_destroy(&mut attr), 0);

        return if ret != 0 {
            Err(())
        } else {
            mem::forget(p); // ownership passed to pthread_create
            Ok(Thread { id: native })
        };

        extern fn thread_start(main: *mut libc::c_void) -> *mut libc::c_void {
            unsafe {
                Box::from_raw(main as *mut Box<FnBox()>)();
                ptr::null_mut()
            }
        }
    }

    pub fn yield_now() {
        let ret = unsafe { libc::sched_yield() };
        debug_assert_eq!(ret, 0);
    }

    pub fn set_name(_name: &str) {
        /*const PR_SET_NAME: libc::c_int = 15;
        let cname = CString::new(name).unwrap_or_else(|_| {
            panic!("thread name may not contain interior null bytes")
        });
        // pthread wrapper only appeared in glibc 2.12, so we use syscall
        // directly.
        unsafe {
            libc::prctl(PR_SET_NAME, cname.as_ptr() as libc::c_ulong, 0, 0, 0);
        }*/
        unimplemented!()
    }

    /*pub fn sleep(dur: Duration) {
        let mut ts = libc::timespec {
            tv_sec: dur.as_secs() as libc::time_t,
            tv_nsec: dur.subsec_nanos() as libc::c_long,
        };

        // If we're awoken with a signal then the return value will be -1 and
        // nanosleep will fill in `ts` with the remaining time.
        unsafe {
            while libc::nanosleep(&ts, &mut ts) == -1 {
                //assert_eq!(os::errno(), libc::EINTR);
            }
        }
    }*/

    pub fn join(self) {
        unsafe {
            let ret = libc::pthread_join(self.id, ptr::null_mut());
            mem::forget(self);
            debug_assert_eq!(ret, 0);
        }
    }

    pub fn id(&self) -> libc::pthread_t { self.id }

    pub fn into_id(self) -> libc::pthread_t {
        let id = self.id;
        mem::forget(self);
        id
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        let ret = unsafe { libc::pthread_detach(self.id) };
        debug_assert_eq!(ret, 0);
    }
}
