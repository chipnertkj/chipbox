pub mod hot;

fn main() {
    use cpal::traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _};
    use rb::{RbConsumer as _, RB as _};

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();
    println!("default output config: {:#?}", config);
    let rb = rb::SpscRb::new(1024);
    let consumer = rb.consumer();
    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let result = consumer.read(data);
                match result {
                    Ok(n) => {
                        if n < data.len() {
                            eprintln!("partial underflow");
                        }
                    }
                    Err(rb::RbError::Empty) => {
                        eprintln!("total underflow")
                    }
                    Err(err) => panic!("unexpected error: {}", err),
                }
            },
            |err| eprintln!("an error occurred on the output stream: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();
}
