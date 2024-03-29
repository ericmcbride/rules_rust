//! Common utilities

pub(crate) mod starlark;
pub(crate) mod target_triple;

pub(crate) const CRATES_IO_INDEX_URL: &str = "https://github.com/rust-lang/crates.io-index";

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// Convert a string into a valid crate module name by applying transforms to invalid characters
pub(crate) fn sanitize_module_name(name: &str) -> String {
    name.replace('-', "_")
}

/// Some character which may be present in version IDs are not valid
/// in Bazel repository names. This converts invalid characters. See
/// [RepositoryName.java](https://github.com/bazelbuild/bazel/blob/4.0.0/src/main/java/com/google/devtools/build/lib/cmdline/RepositoryName.java#L42)
pub(crate) fn sanitize_repository_name(name: &str) -> String {
    name.replace('+', "-")
}

pub(crate) fn sanitize_vendor_file_names(outputs: &BTreeMap<PathBuf, String>) -> BTreeSet<PathBuf> {
    outputs
        .keys()
        .cloned()
        .map(|p| {
            let p_str = sanitize_repository_name(p.to_str().unwrap());
            PathBuf::from(p_str)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sanitize_vendor_file_names() {
        let mut outputs = BTreeMap::new();
        outputs.insert(
            PathBuf::from("/path/to/libbpf-sys-1.3.0+v1.3.0"),
            "test".into(),
        );

        let got = sanitize_vendor_file_names(&outputs);
        for value in got {
            assert_eq!(value, PathBuf::from("/path/to/libbpf-sys-1.3.0-v1.3.0"))
        }
    }

    #[test]
    fn test_sanitize_vendor_file_names_no_change() {
        let mut outputs = BTreeMap::new();
        outputs.insert(PathBuf::from("/path/to/tokio-1.20.0"), "test".into());

        let got = sanitize_vendor_file_names(&outputs);
        for value in got {
            assert_eq!(value, PathBuf::from("/path/to/tokio-1.20.0"))
        }
    }

    #[test]
    fn test_sanitize_repository_name() {
        let name = "anyhow-1.0.0+semver_meta";
        let got = sanitize_repository_name(&name);
        assert_eq!(got, String::from("anyhow-1.0.0-semver_meta"));
    }

    #[test]
    fn test_sanitize_repository_name_no_change() {
        let name = "tokio-1.20.0";
        let got = sanitize_repository_name(&name);
        assert_eq!(got, String::from("tokio-1.20.0"));
    }
}
