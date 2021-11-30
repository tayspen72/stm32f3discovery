//==============================================================================
// Notes
//==============================================================================
// mcu/mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use stm32f3::stm32f303;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum GpioPort {
	PortA,
	PortB,
	PortC,
	PortD,
	PortE,
	PortF,
	PortG,
	PortH
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum GpioMode {
	Input,
	Output,
	AltFunc,
	Analog
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum OutputType {
	PushPull,
	OpenDrain
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PinPull {
	NoPull,
	PullUp,
	PullDown
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum PinState {
	PinLow = 0,
	PinHigh = 1
}

//==============================================================================
// Variables
//==============================================================================
static GPIOA_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOA>>> = 
	Mutex::new(RefCell::new(None));
static GPIOB_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOB>>> = 
	Mutex::new(RefCell::new(None));
static GPIOC_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOC>>> = 
	Mutex::new(RefCell::new(None));
static GPIOD_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOD>>> = 
	Mutex::new(RefCell::new(None));
static GPIOE_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOE>>> = 
	Mutex::new(RefCell::new(None));
static GPIOF_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOF>>> = 
	Mutex::new(RefCell::new(None));
static GPIOG_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOG>>> = 
	Mutex::new(RefCell::new(None));
static GPIOH_HANDLE: Mutex<RefCell<Option<stm32f303::GPIOH>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(
	gpioa: stm32f303::GPIOA,
	gpiob: stm32f303::GPIOB,
	gpioc: stm32f303::GPIOC,
	gpiod: stm32f303::GPIOD,
	gpioe: stm32f303::GPIOE,
	gpiof: stm32f303::GPIOF,
	gpiog: stm32f303::GPIOG,
	gpioh: stm32f303::GPIOH) {

	free(|cs| GPIOA_HANDLE.borrow(cs).replace(Some(gpioa)));
	free(|cs| GPIOB_HANDLE.borrow(cs).replace(Some(gpiob)));
	free(|cs| GPIOC_HANDLE.borrow(cs).replace(Some(gpioc)));
	free(|cs| GPIOD_HANDLE.borrow(cs).replace(Some(gpiod)));
	free(|cs| GPIOE_HANDLE.borrow(cs).replace(Some(gpioe)));
	free(|cs| GPIOF_HANDLE.borrow(cs).replace(Some(gpiof)));
	free(|cs| GPIOG_HANDLE.borrow(cs).replace(Some(gpiog)));
	free(|cs| GPIOH_HANDLE.borrow(cs).replace(Some(gpioh)));
}

#[allow(dead_code)]
pub fn get_pin_state(port: GpioPort, pin: u8) -> Option<PinState> {
	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => if gpio.idr.read().idr0().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					1 => if gpio.idr.read().idr1().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					2 => if gpio.idr.read().idr2().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					3 => if gpio.idr.read().idr3().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					4 => if gpio.idr.read().idr4().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					5 => if gpio.idr.read().idr5().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					6 => if gpio.idr.read().idr6().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					7 => if gpio.idr.read().idr7().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					8 => if gpio.idr.read().idr8().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					9 => if gpio.idr.read().idr9().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					10 => if gpio.idr.read().idr10().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					11 => if gpio.idr.read().idr11().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					12 => if gpio.idr.read().idr12().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					13 => if gpio.idr.read().idr13().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					14 => if gpio.idr.read().idr14().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					15 => if gpio.idr.read().idr15().is_high() { Some(PinState::PinHigh) } else { Some(PinState::PinLow) },
					_ => None
				}
			} else { None },
		}
	})
}

#[allow(dead_code)]
pub fn pin_setup(port: GpioPort, pin: u8, mode: GpioMode, pull: PinPull, state: PinState) {
	set_pin_mode(port, pin, mode);
	set_pin_pull(port, pin, pull);
	set_pin_state(port, pin, state);
}

