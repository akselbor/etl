//! Compile coverage for embedders using only ETL's production features.

/// The provisioning entrypoint must be reachable without exposing Postgres
/// internals.
#[test]
fn source_migration_runner_is_public() {
    let _runner = etl::run_source_migrations;
}
