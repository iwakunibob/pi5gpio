use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

const LED16R: u8 = 16; // red
const LED17G: u8 = 17; // green
const LED22B: u8 = 22; // blue
const LED23Y: u8 = 23; // yellow

const LED24R: u8 = 24; // red
const LED25G: u8 = 25; // green
const LED26B: u8 = 26; // blue
const LED27Y: u8 = 27; // yellow

const LOOPS: u16 = 50;
const ONTIME: u64 = 500;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking LEDs on a {}.", DeviceInfo::new()?.model());

    let mut leds: Vec<rppal::gpio::OutputPin> = vec![];
    leds.push(Gpio::new()?.get(LED16R)?.into_output());
    leds.push(Gpio::new()?.get(LED17G)?.into_output());
    leds.push(Gpio::new()?.get(LED22B)?.into_output());
    leds.push(Gpio::new()?.get(LED23Y)?.into_output());
   
    leds.push(Gpio::new()?.get(LED24R)?.into_output());
    leds.push(Gpio::new()?.get(LED25G)?.into_output());
    leds.push(Gpio::new()?.get(LED26B)?.into_output());
    leds.push(Gpio::new()?.get(LED27Y)?.into_output());

    // let mut pin16r: rppal::gpio::OutputPin = Gpio::new()?.get(LED16R)?.into_output();
    // let mut pin17g = Gpio::new()?.get(LED17G)?.into_output();
    // let mut pin22b = Gpio::new()?.get(LED22B)?.into_output();
    // let mut pin23y = Gpio::new()?.get(LED23Y)?.into_output();


    // Blink the LED by setting the pin's logic level high for 500 ms.
    for m in 1..=LOOPS {
        println!("\nLoop Pass {}", m);
        for i in 0..=7 {
            leds[i].set_high();
            println!("    LED {} is on", i);
            thread::sleep(Duration::from_millis(ONTIME));
            leds[i].set_low();
        }
    }
    println!("Done");
   
    // leds[0].set_high();
    // thread::sleep(Duration::from_millis(1000));
    // leds[0].set_low();
    // pin17g.set_high();
    // thread::sleep(Duration::from_millis(1000));
    // pin17g.set_low();
    // pin22b.set_high();
    // thread::sleep(Duration::from_millis(1000));
    // pin22b.set_low();
    // pin23y.set_high();
    // thread::sleep(Duration::from_millis(1000));
    // pin23y.set_low();

    Ok(())
}