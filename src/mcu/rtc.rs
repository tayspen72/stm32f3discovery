//==============================================================================
// Notes
//==============================================================================
// mcu/rtc.rs

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
static RTC_HANDLE: Mutex<RefCell<Option<stm32f303::RTC>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(
	rtc: stm32f303::RTC) {
	
	free(|cs| RTC_HANDLE.borrow(cs).replace(Some(rtc)));
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}