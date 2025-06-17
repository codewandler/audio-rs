use cpal::traits::{DeviceTrait, HostTrait};

pub fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    println!("host: {:?}", host.id().name());
    let devices = host.devices()?;
    println!("devices");
    for x in devices {
        if let Ok(name) = x.name() {
            println!("  - device: {}", name);

            match x.default_input_config() {
                Ok(x) => println!("    - in: {:?}", x),
                Err(e) => println!("    - in(ERR): {:?}", e),
            }

            match x.default_output_config() {
                Ok(x) => println!("    - out: {:?}", x),
                Err(e) => println!("    - out(ERR): {:?}", e),
            }
        }
    }

    Ok(())
}
