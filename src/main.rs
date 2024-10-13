/*!
 * Show readouts of all ADC channels.
 *
 * This example displays values for all ADC channels over the serial console.  During startup, it
 * also displays the values for Vbandgap, GND, and a readout of the MCU's temperature sensor.  For
 * the meanings of these values, please reference the ATmega328P datasheet.
 *
 * Connections
 * -----------
 *  - `A0` - `A5`: Connect analog voltages as you like to see them read out.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use arduino_hal::adc;
use ufmt::derive::uDebug;

#[derive(uDebug)]
struct DataPack {
    x: u16, 
    y: u16
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    pins.d9.into_output();
    /* 
    let tc1 = dp.TC1;
    tc1.icr1.write(|w| w.bits(4999));
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
    
    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    */
    //ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).unwrap();
    //ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).unwrap();
    //ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).unwrap();


    //a0 --> yukarı=0 aşağı=1023
    let a0 = pins.a0.into_analog_input(&mut adc);
    //a1 --> sağ=0 sol=1024
    let a1 = pins.a1.into_analog_input(&mut adc);

    let button_pin = pins.d2.into_pull_up_input();
    let mut fire_pin = pins.d13.into_output();
    let mut data_pack = DataPack{
        x: 0,
        y: 0
    };

    loop {
        let mut is_high_in_numbers:u16 = 0;
        if button_pin.is_low() {
            fire_pin.set_low();
        }
        else {
            
            is_high_in_numbers = 1;
            fire_pin.set_high();
        }
 
        let values = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
            is_high_in_numbers
        ];

        /* 
        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).unwrap();
          
        }
        */
        //fire_pin.set_high();
         
        for i in 0..values.len() {
            //data_pack.x = values[0];
            //data_pack.y = values[1];
            
            ufmt::uwriteln!(&mut serial, "{} {} {}", values[0], values[1], values[2]);
        }
        
        //ufmt::uwriteln!(&mut serial, "{:?} ", values);
        
        
        arduino_hal::delay_ms(50);

        

/* 
        for duty in 100..=700 {
            tc1.ocr1a.write(|w| w.bits(duty));
            arduino_hal::delay_ms(20);
        }
*/    
    }
}