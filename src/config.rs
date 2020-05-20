use std::fs::read_to_string;

use linked_hash_map::{Entries, LinkedHashMap};
use yaml_rust::{yaml::Yaml, YamlLoader};

static OPTION_GROUPS: &'static [&'static str] =
    &["soma", "axon", "dendrite", "apicaldendrite", "undefined"];

pub struct Config {
    option_groups: LinkedHashMap<&'static str, ConfigOptionGroup>,
}

impl Config {
    pub fn new() -> Result<Config, YamlParseError> {
        let mut config = Config {
            option_groups: LinkedHashMap::new(),
        };
        for group in OPTION_GROUPS {
            config.option_groups.insert(group, ConfigOptionGroup::new());
        }

        let default_config_bytes = include_bytes!("default_config.yml");
        let default_config_yaml = Config::try_parse_yaml(
            std::str::from_utf8(default_config_bytes)
                .expect("Could not parse default config as str."),
        )?;
        config.try_overload_from_yaml(default_config_yaml)?;
        return Ok(config);
    }

    pub fn try_overload_from_file(&mut self, filename: &str) -> Result<(), YamlParseError> {
        let mut yaml = Config::try_parse_yaml_file(filename)?;
        self.try_overload_from_yaml(yaml)
    }

    /// Load the contents of a file as a Yaml object.
    fn try_parse_yaml_file(filename: &str) -> Result<Yaml, YamlParseError> {
        let yaml_string = Config::try_read_file(filename)?;
        let yaml_object = Config::try_parse_yaml(&yaml_string)?;
        return Ok(yaml_object);
    }

    fn try_overload_from_yaml(&mut self, yaml: Yaml) -> Result<(), YamlParseError> {
        // Check whether YAML config file contains a hash (which it should)
        match yaml {
            Yaml::Hash(mut top_level_hash) => {
                // Iterate over top level options that might be in the config file.
                for group in OPTION_GROUPS {
                    // Check whether each config option is there.
                    match top_level_hash.get_mut(&Yaml::from_str(*group)) {
                        Some(mut yaml) => {
                            // Check whether config option is a Hash, if it exists.
                            match yaml {
                                // If it is a hash, parse it.
                                Yaml::Hash(hash) => {
                                    let option_group = parse_config_entries(&mut hash.entries())?;
                                    self.option_groups
                                        .get_mut(*group)
                                        .expect(&format!(
                                            "Could not get group {} even though it exists",
                                            group
                                        ))
                                        .override_options(option_group);
                                }
                                // If it is not a hash, return an Err.
                                _ => {
                                    return Err(YamlParseError::WrongType(format!(
                                        "Expected config group {} to be a hash.",
                                        group
                                    )))
                                }
                            }
                        }
                        // It is OK for a config group to be left out of a file.
                        None => continue,
                    }
                }
            }
            _ => {
                return Err(YamlParseError::WrongType(
                    "Expected contents of config YAML to be a Hash.".to_string(),
                ))
            }
        }

        return Ok(());
    }

    fn try_parse_yaml(yaml_string: &str) -> Result<Yaml, YamlParseError> {
        let config;
        match YamlLoader::load_from_str(&yaml_string) {
            Ok(yaml) => config = yaml,
            Err(_) => {
                return Err(YamlParseError::FileRead(
                    "Could not parse contents of configuration file as YAML".to_string(),
                ))
            }
        }
        debug_assert!(config.len() == 1);

        return Ok(config[0].clone());
    }

    fn try_read_file(filename: &str) -> Result<String, YamlParseError> {
        let yaml_string;
        match read_to_string(filename) {
            Ok(string) => yaml_string = string,
            Err(msg) => {
                return Err(YamlParseError::FileRead(format!(
                    "Could not open configuration file {}: {}",
                    filename, msg
                )))
            }
        }
        return Ok(yaml_string);
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn construct_config() {
        match Config::new() {
            Ok(_) => {}
            Err(parse_error) => match parse_error {
                YamlParseError::WrongType(msg) => panic!(format!(
                    "Could not construct Config due to `YamlParseError::Wrongtype`: {}",
                    msg
                )),
                YamlParseError::FileRead(msg) => panic!(format!(
                    "Could not construct Config due to `YamlParseError::FileRead`: {}",
                    msg
                )),
                YamlParseError::BadValue => {
                    panic!("Could not construct Config due to `YamlParseError::BadValue`")
                }
            },
        }
    }
}

struct ConfigOptionGroup {
    options: LinkedHashMap<String, Option<String>>,
}

impl ConfigOptionGroup {
    fn new() -> ConfigOptionGroup {
        ConfigOptionGroup {
            options: LinkedHashMap::<String, Option<String>>::new(),
        }
    }

