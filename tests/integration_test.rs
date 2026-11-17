//! Integration tests for SigmaOS-system-performance-profiling.

use sigmaos_system_performance_profiling_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
