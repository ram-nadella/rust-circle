fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let circle_build_num =
        std::env::var("CIRCLE_BUILD_NUM").map_err(|_| "CIRCLE_BUILD_NUM env var is not set")?;
    println!("Value of CIRCLE_BUILD_NUM env var is: {}", circle_build_num);
    Ok(())
}
