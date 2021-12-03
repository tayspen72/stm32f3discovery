//==============================================================================
// Notes
//==============================================================================
// mcu/flash.rs

/*
 * Flash is arranged in 128 pages of 2k bytes: 256K total
 * 128k at address 0x0000_0000	- 	0x0003_FFFF
 * 128k at address 0x0800_0000 	- 	0x0803_FFFF
 */

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
static FLASH_HANDLE: Mutex<RefCell<Option<stm32f303::FLASH>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(flash: stm32f303::FLASH) {
	free(|cs| FLASH_HANDLE.borrow(cs).replace(Some(flash)));
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}