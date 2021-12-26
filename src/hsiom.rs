#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSIOM_PRT0"]
    pub hsiom_prt0: HSIOM_PRT0,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - HSIOM_PRT1"]
    pub hsiom_prt1: HSIOM_PRT1,
    _reserved2: [u8; 0xfc],
    #[doc = "0x200 - HSIOM_PRT2"]
    pub hsiom_prt2: HSIOM_PRT2,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct HSIOM_PRT0 {
    #[doc = "0x00 - "]
    pub port_sel0: crate::Reg<self::hsiom_prt0::port_sel0::PORT_SEL0_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HSIOM_PRT0"]
pub mod hsiom_prt0;
#[doc = r"Register block"]
#[repr(C)]
pub struct HSIOM_PRT1 {
    #[doc = "0x00 - "]
    pub port_sel1: crate::Reg<self::hsiom_prt1::port_sel1::PORT_SEL1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HSIOM_PRT1"]
pub mod hsiom_prt1;
#[doc = r"Register block"]
#[repr(C)]
pub struct HSIOM_PRT2 {
    #[doc = "0x00 - "]
    pub port_sel2: crate::Reg<self::hsiom_prt2::port_sel2::PORT_SEL2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HSIOM_PRT2"]
pub mod hsiom_prt2;