#[allow(dead_code)]
pub fn set_alt_func(port: GpioPort, pin: u8, func: u8) {
	let func = func & 0xF;

	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.afrl.modify(|_, w| w.afrl0().bits(func)),
					1 => gpio.afrl.modify(|_, w| w.afrl1().bits(func)),
					2 => gpio.afrl.modify(|_, w| w.afrl2().bits(func)),
					3 => gpio.afrl.modify(|_, w| w.afrl3().bits(func)),
					4 => gpio.afrl.modify(|_, w| w.afrl4().bits(func)),
					5 => gpio.afrl.modify(|_, w| w.afrl5().bits(func)),
					6 => gpio.afrl.modify(|_, w| w.afrl6().bits(func)),
					7 => gpio.afrl.modify(|_, w| w.afrl7().bits(func)),
					8 => gpio.afrh.modify(|_, w| w.afrh8().bits(func)),
					9 => gpio.afrh.modify(|_, w| w.afrh9().bits(func)),
					10 => gpio.afrh.modify(|_, w| w.afrh10().bits(func)),
					11 => gpio.afrh.modify(|_, w| w.afrh11().bits(func)),
					12 => gpio.afrh.modify(|_, w| w.afrh12().bits(func)),
					13 => gpio.afrh.modify(|_, w| w.afrh13().bits(func)),
					14 => gpio.afrh.modify(|_, w| w.afrh14().bits(func)),
					15 => gpio.afrh.modify(|_, w| w.afrh15().bits(func)),
					_ => ()
				};
			},
		}
	});
}

#[allow(dead_code)]
pub fn set_pin_state(port: GpioPort, pin: u8, state: PinState) {
	let state = state == PinState::PinHigh;

	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match pin {
					0 => gpio.bsrr.write(| w | w.br0().bit(!state).bs0().bit(state)),
					1 => gpio.bsrr.write(| w | w.br1().bit(!state).bs1().bit(state)),
					2 => gpio.bsrr.write(| w | w.br2().bit(!state).bs2().bit(state)),
					3 => gpio.bsrr.write(| w | w.br3().bit(!state).bs3().bit(state)),
					4 => gpio.bsrr.write(| w | w.br4().bit(!state).bs4().bit(state)),
					5 => gpio.bsrr.write(| w | w.br5().bit(!state).bs5().bit(state)),
					6 => gpio.bsrr.write(| w | w.br6().bit(!state).bs6().bit(state)),
					7 => gpio.bsrr.write(| w | w.br7().bit(!state).bs7().bit(state)),
					8 => gpio.bsrr.write(| w | w.br8().bit(!state).bs8().bit(state)),
					9 => gpio.bsrr.write(| w | w.br9().bit(!state).bs9().bit(state)),
					10 => gpio.bsrr.write(| w | w.br10().bit(!state).bs10().bit(state)),
					11 => gpio.bsrr.write(| w | w.br11().bit(!state).bs11().bit(state)),
					12 => gpio.bsrr.write(| w | w.br12().bit(!state).bs12().bit(state)),
					13 => gpio.bsrr.write(| w | w.br13().bit(!state).bs13().bit(state)),
					14 => gpio.bsrr.write(| w | w.br14().bit(!state).bs14().bit(state)),
					15 => gpio.bsrr.write(| w | w.br15().bit(!state).bs15().bit(state)),
					_ => ()
				}
			},
		}
	});
}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn set_output_type(port: GpioPort, pin: u8, out_type: OutputType) {
	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpioa::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpioa::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpiob::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpiob::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpioc::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpioc::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpioc::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpioc::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpioc::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpioc::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpioc::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpioc::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpioc::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpioc::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let out_type = match out_type {
					OutputType::PushPull => stm32f303::gpioc::otyper::OT0_A::PUSHPULL,
					OutputType::OpenDrain => stm32f303::gpioc::otyper::OT0_A::OPENDRAIN,
				};
				match pin {
					0 => gpio.otyper.modify(|_, w| w.ot0().variant(out_type)),
					1 => gpio.otyper.modify(|_, w| w.ot1().variant(out_type)),
					2 => gpio.otyper.modify(|_, w| w.ot2().variant(out_type)),
					3 => gpio.otyper.modify(|_, w| w.ot3().variant(out_type)),
					4 => gpio.otyper.modify(|_, w| w.ot4().variant(out_type)),
					5 => gpio.otyper.modify(|_, w| w.ot5().variant(out_type)),
					6 => gpio.otyper.modify(|_, w| w.ot6().variant(out_type)),
					7 => gpio.otyper.modify(|_, w| w.ot7().variant(out_type)),
					8 => gpio.otyper.modify(|_, w| w.ot8().variant(out_type)),
					9 => gpio.otyper.modify(|_, w| w.ot9().variant(out_type)),
					10 => gpio.otyper.modify(|_, w| w.ot10().variant(out_type)),
					11 => gpio.otyper.modify(|_, w| w.ot11().variant(out_type)),
					12 => gpio.otyper.modify(|_, w| w.ot12().variant(out_type)),
					13 => gpio.otyper.modify(|_, w| w.ot13().variant(out_type)),
					14 => gpio.otyper.modify(|_, w| w.ot14().variant(out_type)),
					15 => gpio.otyper.modify(|_, w| w.ot15().variant(out_type)),
					_ => ()
				}
			},
		}
	});
}

