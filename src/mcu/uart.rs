//==============================================================================
// Notes
//==============================================================================
// mcu/uart.rs

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
static UART1_HANDLE: Mutex<RefCell<Option<stm32f303::USART1>>> = 
	Mutex::new(RefCell::new(None));
static UART2_HANDLE: Mutex<RefCell<Option<stm32f303::USART2>>> = 
	Mutex::new(RefCell::new(None));
static UART3_HANDLE: Mutex<RefCell<Option<stm32f303::USART3>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(
	uart1: stm32f303::USART1,
	uart2: stm32f303::USART2,
	uart3: stm32f303::USART3) {

	free(|cs| UART1_HANDLE.borrow(cs).replace(Some(uart1)));
	free(|cs| UART2_HANDLE.borrow(cs).replace(Some(uart2)));
	free(|cs| UART3_HANDLE.borrow(cs).replace(Some(uart3)));
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}