use zanistarast_core::config_loader::load_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config/runtime.toml")?;

    println!("ZANISTARAST CONFIG EXAMPLE");
    println!("--------------------------");
    println!("Deterministic: {}", config.runtime.deterministic);
    println!("Replay Enabled: {}", config.runtime.replay_enabled);
    println!("Strict Mode: {}", config.verification.strict_mode);
    println!("Stop On Failure: {}", config.verification.stop_on_failure);
    println!("Default Provider: {}", config.provider.default_provider);
    println!("Logging Level: {}", config.logging.level);

    Ok(())
}


