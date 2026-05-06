use serde::{Deserialize, Serialize};

/// TOML schema for the zerobrew package manifest.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub metadata: Metadata,
    #[serde(default)]
    pub formulas: Vec<PackageEntry>,
    #[serde(default)]
    pub casks: Vec<PackageEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub schema_version: u32,
    pub exported_at: String,
    pub zerobrew_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageEntry {
    pub name: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub build_from_source: bool,
}

pub const SCHEMA_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_serialize_deserialize() {
        let manifest = PackageManifest {
            metadata: Metadata {
                schema_version: 1,
                exported_at: "2025-05-06T10:00:00+08:00".to_string(),
                zerobrew_version: "0.2.1".to_string(),
            },
            formulas: vec![
                PackageEntry {
                    name: "jq".to_string(),
                    version: "1.7.1".to_string(),
                    build_from_source: false,
                },
                PackageEntry {
                    name: "wget".to_string(),
                    version: "1.21.4".to_string(),
                    build_from_source: true,
                },
            ],
            casks: vec![PackageEntry {
                name: "firefox".to_string(),
                version: "120.0".to_string(),
                build_from_source: false,
            }],
        };

        let toml_str = toml::to_string_pretty(&manifest).unwrap();
        let parsed: PackageManifest = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.metadata.schema_version, 1);
        assert_eq!(parsed.formulas.len(), 2);
        assert_eq!(parsed.formulas[0].name, "jq");
        assert!(!parsed.formulas[0].build_from_source);
        assert!(parsed.formulas[1].build_from_source);
        assert_eq!(parsed.casks.len(), 1);
        assert_eq!(parsed.casks[0].name, "firefox");
    }

    #[test]
    fn deserialize_minimal() {
        let input = r#"
[metadata]
schema_version = 1
exported_at = "2025-01-01T00:00:00Z"
zerobrew_version = "0.1.0"

[[formulas]]
name = "git"
version = "2.44"
"#;
        let manifest: PackageManifest = toml::from_str(input).unwrap();
        assert_eq!(manifest.formulas.len(), 1);
        assert_eq!(manifest.formulas[0].name, "git");
        assert!(!manifest.formulas[0].build_from_source);
        assert!(manifest.casks.is_empty());
    }

    #[test]
    fn build_from_source_false_omitted_in_output() {
        let entry = PackageEntry {
            name: "jq".to_string(),
            version: "1.7".to_string(),
            build_from_source: false,
        };
        let output = toml::to_string_pretty(&entry).unwrap();
        assert!(!output.contains("build_from_source"));
    }

    #[test]
    fn build_from_source_true_included_in_output() {
        let entry = PackageEntry {
            name: "jq".to_string(),
            version: "1.7".to_string(),
            build_from_source: true,
        };
        let output = toml::to_string_pretty(&entry).unwrap();
        assert!(output.contains("build_from_source = true"));
    }
}
