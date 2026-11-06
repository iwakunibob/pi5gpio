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


const ONTIME: u64 = 500;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking led_grp
     on a {}.", DeviceInfo::new()?.model());

    let mut led_group: Vec<rppal::gpio::OutputPin> = vec![];
    led_group.push(Gpio::new()?.get(LED16R)?.into_output());
    led_group.push(Gpio::new()?.get(LED17G)?.into_output());
    led_group.push(Gpio::new()?.get(LED22B)?.into_output());
    led_group.push(Gpio::new()?.get(LED23Y)?.into_output());
    led_group.push(Gpio::new()?.get(LED24R)?.into_output());
    led_group.push(Gpio::new()?.get(LED25G)?.into_output());
    led_group.push(Gpio::new()?.get(LED26B)?.into_output());
    led_group.push(Gpio::new()?.get(LED27Y)?.into_output());


    leds_left(&mut led_group, 600);
    leds_right(&mut led_group, 600);
    leds_left(&mut led_group, 300);
    leds_right(&mut led_group, 300);
    
    println!("Done");
    Ok(())
}

fn leds_left(led_grp: &mut [rppal::gpio::OutputPin], duration: u64) {
    for i in 0..led_grp.len(){
        led_grp[i].set_high();
        thread::sleep(Duration::from_millis(duration));
        led_grp[i].set_low();
    }
}

fn leds_right(led_grp: &mut [rppal::gpio::OutputPin], duration: u64) {
    for i in (0..led_grp.len()).rev() {
        led_grp[i].set_high();
        thread::sleep(Duration::from_millis(duration));
        led_grp[i].set_low();
    }
}


    // // led_grp
    // [0].set_high();
    // // thread::sleep(Duration::from_millis(1000));
    // // led_grp
    // [0].set_low();
    // // pin17g.set_high();
    // // thread::sleep(Duration::from_millis(1000));
    // // pin17g.set_low();
    // // pin22b.set_high();
    // // thread::sleep(Duration::from_millis(1000));
    // // pin22b.set_low();
    // // pin23y.set_high();
    // // thread::sleep(Duration::from_millis(1000));
    // // pin23y.set_low();



    // let mut pin16r: rppal::gpio::OutputPin = Gpio::new()?.get(LED16R)?.into_output();
    // let mut pin17g = Gpio::new()?.get(LED17G)?.into_output();
    // let mut pin22b = Gpio::new()?.get(LED22B)?.into_output();
    // let mut pin23y = Gpio::new()?.get(LED23Y)?.into_output();
