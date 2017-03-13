#![feature(const_fn, recover)]

#[macro_use]
extern crate minhook;
extern crate winapi;
extern crate user32;

use std::ptr;

use winapi::{HWND, LPCSTR, UINT, c_int};

#[allow(non_upper_case_globals)]
static MessageBoxA:
       ::StaticHook<unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT)
                        -> c_int> =
    {
        static __DATA:
               ::AtomicInitCell<::__StaticHookInner<unsafe extern "system" fn(HWND,
                                                                              LPCSTR,
                                                                              LPCSTR,
                                                                              UINT)
                                                        -> c_int>> =
            ::AtomicInitCell::new();
        #[inline(never)]
        unsafe extern "system" fn __detour(__arg_0: HWND, __arg_1: LPCSTR,
                                           __arg_2: LPCSTR, __arg_3: UINT)
         -> c_int {
            ::std::panic::catch_unwind(||
                                           {
                                               let &::__StaticHookInner(_,
                                                                        ref closure) =
                                                   __DATA.get().unwrap();
                                               closure(__arg_0, __arg_1,
                                                       __arg_2, __arg_3)
                                           }).unwrap_or_else(|payload|
                                                                 ::panic::__handle("original",
                                                                                   "MessageBoxA",
                                                                                   payload))
        }
        ::StaticHook::<unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT)
                           ->
                               c_int>::__new(&__DATA,
                                             ::__StaticHookTarget::Static(user32::MessageBoxA),
                                             __detour)
    };

fn main() {
  // Create a detour closure. This closure can capture any Sync variables.
  let detour = |wnd, text, caption, flags| unsafe { MessageBoxA.call_real(wnd, caption, text, flags) };

  // Install the hook.
  unsafe { MessageBoxA.initialize(detour).unwrap(); }

  let hello = b"Hello\0".as_ptr() as LPCSTR;
  let world = b"World\0".as_ptr() as LPCSTR;

  // Call the function.
  unsafe { user32::MessageBoxA(ptr::null_mut(), hello, world, winapi::MB_OK); }

  // Enable the hook.
  MessageBoxA.enable().unwrap();

  // Call the - now hooked - function.
  unsafe { user32::MessageBoxA(ptr::null_mut(), hello, world, winapi::MB_OK); }
}

