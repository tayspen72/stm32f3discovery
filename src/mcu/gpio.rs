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
#[derive(Clone, Copy)]
pub enum PinState {
	PinLow,
	PinHigh
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
pub fn get_pin_state(port: GpioPort, pin: u8) -> PinState {
	let read: u32 = free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.idr.read().bits()
			}
			else {
				0
			},
		}
	});

	if read & (1 << pin) > 0 {
		PinState::PinHigh
	}
	else {
		PinState::PinLow
	}
}

#[allow(dead_code)]
pub fn pin_setup(port: GpioPort, pin: u8, mode: GpioMode) {
	set_pin_mode(port, pin, mode);
}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn set_output_type(port: GpioPort, pin: u8, out_type: OutputType) {
	let mask: u32 = !(0x1 << pin);
	let value: u32 = match out_type {
		OutputType::PushPull => stm32f303::gpioa::otyper::OT0_A::PUSHPULL as u32,
		OutputType::OpenDrain => stm32f303::gpioa::otyper::OT0_A::OPENDRAIN as u32,
	};
	let value = !(value << pin);
	
	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.otyper.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			}
		}
	});
}

#[allow(dead_code)]
fn set_pin_mode(port: GpioPort, pin: u8, mode: GpioMode) {
	let mask: u32 = !(0x3 << (pin * 2));
	let value: u32 = match mode {
		GpioMode::Input => stm32f303::gpioa::moder::MODER0_A::INPUT as u32,
		GpioMode::Output => stm32f303::gpioa::moder::MODER0_A::OUTPUT as u32,
		GpioMode::AltFunc => stm32f303::gpioa::moder::MODER0_A::ALTERNATE as u32,
		GpioMode::Analog => stm32f303::gpioa::moder::MODER0_A::ANALOG as u32
	};
	let value = !(value << (pin * 2));
	
	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.moder.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			}
		}
	});
}

#[allow(dead_code)]
fn set_pin_pull(port: GpioPort, pin: u8, pull: PinPull) {
	let mask: u32 = !(0x3 << (pin * 2));
	let value: u32 = match pull {
		PinPull::NoPull => stm32f303::gpioa::pupdr::PUPDR0_A::FLOATING as u32,
		PinPull::PullUp => stm32f303::gpioa::pupdr::PUPDR0_A::PULLUP as u32,
		PinPull::PullDown => stm32f303::gpioa::pupdr::PUPDR0_A::PULLDOWN as u32
	};
	let value = !(value << (pin * 2));
	
	free(|cs| {
		match port {
			GpioPort::PortA => if let Some(ref mut gpio) = GPIOA_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortB => if let Some(ref mut gpio) = GPIOB_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortC => if let Some(ref mut gpio) = GPIOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortD => if let Some(ref mut gpio) = GPIOD_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortE => if let Some(ref mut gpio) = GPIOE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortF => if let Some(ref mut gpio) = GPIOF_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortG => if let Some(ref mut gpio) = GPIOG_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			},
			GpioPort::PortH => if let Some(ref mut gpio) = GPIOH_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				gpio.pupdr.modify(|r, w| unsafe { w.bits((r.bits() & mask) | value) });
			}
		}
	});
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}
