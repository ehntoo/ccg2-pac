#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Divider command register The (PA_SEL_TYPE, PA_SEL_DIV) field pair allows a divider to be phase aligned with another divider. E.g., consider a 48 MHz \"clk_hf\", and a need for a 12 MHz divided clock A and a 8 MHz divided clock B. Clock A uses 8.0 integer divider 0 and is created by aligning it to \"clk_hf\" ((PA_SEL_TYPE, PA_SEL_DIV) is (3, 63)) and DIV_8_CTL0.INT8_DIV is \"4-1\". Clock B uses 8.0 integer divider 1 and is created by aligning it to clock A ((PA_SEL_TYPE, PA_SEL_DIV) is (0, 0)) and DIV_8_CTL1.INT8_DIV is \"6-1\". This guarantees that clock B is phase aligned with clock B: as the smallest common multiple of the two clock periods is 12 \"clk_hf\" cycles, the clocks A and B will be aligned every 12 \"clk_hf\" cycles. Note: clock B is phase aligned to clock A, but still uses \"clk_hf\" as a reference clock for its divider value."]
    pub div_cmd: crate::Reg<div_cmd::DIV_CMD_SPEC>,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - Programmable clock control register"]
    pub pclk_ctl0: crate::Reg<pclk_ctl0::PCLK_CTL0_SPEC>,
    #[doc = "0x104 - Programmable clock control register"]
    pub pclk_ctl1: crate::Reg<pclk_ctl1::PCLK_CTL1_SPEC>,
    #[doc = "0x108 - Programmable clock control register"]
    pub pclk_ctl2: crate::Reg<pclk_ctl2::PCLK_CTL2_SPEC>,
    #[doc = "0x10c - Programmable clock control register"]
    pub pclk_ctl3: crate::Reg<pclk_ctl3::PCLK_CTL3_SPEC>,
    #[doc = "0x110 - Programmable clock control register"]
    pub pclk_ctl4: crate::Reg<pclk_ctl4::PCLK_CTL4_SPEC>,
    #[doc = "0x114 - Programmable clock control register"]
    pub pclk_ctl5: crate::Reg<pclk_ctl5::PCLK_CTL5_SPEC>,
    #[doc = "0x118 - Programmable clock control register"]
    pub pclk_ctl6: crate::Reg<pclk_ctl6::PCLK_CTL6_SPEC>,
    #[doc = "0x11c - Programmable clock control register"]
    pub pclk_ctl7: crate::Reg<pclk_ctl7::PCLK_CTL7_SPEC>,
    #[doc = "0x120 - Programmable clock control register"]
    pub pclk_ctl8: crate::Reg<pclk_ctl8::PCLK_CTL8_SPEC>,
    #[doc = "0x124 - Programmable clock control register"]
    pub pclk_ctl9: crate::Reg<pclk_ctl9::PCLK_CTL9_SPEC>,
    #[doc = "0x128 - Programmable clock control register"]
    pub pclk_ctl10: crate::Reg<pclk_ctl10::PCLK_CTL10_SPEC>,
    _reserved12: [u8; 0xd4],
    #[doc = "0x200 - Divider control register (for 8.0 divider) Smallest of the divider types."]
    pub div_8_ctl0: crate::Reg<div_8_ctl0::DIV_8_CTL0_SPEC>,
    #[doc = "0x204 - Divider control register (for 8.0 divider) Smallest of the divider types."]
    pub div_8_ctl1: crate::Reg<div_8_ctl1::DIV_8_CTL1_SPEC>,
    #[doc = "0x208 - Divider control register (for 8.0 divider) Smallest of the divider types."]
    pub div_8_ctl2: crate::Reg<div_8_ctl2::DIV_8_CTL2_SPEC>,
    #[doc = "0x20c - Divider control register (for 8.0 divider) Smallest of the divider types."]
    pub div_8_ctl3: crate::Reg<div_8_ctl3::DIV_8_CTL3_SPEC>,
    _reserved16: [u8; 0xf0],
    #[doc = "0x300 - Divider control register (for 16.0 divider)"]
    pub div_16_ctl0: crate::Reg<div_16_ctl0::DIV_16_CTL0_SPEC>,
    #[doc = "0x304 - Divider control register (for 16.0 divider)"]
    pub div_16_ctl1: crate::Reg<div_16_ctl1::DIV_16_CTL1_SPEC>,
    _reserved18: [u8; 0xf8],
    #[doc = "0x400 - Divider control register (for 16.5 divider)"]
    pub div_16_5_ctl0: crate::Reg<div_16_5_ctl0::DIV_16_5_CTL0_SPEC>,
    #[doc = "0x404 - Divider control register (for 16.5 divider)"]
    pub div_16_5_ctl1: crate::Reg<div_16_5_ctl1::DIV_16_5_CTL1_SPEC>,
}
#[doc = "DIV_CMD register accessor: an alias for `Reg<DIV_CMD_SPEC>`"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = "Divider command register The (PA_SEL_TYPE, PA_SEL_DIV) field pair allows a divider to be phase aligned with another divider. E.g., consider a 48 MHz \"clk_hf\", and a need for a 12 MHz divided clock A and a 8 MHz divided clock B. Clock A uses 8.0 integer divider 0 and is created by aligning it to \"clk_hf\" ((PA_SEL_TYPE, PA_SEL_DIV) is (3, 63)) and DIV_8_CTL0.INT8_DIV is \"4-1\". Clock B uses 8.0 integer divider 1 and is created by aligning it to clock A ((PA_SEL_TYPE, PA_SEL_DIV) is (0, 0)) and DIV_8_CTL1.INT8_DIV is \"6-1\". This guarantees that clock B is phase aligned with clock B: as the smallest common multiple of the two clock periods is 12 \"clk_hf\" cycles, the clocks A and B will be aligned every 12 \"clk_hf\" cycles. Note: clock B is phase aligned to clock A, but still uses \"clk_hf\" as a reference clock for its divider value."]
pub mod div_cmd;
#[doc = "PCLK_CTL0 register accessor: an alias for `Reg<PCLK_CTL0_SPEC>`"]
pub type PCLK_CTL0 = crate::Reg<pclk_ctl0::PCLK_CTL0_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl0;
#[doc = "PCLK_CTL1 register accessor: an alias for `Reg<PCLK_CTL1_SPEC>`"]
pub type PCLK_CTL1 = crate::Reg<pclk_ctl1::PCLK_CTL1_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl1;
#[doc = "PCLK_CTL2 register accessor: an alias for `Reg<PCLK_CTL2_SPEC>`"]
pub type PCLK_CTL2 = crate::Reg<pclk_ctl2::PCLK_CTL2_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl2;
#[doc = "PCLK_CTL3 register accessor: an alias for `Reg<PCLK_CTL3_SPEC>`"]
pub type PCLK_CTL3 = crate::Reg<pclk_ctl3::PCLK_CTL3_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl3;
#[doc = "PCLK_CTL4 register accessor: an alias for `Reg<PCLK_CTL4_SPEC>`"]
pub type PCLK_CTL4 = crate::Reg<pclk_ctl4::PCLK_CTL4_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl4;
#[doc = "PCLK_CTL5 register accessor: an alias for `Reg<PCLK_CTL5_SPEC>`"]
pub type PCLK_CTL5 = crate::Reg<pclk_ctl5::PCLK_CTL5_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl5;
#[doc = "PCLK_CTL6 register accessor: an alias for `Reg<PCLK_CTL6_SPEC>`"]
pub type PCLK_CTL6 = crate::Reg<pclk_ctl6::PCLK_CTL6_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl6;
#[doc = "PCLK_CTL7 register accessor: an alias for `Reg<PCLK_CTL7_SPEC>`"]
pub type PCLK_CTL7 = crate::Reg<pclk_ctl7::PCLK_CTL7_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl7;
#[doc = "PCLK_CTL8 register accessor: an alias for `Reg<PCLK_CTL8_SPEC>`"]
pub type PCLK_CTL8 = crate::Reg<pclk_ctl8::PCLK_CTL8_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl8;
#[doc = "PCLK_CTL9 register accessor: an alias for `Reg<PCLK_CTL9_SPEC>`"]
pub type PCLK_CTL9 = crate::Reg<pclk_ctl9::PCLK_CTL9_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl9;
#[doc = "PCLK_CTL10 register accessor: an alias for `Reg<PCLK_CTL10_SPEC>`"]
pub type PCLK_CTL10 = crate::Reg<pclk_ctl10::PCLK_CTL10_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl10;
#[doc = "DIV_8_CTL0 register accessor: an alias for `Reg<DIV_8_CTL0_SPEC>`"]
pub type DIV_8_CTL0 = crate::Reg<div_8_ctl0::DIV_8_CTL0_SPEC>;
#[doc = "Divider control register (for 8.0 divider) Smallest of the divider types."]
pub mod div_8_ctl0;
#[doc = "DIV_8_CTL1 register accessor: an alias for `Reg<DIV_8_CTL1_SPEC>`"]
pub type DIV_8_CTL1 = crate::Reg<div_8_ctl1::DIV_8_CTL1_SPEC>;
#[doc = "Divider control register (for 8.0 divider) Smallest of the divider types."]
pub mod div_8_ctl1;
#[doc = "DIV_8_CTL2 register accessor: an alias for `Reg<DIV_8_CTL2_SPEC>`"]
pub type DIV_8_CTL2 = crate::Reg<div_8_ctl2::DIV_8_CTL2_SPEC>;
#[doc = "Divider control register (for 8.0 divider) Smallest of the divider types."]
pub mod div_8_ctl2;
#[doc = "DIV_8_CTL3 register accessor: an alias for `Reg<DIV_8_CTL3_SPEC>`"]
pub type DIV_8_CTL3 = crate::Reg<div_8_ctl3::DIV_8_CTL3_SPEC>;
#[doc = "Divider control register (for 8.0 divider) Smallest of the divider types."]
pub mod div_8_ctl3;
#[doc = "DIV_16_CTL0 register accessor: an alias for `Reg<DIV_16_CTL0_SPEC>`"]
pub type DIV_16_CTL0 = crate::Reg<div_16_ctl0::DIV_16_CTL0_SPEC>;
#[doc = "Divider control register (for 16.0 divider)"]
pub mod div_16_ctl0;
#[doc = "DIV_16_CTL1 register accessor: an alias for `Reg<DIV_16_CTL1_SPEC>`"]
pub type DIV_16_CTL1 = crate::Reg<div_16_ctl1::DIV_16_CTL1_SPEC>;
#[doc = "Divider control register (for 16.0 divider)"]
pub mod div_16_ctl1;
#[doc = "DIV_16_5_CTL0 register accessor: an alias for `Reg<DIV_16_5_CTL0_SPEC>`"]
pub type DIV_16_5_CTL0 = crate::Reg<div_16_5_ctl0::DIV_16_5_CTL0_SPEC>;
#[doc = "Divider control register (for 16.5 divider)"]
pub mod div_16_5_ctl0;
#[doc = "DIV_16_5_CTL1 register accessor: an alias for `Reg<DIV_16_5_CTL1_SPEC>`"]
pub type DIV_16_5_CTL1 = crate::Reg<div_16_5_ctl1::DIV_16_5_CTL1_SPEC>;
#[doc = "Divider control register (for 16.5 divider)"]
pub mod div_16_5_ctl1;
