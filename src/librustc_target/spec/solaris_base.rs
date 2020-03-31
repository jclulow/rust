use crate::spec::TargetOptions;
use std::default::Default;

pub fn opts() -> TargetOptions {
    TargetOptions {
        dynamic_linking: true,
        executables: true,
        has_rpath: true,
        // Under some conditions when building C code, GCC may need to link
        // additional libraries; e.g., libssp.so.  Allow it to do so:
        no_default_libraries: false,
        target_family: Some("unix".to_string()),
        is_like_solaris: true,
        limit_rdylib_exports: false, // Linker doesn't support this

        ..Default::default()
    }
}
