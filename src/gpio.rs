#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x4c - PRT0"]
    pub prt0: PRT0,
    _reserved1: [u8; 0xb4],
    #[doc = "0x100..0x14c - PRT1"]
    pub prt1: PRT1,
    _reserved2: [u8; 0xb4],
    #[doc = "0x200..0x24c - PRT2"]
    pub prt2: PRT2,
    _reserved3: [u8; 0x0db4],
    #[doc = "0x1000 - "]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1010 - "]
    pub dft_io_test: crate::Reg<dft_io_test::DFT_IO_TEST_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT0 {
    #[doc = "0x00 - "]
    pub dr: crate::Reg<self::prt0::dr::DR_SPEC>,
    #[doc = "0x04 - "]
    pub ps: crate::Reg<self::prt0::ps::PS_SPEC>,
    #[doc = "0x08 - "]
    pub pc: crate::Reg<self::prt0::pc::PC_SPEC>,
    #[doc = "0x0c - "]
    pub intr_cfg: crate::Reg<self::prt0::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x10 - "]
    pub intr: crate::Reg<self::prt0::intr::INTR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - "]
    pub pc2: crate::Reg<self::prt0::pc2::PC2_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - "]
    pub dr_set: crate::Reg<self::prt0::dr_set::DR_SET_SPEC>,
    #[doc = "0x44 - "]
    pub dr_clr: crate::Reg<self::prt0::dr_clr::DR_CLR_SPEC>,
    #[doc = "0x48 - "]
    pub dr_inv: crate::Reg<self::prt0::dr_inv::DR_INV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRT0"]
pub mod prt0;
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT1 {
    #[doc = "0x00 - "]
    pub dr: crate::Reg<self::prt1::dr::DR_SPEC>,
    #[doc = "0x04 - "]
    pub ps: crate::Reg<self::prt1::ps::PS_SPEC>,
    #[doc = "0x08 - "]
    pub pc: crate::Reg<self::prt1::pc::PC_SPEC>,
    #[doc = "0x0c - "]
    pub intr_cfg: crate::Reg<self::prt1::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x10 - "]
    pub intr: crate::Reg<self::prt1::intr::INTR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - "]
    pub pc2: crate::Reg<self::prt1::pc2::PC2_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - "]
    pub dr_set: crate::Reg<self::prt1::dr_set::DR_SET_SPEC>,
    #[doc = "0x44 - "]
    pub dr_clr: crate::Reg<self::prt1::dr_clr::DR_CLR_SPEC>,
    #[doc = "0x48 - "]
    pub dr_inv: crate::Reg<self::prt1::dr_inv::DR_INV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRT1"]
pub mod prt1;
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT2 {
    #[doc = "0x00 - "]
    pub dr: crate::Reg<self::prt2::dr::DR_SPEC>,
    #[doc = "0x04 - "]
    pub ps: crate::Reg<self::prt2::ps::PS_SPEC>,
    #[doc = "0x08 - "]
    pub pc: crate::Reg<self::prt2::pc::PC_SPEC>,
    #[doc = "0x0c - "]
    pub intr_cfg: crate::Reg<self::prt2::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x10 - "]
    pub intr: crate::Reg<self::prt2::intr::INTR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - "]
    pub pc2: crate::Reg<self::prt2::pc2::PC2_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - "]
    pub dr_set: crate::Reg<self::prt2::dr_set::DR_SET_SPEC>,
    #[doc = "0x44 - "]
    pub dr_clr: crate::Reg<self::prt2::dr_clr::DR_CLR_SPEC>,
    #[doc = "0x48 - "]
    pub dr_inv: crate::Reg<self::prt2::dr_inv::DR_INV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRT2"]
pub mod prt2;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = ""]
pub mod intr_cause;
#[doc = "DFT_IO_TEST register accessor: an alias for `Reg<DFT_IO_TEST_SPEC>`"]
pub type DFT_IO_TEST = crate::Reg<dft_io_test::DFT_IO_TEST_SPEC>;
#[doc = ""]
pub mod dft_io_test;
