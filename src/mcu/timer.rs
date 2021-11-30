//==============================================================================
// Notes
//==============================================================================
// mcu/timer.rs

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
// Advanced Control Timers
static TIM1_HANDLE: Mutex<RefCell<Option<stm32f303::TIM1>>> = 
	Mutex::new(RefCell::new(None));
static TIM8_HANDLE: Mutex<RefCell<Option<stm32f303::TIM8>>> = 
	Mutex::new(RefCell::new(None));

// Basic Timers
static TIM6_HANDLE: Mutex<RefCell<Option<stm32f303::TIM6>>> = 
	Mutex::new(RefCell::new(None));
static TIM7_HANDLE: Mutex<RefCell<Option<stm32f303::TIM7>>> = 
	Mutex::new(RefCell::new(None));

// General Purpose Timers
static TIM2_HANDLE: Mutex<RefCell<Option<stm32f303::TIM2>>> = 
	Mutex::new(RefCell::new(None));
static TIM3_HANDLE: Mutex<RefCell<Option<stm32f303::TIM3>>> = 
	Mutex::new(RefCell::new(None));
static TIM4_HANDLE: Mutex<RefCell<Option<stm32f303::TIM4>>> = 
	Mutex::new(RefCell::new(None));

// General Purpose Timers
static TIM15_HANDLE: Mutex<RefCell<Option<stm32f303::TIM15>>> = 
	Mutex::new(RefCell::new(None));
static TIM16_HANDLE: Mutex<RefCell<Option<stm32f303::TIM16>>> = 
	Mutex::new(RefCell::new(None));
static TIM17_HANDLE: Mutex<RefCell<Option<stm32f303::TIM17>>> = 
	Mutex::new(RefCell::new(None));


//==============================================================================
// Public Functions
//==============================================================================
pub fn init(
	tim1: stm32f303::TIM1,
	tim8: stm32f303::TIM8,
	tim6: stm32f303::TIM6,
	tim7: stm32f303::TIM7,
	tim2: stm32f303::TIM2,
	tim3: stm32f303::TIM3,
	tim4: stm32f303::TIM4,
	tim15: stm32f303::TIM15,
	tim16: stm32f303::TIM16,
	tim17: stm32f303::TIM17) {
	
	free(|cs| TIM1_HANDLE.borrow(cs).replace(Some(tim1)));
	free(|cs| TIM8_HANDLE.borrow(cs).replace(Some(tim8)));
	free(|cs| TIM6_HANDLE.borrow(cs).replace(Some(tim6)));
	free(|cs| TIM7_HANDLE.borrow(cs).replace(Some(tim7)));
	free(|cs| TIM2_HANDLE.borrow(cs).replace(Some(tim2)));
	free(|cs| TIM3_HANDLE.borrow(cs).replace(Some(tim3)));
	free(|cs| TIM4_HANDLE.borrow(cs).replace(Some(tim4)));
	free(|cs| TIM15_HANDLE.borrow(cs).replace(Some(tim15)));
	free(|cs| TIM16_HANDLE.borrow(cs).replace(Some(tim16)));
	free(|cs| TIM17_HANDLE.borrow(cs).replace(Some(tim17)));
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}