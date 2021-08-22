fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    read_and_print_env_var("CIRCLE_BUILD_NUM")?;
    read_and_print_env_var("CIRCLE_USERNAME")?;
    read_and_print_env_var("CUSTOM_PROJECT_VAR")?;
    Ok(())
}

fn read_and_print_env_var(env_var_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let env_var_value =
        std::env::var(env_var_name).map_err(|_| format!("{} env var is not set", env_var_name))?;
    println!("Value of {} env var is: {}", env_var_name, env_var_value);
    Ok(())
}
