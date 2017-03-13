# For rustc issue

I tried [this example](https://github.com/Jascha-N/minhook-rs#example) by following command.

```cargo rustc --bin original -- -Z unstable-options --pretty=expanded```

The result was
```
$ cargo rustc --bin original -- -Z unstable-options --pretty=expanded
   Compiling nppElevatedSave v0.1.0 (file:///D:/Jeiea/Project/2017/03/rust-issue)
error: tuple struct `__StaticHookInner` is private
  --> src/original.rs:12:1
   |
12 |   static_hooks! {
   |  _^ starting here...
13 | |   // Create a hook for user32::MessageBoxA.
14 | |   impl MessageBoxA for user32::MessageBoxA: unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT) -> c_int;
15 | | }
   | |_^ ...ending here
   |
   = note: this error originates in a macro outside of the current crate

#![feature(prelude_import)]
#![no_std]
#![feature(const_fn, recover)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;

#[macro_use]
extern crate minhook;
extern crate winapi;
extern crate user32;

use std::ptr;

use winapi::{HWND, LPCSTR, UINT, c_int};

// Create a hook for user32::MessageBoxA.

// Create a detour closure. This closure can capture any Sync variables.

// Install the hook.


// Call the function.

// Enable the hook.

// Call the - now hooked - function.

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
    let detour =
        |wnd, text, caption, flags|
            unsafe { MessageBoxA.call_real(wnd, caption, text, flags) };
    unsafe { MessageBoxA.initialize(detour).unwrap(); }
    let hello = b"Hello\x00".as_ptr() as LPCSTR;
    let world = b"World\x00".as_ptr() as LPCSTR;
    unsafe {
        user32::MessageBoxA(ptr::null_mut(), hello, world, winapi::MB_OK);
    }
    MessageBoxA.enable().unwrap();
    unsafe {
        user32::MessageBoxA(ptr::null_mut(), hello, world, winapi::MB_OK);
    }
}
error: aborting due to previous error

error: Could not compile `nppElevatedSave`.

To learn more, run the command again with --verbose.
```

I think comment lines are out of sync.

And I tried building expanded form by

```cargo build --bin expanded```

And the result was
```
$ cargo build --bin expanded
   Compiling nppElevatedSave v0.1.0 (file:///D:/Jeiea/Project/2017/03/rust-issue)
error[E0412]: cannot find type `StaticHook` in the crate root
  --> src/expanded.rs:14:8
   |
14 |          ::StaticHook<unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT)
   |  ________^ starting here...
15 | |                         -> c_int> =
   | |_________________________________^ ...ending here: not found in the crate root
   |
   = help: possible candidate is found in another module, you can import it into scope:
             `use minhook::StaticHook;`

error[E0412]: cannot find type `AtomicInitCell` in the crate root
  --> src/expanded.rs:18:16
   |
18 |                  ::AtomicInitCell<::__StaticHookInner<unsafe extern "system" fn(HWND,
   |  ________________^ starting here...
19 | |                                                                               LPCSTR,
20 | |                                                                               LPCSTR,
21 | |                                                                               UINT)
22 | |                                                         -> c_int>> =
   | |__________________________________________________________________^ ...ending here: not found in the crate root
   |
   = help: possible candidate is found in another module, you can import it into scope:
             `use minhook::AtomicInitCell;`

error[E0412]: cannot find type `__StaticHookInner` in the crate root
  --> src/expanded.rs:18:33
   |
18 |                  ::AtomicInitCell<::__StaticHookInner<unsafe extern "system" fn(HWND,
   |  _________________________________^ starting here...
19 | |                                                                               LPCSTR,
20 | |                                                                               LPCSTR,
21 | |                                                                               UINT)
22 | |                                                         -> c_int>> =
   | |_________________________________________________________________^ ...ending here: not found in the crate root
   |
   = help: possible candidate is found in another module, you can import it into scope:
             `use minhook::__StaticHookInner;`

error[E0433]: failed to resolve. Maybe a missing `extern crate AtomicInitCell;`?
  --> src/expanded.rs:23:13
   |
23 |             ::AtomicInitCell::new();
   |             ^^^^^^^^^^^^^^^^^^^^^ Maybe a missing `extern crate AtomicInitCell;`?

error[E0531]: cannot find tuple struct/variant `__StaticHookInner` in the crate root
  --> src/expanded.rs:30:53
   |
30 |                                                let &::__StaticHookInner(_,
   |                                                     ^^^^^^^^^^^^^^^^^^^ not found in the crate root

error[E0433]: failed to resolve. Did you mean `minhook::panic`?
  --> src/expanded.rs:36:66
   |
36 |                                                                  ::panic::__handle("original",
   |                                                                  ^^^^^^^^^^^^^^^^^ Did you mean `minhook::panic`?

error[E0433]: failed to resolve. Maybe a missing `extern crate StaticHook;`?
  --> src/expanded.rs:40:9
   |
40 |           ::StaticHook::<unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT)
   |  _________^ starting here...
41 | |                            ->
42 | |                                c_int>::__new(&__DATA,
   | |____________________________________________^ ...ending here: Maybe a missing `extern crate StaticHook;`?

error[E0433]: failed to resolve. Maybe a missing `extern crate __StaticHookTarget;`?
  --> src/expanded.rs:43:46
   |
43 |                                              ::__StaticHookTarget::Static(user32::MessageBoxA),
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Maybe a missing `extern crate __StaticHookTarget;`?

error: aborting due to 8 previous errors

error: Could not compile `nppElevatedSave`.

To learn more, run the command again with --verbose.
```

It seems `$crate` isn't resolved well.