pub use atmega_hal::port::mode;
pub use atmega_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    pub struct Pins from atmega_hal::Pins {
        pub endtemp: atmega_hal::port::PA7 = pa7,
        pub bedtemp: atmega_hal::port::PA6 = pa6,
        pub z_enable: atmega_hal::port::PA5 = pa5,
        pub ext_a4: atmega_hal::port::PA4 = pa4,
        pub ext_a3: atmega_hal::port::PA3 = pa3,
        pub ext_a2: atmega_hal::port::PA2 = pa2,
        pub ext_a1: atmega_hal::port::PA1 = pa1,
        pub sd_ss: atmega_hal::port::PA0 = pa0,

        pub sclk: atmega_hal::port::PB7 = pb7,
        pub miso: atmega_hal::port::PB6 = pb6,
        pub mosi: atmega_hal::port::PB5 = pb5,
        pub fan: atmega_hal::port::PB4 = pb4,
        pub z_step: atmega_hal::port::PB3 = pb3,
        pub z_dir: atmega_hal::port::PB2 = pb2,
        pub e_step: atmega_hal::port::PB1 = pb1,
        pub e_dir: atmega_hal::port::PB0 = pb0,

        pub y_dir: atmega_hal::port::PC7 = pc7,
        pub y_step: atmega_hal::port::PC6 = pc6,
        pub x_dir: atmega_hal::port::PC5 = pc5,
        pub z_stop: atmega_hal::port::PC4 = pc4,
        pub y_stop: atmega_hal::port::PC3 = pc3,
        pub x_stop: atmega_hal::port::PC2 = pc2,
        pub ext_sda: atmega_hal::port::PC1 = pc1,
        pub ext_scl: atmega_hal::port::PC0 = pc0,

        pub x_step: atmega_hal::port::PD7 = pd7,
        pub xy_enable: atmega_hal::port::PD6 = pd6,
        pub hotend: atmega_hal::port::PD5 = pd5,
        pub hotbed: atmega_hal::port::PD4 = pd4,
        pub ext_tx1: atmega_hal::port::PD3 = pd3,
        pub ext_rx1: atmega_hal::port::PD2 = pd2,
        pub aofi: atmega_hal::port::PD1 = pd1,
        pub aifo: atmega_hal::port::PD0 = pd0,
    }
}
