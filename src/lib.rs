#![doc = "Peripheral access API for CCG2 microcontrollers (generated using svd2rust v0.20.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.20.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
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
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn PICU0();
    fn PICU1();
    fn PICU2();
    fn GANGEDPICU();
    fn USBPD0();
    fn WDT();
    fn SCB0();
    fn SCB1();
    fn SPCIF();
    fn USBPD1();
    fn TCPWM0();
    fn TCPWM1();
    fn TCPWM2();
    fn TCPWM3();
    fn TCPWM4();
    fn TCPWM5();
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
pub static __INTERRUPTS: [Vector; 16] = [
    Vector { _handler: PICU0 },
    Vector { _handler: PICU1 },
    Vector { _handler: PICU2 },
    Vector {
        _handler: GANGEDPICU,
    },
    Vector { _handler: USBPD0 },
    Vector { _handler: WDT },
    Vector { _handler: SCB0 },
    Vector { _handler: SCB1 },
    Vector { _handler: SPCIF },
    Vector { _handler: USBPD1 },
    Vector { _handler: TCPWM0 },
    Vector { _handler: TCPWM1 },
    Vector { _handler: TCPWM2 },
    Vector { _handler: TCPWM3 },
    Vector { _handler: TCPWM4 },
    Vector { _handler: TCPWM5 },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - PICU0"]
    PICU0 = 0,
    #[doc = "1 - PICU1"]
    PICU1 = 1,
    #[doc = "2 - PICU2"]
    PICU2 = 2,
    #[doc = "3 - GANGEDPICU"]
    GANGEDPICU = 3,
    #[doc = "4 - USBPD0"]
    USBPD0 = 4,
    #[doc = "5 - WDT"]
    WDT = 5,
    #[doc = "6 - SCB0"]
    SCB0 = 6,
    #[doc = "7 - SCB1"]
    SCB1 = 7,
    #[doc = "8 - SPCIF"]
    SPCIF = 8,
    #[doc = "9 - USBPD1"]
    USBPD1 = 9,
    #[doc = "10 - TCPWM0"]
    TCPWM0 = 10,
    #[doc = "11 - TCPWM1"]
    TCPWM1 = 11,
    #[doc = "12 - TCPWM2"]
    TCPWM2 = 12,
    #[doc = "13 - TCPWM3"]
    TCPWM3 = 13,
    #[doc = "14 - TCPWM4"]
    TCPWM4 = 14,
    #[doc = "15 - TCPWM5"]
    TCPWM5 = 15,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "PERI"]
pub struct PERI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PERI {}
impl PERI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const peri::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const peri::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PERI {
    type Target = peri::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PERI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI").finish()
    }
}
#[doc = "PERI"]
pub mod peri;
#[doc = "HSIOM"]
pub struct HSIOM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSIOM {}
impl HSIOM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hsiom::RegisterBlock = 0x4002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsiom::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HSIOM {
    type Target = hsiom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HSIOM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSIOM").finish()
    }
}
#[doc = "HSIOM"]
pub mod hsiom;
#[doc = "SRSSLT"]
pub struct SRSSLT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRSSLT {}
impl SRSSLT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const srsslt::RegisterBlock = 0x4003_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const srsslt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SRSSLT {
    type Target = srsslt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SRSSLT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRSSLT").finish()
    }
}
#[doc = "SRSSLT"]
pub mod srsslt;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x4004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "SCB0"]
pub struct SCB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB0 {}
impl SCB0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4005_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB0 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB0").finish()
    }
}
#[doc = "SCB0"]
pub mod scb0;
#[doc = "SCB1"]
pub struct SCB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB1 {}
impl SCB1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb1::RegisterBlock = 0x4006_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB1 {
    type Target = scb1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB1").finish()
    }
}
#[doc = "SCB1"]
pub mod scb1;
#[doc = "TCPWM"]
pub struct TCPWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCPWM {}
impl TCPWM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tcpwm::RegisterBlock = 0x4007_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcpwm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TCPWM {
    type Target = tcpwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TCPWM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCPWM").finish()
    }
}
#[doc = "TCPWM"]
pub mod tcpwm;
#[doc = "USBPD"]
pub struct USBPD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBPD {}
impl USBPD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usbpd::RegisterBlock = 0x4008_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbpd::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USBPD {
    type Target = usbpd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USBPD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBPD").finish()
    }
}
#[doc = "USBPD"]
pub mod usbpd;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PERI"]
    pub PERI: PERI,
    #[doc = "HSIOM"]
    pub HSIOM: HSIOM,
    #[doc = "SRSSLT"]
    pub SRSSLT: SRSSLT,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "SCB0"]
    pub SCB0: SCB0,
    #[doc = "SCB1"]
    pub SCB1: SCB1,
    #[doc = "TCPWM"]
    pub TCPWM: TCPWM,
    #[doc = "USBPD"]
    pub USBPD: USBPD,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PERI: PERI {
                _marker: PhantomData,
            },
            HSIOM: HSIOM {
                _marker: PhantomData,
            },
            SRSSLT: SRSSLT {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            SCB0: SCB0 {
                _marker: PhantomData,
            },
            SCB1: SCB1 {
                _marker: PhantomData,
            },
            TCPWM: TCPWM {
                _marker: PhantomData,
            },
            USBPD: USBPD {
                _marker: PhantomData,
            },
        }
    }
}
