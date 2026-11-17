// Integration test for SigmaOS-system-performance-profiling
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use system_performance_profiling as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
