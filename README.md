# Failure
When provided with an `ssh://` url the cargo `Workspace::new` call panics. 

Here is a snipped backtrace with the relevent cargo and tarpaulin entries.

```
13: core::result::unwrap_failed
	at src/libcore/result.rs:1188
14: cargo::util::canonical_url::CanonicalUrl::new
15: cargo::core::source::source_id::SourceId::new
16: cargo::util::toml::DetailedTomlDependency::to_dependency
17: cargo::util::toml::TomlDependency::to_dependency
18: cargo::util::toml::TomlManifest::to_real_manifest::process_dependencies
19: cargo::util::toml::TomlManifest::to_real_manifest
20: cargo::util::toml::read_manifest
21: cargo::core::workspace::Packages::load
22: cargo::core::workspace::Workspace::find_root
23: cargo::core::workspace::Workspace::new
24: cargo_tarpaulin::launch_tarpaulin
25: cargo_tarpaulin::run
26: cargo_tarpaulin::main
```
