# vtables
Interact with C++ virtual method tables from Rust

## Usage
Clone both this and [vtables_derive](https://github.com/not-wlan/vtables_derive) and add the following to your Cargo.toml:

```
[dependencies]
vtables = { path = "../vtables" }
vtables_derive = { path = "../vtables_derive" }
```

You can then use methods from the virtual method table like this:
```
use vtables::VTable;
use vtables_derive::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineClient {
}

#[allow(non_snake_case)]
impl EngineClient {
    #[virtual_index(5)]
    pub fn GetScreenSize(&self, width: *mut i32, height: *mut i32) {}
    
    #[virtual_index(26)]
    pub fn IsInGame(&self) -> bool {}

    #[virtual_index(108)]
    pub fn ExecuteClientCmd(&self, command: *const c_char) {}

    #[virtual_index(113)]
    pub fn ClientCmd_Unrestricted(&self, command: *const c_char) {}
}
```
A field containing the virtual method table pointer will automatically be added to your structure. Support for multiple inheritance is untested.