#[allow(dead_code)]
fn set_pin_mode(port: GpioPort, pin: u8, mode: GpioMode) {
	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpioa::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpioa::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpioa::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpioa::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpiob::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpiob::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpiob::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpiob::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpioc::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpioc::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpioc::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpioc::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpioc::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpioc::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpioc::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpioc::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpioc::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpioc::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpioc::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpioc::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpioc::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpioc::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpioc::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpioc::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpioc::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpioc::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpioc::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpioc::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let mode = match mode {
					GpioMode::Input => stm32f303::gpioc::moder::MODER0_A::INPUT,
					GpioMode::Output => stm32f303::gpioc::moder::MODER0_A::OUTPUT,
					GpioMode::AltFunc => stm32f303::gpioc::moder::MODER0_A::ALTERNATE,
					GpioMode::Analog => stm32f303::gpioc::moder::MODER0_A::ANALOG
				};
				match pin {
					0 => gpio.moder.modify(|_, w| w.moder0().variant(mode)),
					1 => gpio.moder.modify(|_, w| w.moder1().variant(mode)),
					2 => gpio.moder.modify(|_, w| w.moder2().variant(mode)),
					3 => gpio.moder.modify(|_, w| w.moder3().variant(mode)),
					4 => gpio.moder.modify(|_, w| w.moder4().variant(mode)),
					5 => gpio.moder.modify(|_, w| w.moder5().variant(mode)),
					6 => gpio.moder.modify(|_, w| w.moder6().variant(mode)),
					7 => gpio.moder.modify(|_, w| w.moder7().variant(mode)),
					8 => gpio.moder.modify(|_, w| w.moder8().variant(mode)),
					9 => gpio.moder.modify(|_, w| w.moder9().variant(mode)),
					10 => gpio.moder.modify(|_, w| w.moder10().variant(mode)),
					11 => gpio.moder.modify(|_, w| w.moder11().variant(mode)),
					12 => gpio.moder.modify(|_, w| w.moder12().variant(mode)),
					13 => gpio.moder.modify(|_, w| w.moder13().variant(mode)),
					14 => gpio.moder.modify(|_, w| w.moder14().variant(mode)),
					15 => gpio.moder.modify(|_, w| w.moder15().variant(mode)),
					_ => ()
				}
			},
		}
	});
}

#[allow(dead_code)]
fn set_pin_pull(port: GpioPort, pin: u8, pull: PinPull) {
	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpioa::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpioa::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpioa::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpiob::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpiob::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpiob::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpioc::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpioc::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpioc::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpioc::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpioc::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpioc::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpioc::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpioc::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpioc::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpioc::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpioc::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpioc::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpioc::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpioc::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpioc::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				let pull = match pull {
					PinPull::NoPull => stm32f303::gpioc::pupdr::PUPDR0_A::FLOATING,
					PinPull::PullUp => stm32f303::gpioc::pupdr::PUPDR0_A::PULLUP,
					PinPull::PullDown => stm32f303::gpioc::pupdr::PUPDR0_A::PULLDOWN,
				};
				match pin {
					0 => gpio.pupdr.modify(|_, w| w.pupdr0().variant(pull)),
					1 => gpio.pupdr.modify(|_, w| w.pupdr1().variant(pull)),
					2 => gpio.pupdr.modify(|_, w| w.pupdr2().variant(pull)),
					3 => gpio.pupdr.modify(|_, w| w.pupdr3().variant(pull)),
					4 => gpio.pupdr.modify(|_, w| w.pupdr4().variant(pull)),
					5 => gpio.pupdr.modify(|_, w| w.pupdr5().variant(pull)),
					6 => gpio.pupdr.modify(|_, w| w.pupdr6().variant(pull)),
					7 => gpio.pupdr.modify(|_, w| w.pupdr7().variant(pull)),
					8 => gpio.pupdr.modify(|_, w| w.pupdr8().variant(pull)),
					9 => gpio.pupdr.modify(|_, w| w.pupdr9().variant(pull)),
					10 => gpio.pupdr.modify(|_, w| w.pupdr10().variant(pull)),
					11 => gpio.pupdr.modify(|_, w| w.pupdr11().variant(pull)),
					12 => gpio.pupdr.modify(|_, w| w.pupdr12().variant(pull)),
					13 => gpio.pupdr.modify(|_, w| w.pupdr13().variant(pull)),
					14 => gpio.pupdr.modify(|_, w| w.pupdr14().variant(pull)),
					15 => gpio.pupdr.modify(|_, w| w.pupdr15().variant(pull)),
					_ => ()
				}
			},
		}
	});
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}