//==============================================================================
// Notes
//==============================================================================
// mcu/adc.rs

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
static ADC1_HANDLE: Mutex<RefCell<Option<stm32f303::ADC1>>> = 
	Mutex::new(RefCell::new(None));
static ADC2_HANDLE: Mutex<RefCell<Option<stm32f303::ADC2>>> = 
	Mutex::new(RefCell::new(None));
static ADC3_HANDLE: Mutex<RefCell<Option<stm32f303::ADC3>>> = 
	Mutex::new(RefCell::new(None));
static ADC4_HANDLE: Mutex<RefCell<Option<stm32f303::ADC4>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(
	adc1: stm32f303::ADC1,
	adc2: stm32f303::ADC2,
	adc3: stm32f303::ADC3,
	adc4: stm32f303::ADC4) {
	
	free(|cs| ADC1_HANDLE.borrow(cs).replace(Some(adc1)));
	free(|cs| ADC2_HANDLE.borrow(cs).replace(Some(adc2)));
	free(|cs| ADC3_HANDLE.borrow(cs).replace(Some(adc3)));
	free(|cs| ADC4_HANDLE.borrow(cs).replace(Some(adc4)));
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}