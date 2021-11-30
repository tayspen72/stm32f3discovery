//==============================================================================
// Notes
//==============================================================================
// mcu/spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
// use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use stm32f3::stm32f303;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static SPI1_HANDLE: Mutex<RefCell<Option<stm32f303::SPI1>>> = 
	Mutex::new(RefCell::new(None));
static SPI2_HANDLE: Mutex<RefCell<Option<stm32f303::SPI2>>> = 
	Mutex::new(RefCell::new(None));
static SPI3_HANDLE: Mutex<RefCell<Option<stm32f303::SPI3>>> = 
	Mutex::new(RefCell::new(None));
static SPI4_HANDLE: Mutex<RefCell<Option<stm32f303::SPI4>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(
	spi1: stm32f303::SPI1,
	spi2: stm32f303::SPI2,
	spi3: stm32f303::SPI3,
	spi4: stm32f303::SPI4) {
	
	free(|cs| SPI1_HANDLE.borrow(cs).replace(Some(spi1)));
	free(|cs| SPI2_HANDLE.borrow(cs).replace(Some(spi2)));
	free(|cs| SPI3_HANDLE.borrow(cs).replace(Some(spi3)));
	free(|cs| SPI4_HANDLE.borrow(cs).replace(Some(spi4)));
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}