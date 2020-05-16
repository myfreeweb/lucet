use libc::{c_void, ucontext_t};

#[derive(Clone, Copy, Debug)]
pub struct UContextPtr(*mut ucontext_t);

impl UContextPtr {
    #[inline]
    pub fn new(ptr: *mut c_void) -> Self {
        assert!(!ptr.is_null(), "non-null context");
        UContextPtr(ptr as *mut ucontext_t)
    }

    #[inline]
    pub fn get_ip(self) -> *const c_void {
        let mcontext = &unsafe { self.0.as_ref().unwrap() }.uc_mcontext;
        mcontext.mc_rip as *const _
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UContext {
    context: *mut ucontext_t,
}

impl UContext {
    #[inline]
    pub fn new(ptr: *mut c_void) -> Self {
        UContext {
            context: unsafe { (ptr as *mut ucontext_t).as_mut().expect("non-null context") },
        }
    }

    pub fn as_ptr(&mut self) -> UContextPtr {
        UContextPtr::new(self.context as *mut _ as *mut _)
    }
}

impl Into<UContext> for UContextPtr {
    #[inline]
    fn into(self) -> UContext {
        UContext { context: self.0 }
    }
}
