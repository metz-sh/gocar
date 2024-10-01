use config::Config;
use std::{collections::HashMap, sync::OnceLock};


pub const CONFIG_FILE_NAME: &str = ".gocar.json";

pub fn config() -> &'static Option<Config> {
	static CONFIG: OnceLock<Option<Config>> = OnceLock::new();
    CONFIG.get_or_init(|| {
	    Config::builder()
	       .add_source(config::File::with_name(CONFIG_FILE_NAME))
	       .build().ok()
    })
}

pub fn get_post_install(script_type: &str) -> Option<HashMap<String, config::Value>> {
    config().as_ref().and_then(|c| c.get_table(&format!("scripts.{}", script_type)).ok())
}

pub fn get_post_install_script(script_type: &str, package_name: &String) -> Option<String> {
	let post_install = get_post_install(script_type)?;
	let value = &post_install.get(package_name.as_str())?.kind;
	Some(value.to_string())
}

pub fn get_registry_path() -> &'static Option<String> {
	static PATH: OnceLock<Option<String>> = OnceLock::new();
	PATH.get_or_init(|| {
		config()
		.as_ref()
		.and_then(|c| c.get_string("registry").ok())
	})
}
