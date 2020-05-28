pub use sixtyfps_rs_macro::sixtyfps;

/// internal re_exports used by the macro generated
pub mod re_exports {
    pub use const_field_offset::{self, FieldOffsets};
    pub use corelib::abi::datastructures::{Component, ComponentTO, ComponentVTable, ItemTreeNode};
    pub use corelib::abi::primitives::*;
    pub use corelib::abi::properties::Property;
    pub use corelib::abi::signals::Signal;
    pub use corelib::ComponentVTable_static;
    pub use corelib::EvaluationContext;
    pub use corelib::SharedString;
    pub use gl::sixtyfps_runtime_run_component_with_gl_renderer;
    pub use once_cell::sync::Lazy;
    pub use vtable::{self, *};
}

#[cfg(doctest)]
mod compile_fail_tests;
