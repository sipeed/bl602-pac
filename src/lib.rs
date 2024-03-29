#![doc = "Peripheral access API for BL602 microcontrollers (generated using svd2rust v0.28.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 0] = [];
#[doc = "Global register"]
pub struct GLB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GLB {}
impl GLB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const glb::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const glb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GLB {
    type Target = glb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GLB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLB").finish()
    }
}
#[doc = "Global register"]
pub mod glb;
#[doc = "RF control"]
pub struct RF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RF {}
impl RF {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rf::RegisterBlock = 0x4000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rf::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RF {
    type Target = rf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF").finish()
    }
}
#[doc = "RF control"]
pub mod rf;
#[doc = "Universal DAC/ADC/ACOMP interface control"]
pub struct GPIP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIP {}
impl GPIP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpip::RegisterBlock = 0x4000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpip::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIP {
    type Target = gpip::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIP").finish()
    }
}
#[doc = "Universal DAC/ADC/ACOMP interface control"]
pub mod gpip;
#[doc = "SEC_DBG."]
pub struct SEC_DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_DBG {}
impl SEC_DBG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sec_dbg::RegisterBlock = 0x4000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sec_dbg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SEC_DBG {
    type Target = sec_dbg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SEC_DBG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_DBG").finish()
    }
}
#[doc = "SEC_DBG."]
pub mod sec_dbg;
#[doc = "SEC_ENG."]
pub struct SEC_ENG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_ENG {}
impl SEC_ENG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sec_eng::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sec_eng::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SEC_ENG {
    type Target = sec_eng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SEC_ENG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_ENG").finish()
    }
}
#[doc = "SEC_ENG."]
pub mod sec_eng;
#[doc = "TZC_SEC."]
pub struct TZC_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TZC_SEC {}
impl TZC_SEC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tzc_sec::RegisterBlock = 0x4000_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tzc_sec::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TZC_SEC {
    type Target = tzc_sec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TZC_SEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_SEC").finish()
    }
}
#[doc = "TZC_SEC."]
pub mod tzc_sec;
#[doc = "TZC_NSEC."]
pub struct TZC_NSEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TZC_NSEC {}
impl TZC_NSEC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tzc_nsec::RegisterBlock = 0x4000_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tzc_nsec::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TZC_NSEC {
    type Target = tzc_nsec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TZC_NSEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_NSEC").finish()
    }
}
#[doc = "TZC_NSEC."]
pub mod tzc_nsec;
#[doc = "EF_DATA_0"]
pub struct EF_DATA_0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EF_DATA_0 {}
impl EF_DATA_0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ef_data_0::RegisterBlock = 0x4000_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ef_data_0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EF_DATA_0 {
    type Target = ef_data_0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EF_DATA_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EF_DATA_0").finish()
    }
}
#[doc = "EF_DATA_0"]
pub mod ef_data_0;
#[doc = "EF_DATA_1."]
pub struct EF_DATA_1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EF_DATA_1 {}
impl EF_DATA_1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ef_data_1::RegisterBlock = 0x4000_7080 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ef_data_1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EF_DATA_1 {
    type Target = ef_data_1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EF_DATA_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EF_DATA_1").finish()
    }
}
#[doc = "EF_DATA_1."]
pub mod ef_data_1;
#[doc = "eFuse memory control"]
pub struct EF_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EF_CTRL {}
impl EF_CTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ef_ctrl::RegisterBlock = 0x4000_7800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ef_ctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EF_CTRL {
    type Target = ef_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EF_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EF_CTRL").finish()
    }
}
#[doc = "eFuse memory control"]
pub mod ef_ctrl;
#[doc = "CCI."]
pub struct CCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCI {}
impl CCI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cci::RegisterBlock = 0x4000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cci::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CCI {
    type Target = cci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CCI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCI").finish()
    }
}
#[doc = "CCI."]
pub mod cci;
#[doc = "Cache control"]
pub struct L1C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for L1C {}
impl L1C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const l1c::RegisterBlock = 0x4000_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const l1c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for L1C {
    type Target = l1c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for L1C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1C").finish()
    }
}
#[doc = "Cache control"]
pub mod l1c;
#[doc = "UART0 control"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x4000_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "UART0 control"]
pub mod uart0;
#[doc = "UART1 control"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x4000_a100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "UART1 control"]
pub use self::uart0 as uart1;
#[doc = "SPI master / slave control"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0x4000_a200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI").finish()
    }
}
#[doc = "SPI master / slave control"]
pub mod spi;
#[doc = "I2C control"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0x4000_a300 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C").finish()
    }
}
#[doc = "I2C control"]
pub mod i2c;
#[doc = "Pulse width modulation control"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm::RegisterBlock = 0x4000_a400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM").finish()
    }
}
#[doc = "Pulse width modulation control"]
pub mod pwm;
#[doc = "Timer control"]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer::RegisterBlock = 0x4000_a500 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER").finish()
    }
}
#[doc = "Timer control"]
pub mod timer;
#[doc = "Infrared remote control"]
pub struct IR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IR {}
impl IR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ir::RegisterBlock = 0x4000_a600 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ir::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IR {
    type Target = ir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR").finish()
    }
}
#[doc = "Infrared remote control"]
pub mod ir;
#[doc = "Checksum engine"]
pub struct CKS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CKS {}
impl CKS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cks::RegisterBlock = 0x4000_a700 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cks::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CKS {
    type Target = cks::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CKS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKS").finish()
    }
}
#[doc = "Checksum engine"]
pub mod cks;
#[doc = "Flash control"]
pub struct SF_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SF_CTRL {}
impl SF_CTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sf_ctrl::RegisterBlock = 0x4000_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sf_ctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SF_CTRL {
    type Target = sf_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SF_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SF_CTRL").finish()
    }
}
#[doc = "Flash control"]
pub mod sf_ctrl;
#[doc = "DMA control"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma::RegisterBlock = 0x4000_c000 as *const _;
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
#[doc = "DMA control"]
pub mod dma;
#[doc = "Sleep control (power-down sleep)"]
pub struct PDS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDS {}
impl PDS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pds::RegisterBlock = 0x4000_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pds::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PDS {
    type Target = pds::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDS").finish()
    }
}
#[doc = "Sleep control (power-down sleep)"]
pub mod pds;
#[doc = "Deep Sleep Control (Hibernation)"]
pub struct HBN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HBN {}
impl HBN {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hbn::RegisterBlock = 0x4000_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hbn::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HBN {
    type Target = hbn::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HBN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HBN").finish()
    }
}
#[doc = "Deep Sleep Control (Hibernation)"]
pub mod hbn;
#[doc = "Always-ON periherals"]
pub struct AON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON {}
impl AON {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aon::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AON {
    type Target = aon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AON {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AON").finish()
    }
}
#[doc = "Always-ON periherals"]
pub mod aon;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "GLB"]
    pub GLB: GLB,
    #[doc = "RF"]
    pub RF: RF,
    #[doc = "GPIP"]
    pub GPIP: GPIP,
    #[doc = "SEC_DBG"]
    pub SEC_DBG: SEC_DBG,
    #[doc = "SEC_ENG"]
    pub SEC_ENG: SEC_ENG,
    #[doc = "TZC_SEC"]
    pub TZC_SEC: TZC_SEC,
    #[doc = "TZC_NSEC"]
    pub TZC_NSEC: TZC_NSEC,
    #[doc = "EF_DATA_0"]
    pub EF_DATA_0: EF_DATA_0,
    #[doc = "EF_DATA_1"]
    pub EF_DATA_1: EF_DATA_1,
    #[doc = "EF_CTRL"]
    pub EF_CTRL: EF_CTRL,
    #[doc = "CCI"]
    pub CCI: CCI,
    #[doc = "L1C"]
    pub L1C: L1C,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
    #[doc = "IR"]
    pub IR: IR,
    #[doc = "CKS"]
    pub CKS: CKS,
    #[doc = "SF_CTRL"]
    pub SF_CTRL: SF_CTRL,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "PDS"]
    pub PDS: PDS,
    #[doc = "HBN"]
    pub HBN: HBN,
    #[doc = "AON"]
    pub AON: AON,
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
            GLB: GLB {
                _marker: PhantomData,
            },
            RF: RF {
                _marker: PhantomData,
            },
            GPIP: GPIP {
                _marker: PhantomData,
            },
            SEC_DBG: SEC_DBG {
                _marker: PhantomData,
            },
            SEC_ENG: SEC_ENG {
                _marker: PhantomData,
            },
            TZC_SEC: TZC_SEC {
                _marker: PhantomData,
            },
            TZC_NSEC: TZC_NSEC {
                _marker: PhantomData,
            },
            EF_DATA_0: EF_DATA_0 {
                _marker: PhantomData,
            },
            EF_DATA_1: EF_DATA_1 {
                _marker: PhantomData,
            },
            EF_CTRL: EF_CTRL {
                _marker: PhantomData,
            },
            CCI: CCI {
                _marker: PhantomData,
            },
            L1C: L1C {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
            IR: IR {
                _marker: PhantomData,
            },
            CKS: CKS {
                _marker: PhantomData,
            },
            SF_CTRL: SF_CTRL {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            PDS: PDS {
                _marker: PhantomData,
            },
            HBN: HBN {
                _marker: PhantomData,
            },
            AON: AON {
                _marker: PhantomData,
            },
        }
    }
}
