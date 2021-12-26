#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub div_cmd: crate::Reg<div_cmd::DIV_CMD_SPEC>,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - "]
    pub pclk_ctl0: crate::Reg<pclk_ctl0::PCLK_CTL0_SPEC>,
    #[doc = "0x104 - "]
    pub pclk_ctl1: crate::Reg<pclk_ctl1::PCLK_CTL1_SPEC>,
    #[doc = "0x108 - "]
    pub pclk_ctl2: crate::Reg<pclk_ctl2::PCLK_CTL2_SPEC>,
    #[doc = "0x10c - "]
    pub pclk_ctl3: crate::Reg<pclk_ctl3::PCLK_CTL3_SPEC>,
    #[doc = "0x110 - "]
    pub pclk_ctl4: crate::Reg<pclk_ctl4::PCLK_CTL4_SPEC>,
    #[doc = "0x114 - "]
    pub pclk_ctl5: crate::Reg<pclk_ctl5::PCLK_CTL5_SPEC>,
    #[doc = "0x118 - "]
    pub pclk_ctl6: crate::Reg<pclk_ctl6::PCLK_CTL6_SPEC>,
    #[doc = "0x11c - "]
    pub pclk_ctl7: crate::Reg<pclk_ctl7::PCLK_CTL7_SPEC>,
    #[doc = "0x120 - "]
    pub pclk_ctl8: crate::Reg<pclk_ctl8::PCLK_CTL8_SPEC>,
    #[doc = "0x124 - "]
    pub pclk_ctl9: crate::Reg<pclk_ctl9::PCLK_CTL9_SPEC>,
    #[doc = "0x128 - "]
    pub pclk_ctl10: crate::Reg<pclk_ctl10::PCLK_CTL10_SPEC>,
    _reserved12: [u8; 0xd4],
    #[doc = "0x200 - "]
    pub div_8_ctl0: crate::Reg<div_8_ctl0::DIV_8_CTL0_SPEC>,
    #[doc = "0x204 - "]
    pub div_8_ctl1: crate::Reg<div_8_ctl1::DIV_8_CTL1_SPEC>,
    #[doc = "0x208 - "]
    pub div_8_ctl2: crate::Reg<div_8_ctl2::DIV_8_CTL2_SPEC>,
    #[doc = "0x20c - "]
    pub div_8_ctl3: crate::Reg<div_8_ctl3::DIV_8_CTL3_SPEC>,
    _reserved16: [u8; 0xf0],
    #[doc = "0x300 - "]
    pub div_16_ctl0: crate::Reg<div_16_ctl0::DIV_16_CTL0_SPEC>,
    #[doc = "0x304 - "]
    pub div_16_ctl1: crate::Reg<div_16_ctl1::DIV_16_CTL1_SPEC>,
    _reserved18: [u8; 0xf8],
    #[doc = "0x400 - "]
    pub div_16_5_ctl0: crate::Reg<div_16_5_ctl0::DIV_16_5_CTL0_SPEC>,
    #[doc = "0x404 - "]
    pub div_16_5_ctl1: crate::Reg<div_16_5_ctl1::DIV_16_5_CTL1_SPEC>,
}
#[doc = "DIV_CMD register accessor: an alias for `Reg<DIV_CMD_SPEC>`"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = ""]
pub mod div_cmd;
#[doc = "PCLK_CTL0 register accessor: an alias for `Reg<PCLK_CTL0_SPEC>`"]
pub type PCLK_CTL0 = crate::Reg<pclk_ctl0::PCLK_CTL0_SPEC>;
#[doc = ""]
pub mod pclk_ctl0;
#[doc = "PCLK_CTL1 register accessor: an alias for `Reg<PCLK_CTL1_SPEC>`"]
pub type PCLK_CTL1 = crate::Reg<pclk_ctl1::PCLK_CTL1_SPEC>;
#[doc = ""]
pub mod pclk_ctl1;
#[doc = "PCLK_CTL2 register accessor: an alias for `Reg<PCLK_CTL2_SPEC>`"]
pub type PCLK_CTL2 = crate::Reg<pclk_ctl2::PCLK_CTL2_SPEC>;
#[doc = ""]
pub mod pclk_ctl2;
#[doc = "PCLK_CTL3 register accessor: an alias for `Reg<PCLK_CTL3_SPEC>`"]
pub type PCLK_CTL3 = crate::Reg<pclk_ctl3::PCLK_CTL3_SPEC>;
#[doc = ""]
pub mod pclk_ctl3;
#[doc = "PCLK_CTL4 register accessor: an alias for `Reg<PCLK_CTL4_SPEC>`"]
pub type PCLK_CTL4 = crate::Reg<pclk_ctl4::PCLK_CTL4_SPEC>;
#[doc = ""]
pub mod pclk_ctl4;
#[doc = "PCLK_CTL5 register accessor: an alias for `Reg<PCLK_CTL5_SPEC>`"]
pub type PCLK_CTL5 = crate::Reg<pclk_ctl5::PCLK_CTL5_SPEC>;
#[doc = ""]
pub mod pclk_ctl5;
#[doc = "PCLK_CTL6 register accessor: an alias for `Reg<PCLK_CTL6_SPEC>`"]
pub type PCLK_CTL6 = crate::Reg<pclk_ctl6::PCLK_CTL6_SPEC>;
#[doc = ""]
pub mod pclk_ctl6;
#[doc = "PCLK_CTL7 register accessor: an alias for `Reg<PCLK_CTL7_SPEC>`"]
pub type PCLK_CTL7 = crate::Reg<pclk_ctl7::PCLK_CTL7_SPEC>;
#[doc = ""]
pub mod pclk_ctl7;
#[doc = "PCLK_CTL8 register accessor: an alias for `Reg<PCLK_CTL8_SPEC>`"]
pub type PCLK_CTL8 = crate::Reg<pclk_ctl8::PCLK_CTL8_SPEC>;
#[doc = ""]
pub mod pclk_ctl8;
#[doc = "PCLK_CTL9 register accessor: an alias for `Reg<PCLK_CTL9_SPEC>`"]
pub type PCLK_CTL9 = crate::Reg<pclk_ctl9::PCLK_CTL9_SPEC>;
#[doc = ""]
pub mod pclk_ctl9;
#[doc = "PCLK_CTL10 register accessor: an alias for `Reg<PCLK_CTL10_SPEC>`"]
pub type PCLK_CTL10 = crate::Reg<pclk_ctl10::PCLK_CTL10_SPEC>;
#[doc = ""]
pub mod pclk_ctl10;
#[doc = "DIV_8_CTL0 register accessor: an alias for `Reg<DIV_8_CTL0_SPEC>`"]
pub type DIV_8_CTL0 = crate::Reg<div_8_ctl0::DIV_8_CTL0_SPEC>;
#[doc = ""]
pub mod div_8_ctl0;
#[doc = "DIV_8_CTL1 register accessor: an alias for `Reg<DIV_8_CTL1_SPEC>`"]
pub type DIV_8_CTL1 = crate::Reg<div_8_ctl1::DIV_8_CTL1_SPEC>;
#[doc = ""]
pub mod div_8_ctl1;
#[doc = "DIV_8_CTL2 register accessor: an alias for `Reg<DIV_8_CTL2_SPEC>`"]
pub type DIV_8_CTL2 = crate::Reg<div_8_ctl2::DIV_8_CTL2_SPEC>;
#[doc = ""]
pub mod div_8_ctl2;
#[doc = "DIV_8_CTL3 register accessor: an alias for `Reg<DIV_8_CTL3_SPEC>`"]
pub type DIV_8_CTL3 = crate::Reg<div_8_ctl3::DIV_8_CTL3_SPEC>;
#[doc = ""]
pub mod div_8_ctl3;
#[doc = "DIV_16_CTL0 register accessor: an alias for `Reg<DIV_16_CTL0_SPEC>`"]
pub type DIV_16_CTL0 = crate::Reg<div_16_ctl0::DIV_16_CTL0_SPEC>;
#[doc = ""]
pub mod div_16_ctl0;
#[doc = "DIV_16_CTL1 register accessor: an alias for `Reg<DIV_16_CTL1_SPEC>`"]
pub type DIV_16_CTL1 = crate::Reg<div_16_ctl1::DIV_16_CTL1_SPEC>;
#[doc = ""]
pub mod div_16_ctl1;
#[doc = "DIV_16_5_CTL0 register accessor: an alias for `Reg<DIV_16_5_CTL0_SPEC>`"]
pub type DIV_16_5_CTL0 = crate::Reg<div_16_5_ctl0::DIV_16_5_CTL0_SPEC>;
#[doc = ""]
pub mod div_16_5_ctl0;
#[doc = "DIV_16_5_CTL1 register accessor: an alias for `Reg<DIV_16_5_CTL1_SPEC>`"]
pub type DIV_16_5_CTL1 = crate::Reg<div_16_5_ctl1::DIV_16_5_CTL1_SPEC>;
#[doc = ""]
pub mod div_16_5_ctl1;
