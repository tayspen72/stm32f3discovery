//==============================================================================
// Notes
//==============================================================================
// mcu/mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use stm32f3::stm32f303;

pub mod adc;
pub mod clocks;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod rtc;
pub mod spi;
pub mod timer;
pub mod uart;
pub mod wdt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	let peripherals = stm32f303::Peripherals::take().unwrap();

	clocks::init(
		peripherals.RCC
	);
	
	adc::init(
		peripherals.ADC1,
		peripherals.ADC2,
		peripherals.ADC3,
		peripherals.ADC4
	);
	flash::init(
		peripherals.FLASH
	);
	gpio::init(
		peripherals.GPIOA,
		peripherals.GPIOB,
		peripherals.GPIOC,
		peripherals.GPIOD,
		peripherals.GPIOE,
		peripherals.GPIOF,
		peripherals.GPIOG,
		peripherals.GPIOH
	);
	i2c::init(
		peripherals.I2C1,
		peripherals.I2C2,
		peripherals.I2C3
	);
	rtc::init(
		peripherals.RTC
	);
	spi::init(
		peripherals.SPI1,
		peripherals.SPI2,
		peripherals.SPI3,
		peripherals.SPI4
	);
	timer::init(
		peripherals.TIM1,
		peripherals.TIM8,
		peripherals.TIM6,
		peripherals.TIM7,
		peripherals.TIM2,
		peripherals.TIM3,
		peripherals.TIM4,
		peripherals.TIM15,
		peripherals.TIM16,
		peripherals.TIM17
	);
	uart::init(
		peripherals.USART1,
		peripherals.USART2,
		peripherals.USART3
	);
	wdt::init(
		peripherals.IWDG
	);
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
	adc::task_handler();
	clocks::task_handler();
	flash::task_handler();
	gpio::task_handler();
	i2c::task_handler();
	rtc::task_handler();
	spi::task_handler();
	timer::task_handler();
	uart::task_handler();
	wdt::task_handler();
}