    fn override_options(&mut self, mut overrides: ConfigOptionGroup) {
        for entry in overrides.options.entries() {
            self.options
                .insert(entry.key().clone(), entry.get().clone());
        }
    }
}

/// Parse the entries in a `Yaml::Hash` into a `ConfigOptionGroup`.
///
/// `Yaml::Null` variants result in a value of `Option::None`, and all other
/// valid `Yaml` variants are coerced to `Option::Some(String)`.
fn parse_config_entries(
    entries: &mut Entries<Yaml, Yaml>,
) -> Result<ConfigOptionGroup, YamlParseError> {
    let mut group = ConfigOptionGroup::new();
    for entry in entries {
        let key: String = entry
            .key()
            .as_str()
            .expect("Could not get Yaml key as String.")
            .to_string();
        let val: Option<String>;
        match entry.get() {
            Yaml::Null => val = None,
            Yaml::String(string) => val = Some(string.clone()),
            Yaml::Real(num) => val = Some(num.as_str().to_string()),
            Yaml::Integer(num) => val = Some(num.to_string()),
            Yaml::Boolean(bool_value) => val = Some(bool_value.to_string()),
            _ => {
                return Err(YamlParseError::WrongType(format!(
                    "Expected value of YAML {} to be Null or String-like.",
                    entry.key().as_str().unwrap()
                )))
            }
        }
        group.options.insert(key, val);
    }
    return Ok(group);
}

pub enum YamlParseError {
    /// Yaml enum is not the expected variant (see `yaml_rust::yaml::Yaml`).
    WrongType(String),
    /// Yaml object does not exist (see `yaml_rust::yaml::Yaml::BadValue`).
    BadValue,
    /// Could not read Yaml from a file.
    FileRead(String),
}

#[cfg(test)]
mod parse_config_entries_tests {
    use super::*;

    fn load_hash_from_str(string: &str) -> LinkedHashMap<Yaml, Yaml> {
        let doc = YamlLoader::load_from_str(string)
            .expect(&format!("Could not load {} as a yaml string", string))[0]
            .clone();
        doc.into_hash().expect("Could not create yaml hash")
    }

    #[test]
    fn parse_single_string_entry() {
        let mut yaml = load_hash_from_str("key: value");
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key"),
            "Resulting HashMap does not contain expected key 'key'"
        );
        assert_eq!(
            parsed.options["key"],
            Some("value".to_string()),
            "Expected value associated with key 'key' to be 'value'"
        );
    }

    #[test]
    fn parse_multiple_string_entries() {
        let yaml_string = "key1: value1\nkey2: value2";
        let mut yaml = load_hash_from_str(yaml_string);
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key1"),
            "Resulting HashMap does not contain expected key 'key1'"
        );
        assert!(
            parsed.options.contains_key("key2"),
            "Resulting HashMap does not contain expected key 'key2'"
        );
        assert_eq!(
            parsed.options["key1"],
            Some("value1".to_string()),
            "Expected value associated with key 'key1' to be 'value1'"
        );
        assert_eq!(
            parsed.options["key2"],
            Some("value2".to_string()),
            "Expected value associated with key 'key2' to be 'value2'"
        );
    }

    #[test]
    fn parse_float_as_string() {
        let mut yaml = load_hash_from_str("key: 1.23");
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key"),
            "Resulting HashMap does not contain expected key 'key'"
        );
        assert_eq!(
            parsed.options["key"],
            Some("1.23".to_string()),
            "Expected value associated with key 'key' to be '1.23'"
        );
    }

    #[test]
    fn parse_int_as_string() {
        let mut yaml = load_hash_from_str("key: 23");
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key"),
            "Resulting HashMap does not contain expected key 'key'"
        );
        assert_eq!(
            parsed.options["key"],
            Some("23".to_string()),
            "Expected value associated with key 'key' to be '23'"
        );
    }

    #[test]
    fn parse_bool_true_as_string() {
        let mut yaml = load_hash_from_str("key: true");
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key"),
            "Resulting HashMap does not contain expected key 'key'"
        );
        assert_eq!(
            parsed.options["key"],
            Some("true".to_string()),
            "Expected value associated with key 'key' to be 'true'"
        );
    }

    #[test]
    fn parse_bool_True_as_string() {
        let mut yaml = load_hash_from_str("key: True");
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key"),
            "Resulting HashMap does not contain expected key 'key'"
        );
        assert_eq!(
            parsed.options["key"],
            Some("True".to_string()),
            "Expected value associated with key 'key' to be 'True'"
        );
    }

    #[test]
    fn parse_bool_TRUE_as_string() {
        let mut yaml = load_hash_from_str("key: TRUE");
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key"),
            "Resulting HashMap does not contain expected key 'key'"
        );
        assert_eq!(
            parsed.options["key"],
            Some("TRUE".to_string()),
            "Expected value associated with key 'key' to be 'TRUE'"
        );
    }

    #[test]
    fn parse_empty_value_as_None() {
        let mut yaml = load_hash_from_str("key:");
        let parsed;
        match parse_config_entries(&mut yaml.entries()) {
            Ok(result) => parsed = result,
            Err(_) => panic!("Could not parse hash entry as String key and Option<String> value"),
        }

        assert!(
            parsed.options.contains_key("key"),
            "Resulting HashMap does not contain expected key 'key'"
        );
        assert_eq!(
            parsed.options["key"], None,
            "Expected value associated with key 'key' to be None"
        );
    }
}
