#![doc = "Peripheral access API for STM32G070 microcontrollers (generated using svd2rust v0.28.0 (27eebb4 2022-12-25))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC_STAMP();
    fn FLASH();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn DMA_CHANNEL1();
    fn DMA_CHANNEL2_3();
    fn DMA_CHANNEL4_5_6_7();
    fn ADC();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM3();
    fn TIM6();
    fn TIM7();
    fn TIM14();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn I2C2();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3_USART4();
    fn CEC();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: RTC_STAMP,
    },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA_CHANNEL1,
    },
    Vector {
        _handler: DMA_CHANNEL2_3,
    },
    Vector {
        _handler: DMA_CHANNEL4_5_6_7,
    },
    Vector { _handler: ADC },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _reserved: 0 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector { _handler: TIM14 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector {
        _handler: USART3_USART4,
    },
    Vector { _handler: CEC },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - Window watchdog interrupt"]
    WWDG = 0,
    #[doc = "1 - Power voltage detector interrupt"]
    PVD = 1,
    #[doc = "2 - RTC and TAMP interrupts"]
    RTC_STAMP = 2,
    #[doc = "3 - Flash global interrupt"]
    FLASH = 3,
    #[doc = "4 - RCC global interrupt"]
    RCC = 4,
    #[doc = "5 - EXTI line 0 & 1 interrupt"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI line 2 & 3 interrupt"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI line 4 to 15 interrupt"]
    EXTI4_15 = 7,
    #[doc = "9 - DMA channel 1 interrupt"]
    DMA_CHANNEL1 = 9,
    #[doc = "10 - DMA channel 2 & 3 interrupts"]
    DMA_CHANNEL2_3 = 10,
    #[doc = "11 - DMA channel 4, 5, 6 & 7 and DMAMUX"]
    DMA_CHANNEL4_5_6_7 = 11,
    #[doc = "12 - ADC Interrupt"]
    ADC = 12,
    #[doc = "13 - TIM1 break, update, trigger and commutation interrupts"]
    TIM1_BRK_UP_TRG_COM = 13,
    #[doc = "14 - TIM1 Capture Compare interrupt"]
    TIM1_CC = 14,
    #[doc = "16 - TIM3 global interrupt"]
    TIM3 = 16,
    #[doc = "17 - TIM6 global interrupt"]
    TIM6 = 17,
    #[doc = "18 - TIM7 global interrupt"]
    TIM7 = 18,
    #[doc = "19 - TIM14 global interrupt"]
    TIM14 = 19,
    #[doc = "20 - Timer 15 global interrupt"]
    TIM15 = 20,
    #[doc = "21 - TIM16 global interrupt"]
    TIM16 = 21,
    #[doc = "22 - TIM17 global interrupt"]
    TIM17 = 22,
    #[doc = "23 - I2C1 global interrupt"]
    I2C1 = 23,
    #[doc = "24 - I2C2 global interrupt"]
    I2C2 = 24,
    #[doc = "25 - SPI1 global interrupt"]
    SPI1 = 25,
    #[doc = "26 - SPI2 global interrupt"]
    SPI2 = 26,
    #[doc = "27 - USART1 global interrupt"]
    USART1 = 27,
    #[doc = "28 - USART2 global interrupt"]
    USART2 = 28,
    #[doc = "29 - USART3 + USART4 interrupt"]
    USART3_USART4 = 29,
    #[doc = "30 - CEC global interrupt"]
    CEC = 30,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const iwdg::RegisterBlock = 0x4000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IWDG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG").finish()
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "System window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wwdg::RegisterBlock = 0x4000_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WWDG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDG").finish()
    }
}
#[doc = "System window watchdog"]
pub mod wwdg;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flash::RegisterBlock = 0x4002_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FLASH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH").finish()
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Debug support"]
pub struct DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBG {}
impl DBG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dbg::RegisterBlock = 0x4001_5800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DBG {
    type Target = dbg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DBG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG").finish()
    }
}
#[doc = "Debug support"]
pub mod dbg;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rcc::RegisterBlock = 0x4002_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC").finish()
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwr::RegisterBlock = 0x4000_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR").finish()
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "DMA controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma::RegisterBlock = 0x4002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA").finish()
    }
}
#[doc = "DMA controller"]
pub mod dma;
#[doc = "DMAMUX"]
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dmamux::RegisterBlock = 0x4002_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMAMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX").finish()
    }
}
#[doc = "DMAMUX"]
pub mod dmamux;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioa::RegisterBlock = 0x5000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOA").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x5000_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x5000_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOC {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOC").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub use self::gpiob as gpioc;
#[doc = "General-purpose I/Os"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x5000_0c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOD {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOD").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub use self::gpiob as gpiod;
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x5000_1400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOF {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOF").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub use self::gpiob as gpiof;
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x4002_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const exti::RegisterBlock = 0x4002_1800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EXTI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI").finish()
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "General purpose timers"]
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim15::RegisterBlock = 0x4001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim15::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15").finish()
    }
}
#[doc = "General purpose timers"]
pub mod tim15;
#[doc = "General purpose timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim16::RegisterBlock = 0x4001_4400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16").finish()
    }
}
#[doc = "General purpose timers"]
pub mod tim16;
#[doc = "General purpose timers"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim16::RegisterBlock = 0x4001_4800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM17").finish()
    }
}
#[doc = "General purpose timers"]
pub use self::tim16 as tim17;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4001_3800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1").finish()
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_4400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART2").finish()
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use self::usart1 as usart2;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_4c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART4").finish()
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use self::usart1 as usart4;
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x4001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub mod spi1;
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x4000_3800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub use self::spi1 as spi2;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim1::RegisterBlock = 0x4001_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1").finish()
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Analog to Digital Converter instance 1"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4001_2400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "Analog to Digital Converter instance 1"]
pub mod adc;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const syscfg::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG").finish()
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Tamper and backup registers"]
pub struct TAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAMP {}
impl TAMP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tamp::RegisterBlock = 0x4000_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tamp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TAMP {
    type Target = tamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TAMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP").finish()
    }
}
#[doc = "Tamper and backup registers"]
pub mod tamp;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c1::RegisterBlock = 0x4000_5400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1").finish()
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Inter-integrated circuit"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c1::RegisterBlock = 0x4000_5800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C2").finish()
    }
}
#[doc = "Inter-integrated circuit"]
pub use self::i2c1 as i2c2;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x4000_2800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "General purpose timers"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim14::RegisterBlock = 0x4000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim14::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM14 {
    type Target = tim14::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM14").finish()
    }
}
#[doc = "General purpose timers"]
pub mod tim14;
#[doc = "Basic timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim6::RegisterBlock = 0x4000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM6").finish()
    }
}
#[doc = "Basic timers"]
pub mod tim6;
#[doc = "General-purpose-timers"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim3::RegisterBlock = 0x4000_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM3 {
    type Target = tim3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3").finish()
    }
}
#[doc = "General-purpose-timers"]
pub mod tim3;
#[doc = "SysTick timer"]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const stk::RegisterBlock = 0xe000_e010 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stk::RegisterBlock {
        Self::PTR
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for STK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STK").finish()
    }
}
#[doc = "SysTick timer"]
pub mod stk;
#[doc = "Basic timers"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim6::RegisterBlock = 0x4000_1400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM7").finish()
    }
}
#[doc = "Basic timers"]
pub use self::tim6 as tim7;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_4800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART3").finish()
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use self::usart1 as usart3;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "DBG"]
    pub DBG: DBG,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "TAMP"]
    pub TAMP: TAMP,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "USART3"]
    pub USART3: USART3,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            DBG: DBG {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART4: USART4 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            TAMP: TAMP {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            TIM14: TIM14 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
        }
    }
}
