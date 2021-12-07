//==============================================================================
// Notes
//==============================================================================
// mcu/clocks.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use stm32f3::stm32f303;

use crate::config;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum AhbPeripherals {
	DMA1 = 		0x0000_0001,
	DMA2 = 		0x0000_0002,
	SRAM = 		0x0000_0004,
	FLITF = 	0x0000_0010,
	FMC = 		0x0000_0020,
	CRC = 		0x0000_0040,
	IOPH = 		0x0001_0000,
	IOPA = 		0x0002_0000,
	IOPB = 		0x0004_0000,
	IOPC = 		0x0008_0000,
	IOPD = 		0x0010_0000,
	IOPE = 		0x0020_0000,
	IOPF = 		0x0040_0000,
	IOPG = 		0x0080_0000,
	TSC = 		0x0100_0000,
	ADC12 =		0x1000_0000,
	ADC34 =		0x2000_0000,
}

#[allow(dead_code)]
pub enum Apb1Peripherals {
	TIM2 = 		0x0000_0001,
	TIM3 = 		0x0000_0002,
	TIM4 = 		0x0000_0004,
	TIM6 = 		0x0000_0010,
	TIM7 = 		0x0000_0020,
	WWD = 		0x0000_0800,
	SPI2 = 		0x0000_4000,
	SPI3 = 		0x0000_8000,
	USART2 = 	0x0002_0000,
	USART3 = 	0x0004_0000,
	USART4 = 	0x0008_0000,
	USART5 = 	0x0010_0000,
	I2C1 = 		0x0020_0000,
	I1C2 = 		0x0040_0000,
	USB = 		0x0080_0000,
	CAN = 		0x0200_0000,
	DAC2 = 		0x0400_0000,
	PWR = 		0x1000_0000,
	DAC1 = 		0x2000_0000,
	I2C3 = 		0x4000_0000,
}

#[allow(dead_code)]
pub enum Apb2Peripherals {
	SYSCFG = 	0x0000_0001,
	TIM1 = 		0x0000_0800,
	SPI1 = 		0x0000_1000,
	TIM8 = 		0x0000_2000,
	USART1 = 	0x0000_4000,
	SPI4 = 		0x0000_8000,
	TIM15 = 	0x0001_0000,
	TIM16 = 	0x0002_0000,
	TIM17 = 	0x0004_0000,
	TIM20 = 	0x0010_0000,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Clocks {
	sysclk: u32,
	hclk: u32,
	lclk: u32,
	pclk1: u32,
	pclk2: u32,
}

//==============================================================================
// Variables
//==============================================================================
static RCC_HANDLE: Mutex<RefCell<Option<stm32f303::RCC>>> = 
	Mutex::new(RefCell::new(None));

static CLOCKS: Mutex<Cell<Clocks>> = Mutex::new(Cell::new(Clocks {
	sysclk: 0,
	hclk: 0,
	lclk: 0,
	pclk1: 0,
	pclk2: 0,
}));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(
	rcc: stm32f303::RCC) {
	
	init_clocks(&rcc);

	free(|cs| RCC_HANDLE.borrow(cs).replace(Some(rcc)));
}

#[allow(dead_code)]
pub fn get_clocks() -> Clocks {
	free(|cs| CLOCKS.borrow(cs).get())
}

#[allow(dead_code)]
pub fn set_ahb_peripheral_clock_enable(peripheral: AhbPeripherals, enable: bool) {
	free(|cs| if let Some(rcc) = RCC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
		if enable {
			rcc.ahbenr.modify(|r, w| unsafe { w.bits( r.bits() | peripheral as u32) });
		}
		else {
			rcc.ahbenr.modify(|r, w| unsafe { w.bits( r.bits() & !(peripheral as u32)) });
		}
	});
}

#[allow(dead_code)]
pub fn set_apb1_peripheral_clock_enable(peripheral: Apb1Peripherals, enable: bool) {
	free(|cs| if let Some(rcc) = RCC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
		if enable {
			rcc.ahbenr.modify(|r, w| unsafe { w.bits( r.bits() | peripheral as u32) });
		}
		else {
			rcc.ahbenr.modify(|r, w| unsafe { w.bits( r.bits() & !(peripheral as u32)) });
		}
	});
}

#[allow(dead_code)]
pub fn set_apb2_peripheral_clock_enable(peripheral: Apb2Peripherals, enable: bool) {
	free(|cs| if let Some(rcc) = RCC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
		if enable {
			rcc.ahbenr.modify(|r, w| unsafe { w.bits( r.bits() | peripheral as u32) });
		}
		else {
			rcc.ahbenr.modify(|r, w| unsafe { w.bits( r.bits() & !(peripheral as u32)) });
		}
	});
}

//==============================================================================
// Private Functions
//==============================================================================
fn init_clocks(
	rcc: &stm32f303::RCC) {
	// The main system clock can be sourced from the following:
	//   * HSI (High Speed Internal)
	//   * HSE (High Speed Extetnal)
	//   * PLL
	//   * LSI (Low Speed Internal 40kHz)
	//   * LSE (Low Speed External - 35.768kHz) 

	// This board has an 8MHz clock signal from the ST-Link MCU at 8MHz on HSE
	let (hse_on, hsi_on, sw) = if config::EXTERNAL_HIGH_SPEED { (
		stm32f303::rcc::cr::HSEON_A::ON,
		stm32f303::rcc::cr::HSION_A::OFF,
		stm32f303::rcc::cfgr::SW_A::HSE,
	) }
	else { (
		stm32f303::rcc::cr::HSEON_A::OFF,
		stm32f303::rcc::cr::HSION_A::ON,
		stm32f303::rcc::cfgr::SW_A::HSI,
	) };

	let (rtc_sel, lse_on) = if config::EXTERNAL_LOW_SPEED { (
		stm32f303::rcc::bdcr::RTCSEL_A::LSE,
		stm32f303::rcc::bdcr::LSEON_A::ON,
	) }
	else { (
		stm32f303::rcc::bdcr::RTCSEL_A::LSI,
		stm32f303::rcc::bdcr::LSEON_A::OFF,
	) };

	rcc.cr.write(|w| w
		.pllon().off()
		.hseon().variant(hse_on)
		.hsion().variant(hsi_on)
	);

	if config::EXTERNAL_HIGH_SPEED {
		while rcc.cr.read().hserdy().is_not_ready() {};
	}
	else {
		while rcc.cr.read().hsirdy().is_not_ready() {};
	}

	rcc.cfgr.write(|w| w
		.sw().variant(sw)
		.hpre().div4()
		.ppre1().div2()
		.ppre2().div2()
	);

	rcc.bdcr.write(|w| w
		.rtcen().enabled()
		.rtcsel().variant(rtc_sel)
		.lseon().variant(lse_on)
	);

	let clocks: Clocks = Clocks {
		sysclk: config::HIGH_SPEED_CLOCK,
		hclk: config::HIGH_SPEED_CLOCK / 4,
		lclk: config::LOW_SPEED_CLOCK,
		pclk1: config::HIGH_SPEED_CLOCK / 8,
		pclk2: config::HIGH_SPEED_CLOCK / 8,
	};

	free(|cs| CLOCKS.borrow(cs).set(clocks));
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}