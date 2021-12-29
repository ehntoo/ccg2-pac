#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - "]
    pub sysreq: crate::Reg<sysreq::SYSREQ_SPEC>,
    #[doc = "0x08 - "]
    pub sysarg: crate::Reg<sysarg::SYSARG_SPEC>,
    #[doc = "0x0c - "]
    pub protection: crate::Reg<protection::PROTECTION_SPEC>,
    #[doc = "0x10 - "]
    pub priv_rom: crate::Reg<priv_rom::PRIV_ROM_SPEC>,
    #[doc = "0x14 - "]
    pub priv_ram: crate::Reg<priv_ram::PRIV_RAM_SPEC>,
    #[doc = "0x18 - "]
    pub priv_flash: crate::Reg<priv_flash::PRIV_FLASH_SPEC>,
    #[doc = "0x1c - "]
    pub wounding: crate::Reg<wounding::WOUNDING_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x2c - "]
    pub dft: crate::Reg<dft::DFT_SPEC>,
    #[doc = "0x30 - "]
    pub flash_ctl: crate::Reg<flash_ctl::FLASH_CTL_SPEC>,
    #[doc = "0x34 - "]
    pub rom_ctl: crate::Reg<rom_ctl::ROM_CTL_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x40 - "]
    pub bist_cmd: crate::Reg<bist_cmd::BIST_CMD_SPEC>,
    #[doc = "0x44 - "]
    pub bist_data: crate::Reg<bist_data::BIST_DATA_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - "]
    pub bist_ctl: crate::Reg<bist_ctl::BIST_CTL_SPEC>,
    #[doc = "0x50 - "]
    pub bist_step0_ctl: crate::Reg<bist_step0_ctl::BIST_STEP0_CTL_SPEC>,
    _reserved15: [u8; 0x2c],
    #[doc = "0x80 - "]
    pub bist_status: crate::Reg<bist_status::BIST_STATUS_SPEC>,
    #[doc = "0x84 - "]
    pub bist_data_act: crate::Reg<bist_data_act::BIST_DATA_ACT_SPEC>,
    #[doc = "0x88 - "]
    pub bist_data_exp: crate::Reg<bist_data_exp::BIST_DATA_EXP_SPEC>,
    #[doc = "0x8c - "]
    pub bist_addr: crate::Reg<bist_addr::BIST_ADDR_SPEC>,
    #[doc = "0x90 - "]
    pub bist_misr: crate::Reg<bist_misr::BIST_MISR_SPEC>,
    _reserved20: [u8; 0x2c],
    #[doc = "0xc0 - "]
    pub ptm_ctl: crate::Reg<ptm_ctl::PTM_CTL_SPEC>,
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = ""]
pub mod config;
#[doc = "SYSREQ register accessor: an alias for `Reg<SYSREQ_SPEC>`"]
pub type SYSREQ = crate::Reg<sysreq::SYSREQ_SPEC>;
#[doc = ""]
pub mod sysreq;
#[doc = "SYSARG register accessor: an alias for `Reg<SYSARG_SPEC>`"]
pub type SYSARG = crate::Reg<sysarg::SYSARG_SPEC>;
#[doc = ""]
pub mod sysarg;
#[doc = "PROTECTION register accessor: an alias for `Reg<PROTECTION_SPEC>`"]
pub type PROTECTION = crate::Reg<protection::PROTECTION_SPEC>;
#[doc = ""]
pub mod protection;
#[doc = "PRIV_ROM register accessor: an alias for `Reg<PRIV_ROM_SPEC>`"]
pub type PRIV_ROM = crate::Reg<priv_rom::PRIV_ROM_SPEC>;
#[doc = ""]
pub mod priv_rom;
#[doc = "PRIV_RAM register accessor: an alias for `Reg<PRIV_RAM_SPEC>`"]
pub type PRIV_RAM = crate::Reg<priv_ram::PRIV_RAM_SPEC>;
#[doc = ""]
pub mod priv_ram;
#[doc = "PRIV_FLASH register accessor: an alias for `Reg<PRIV_FLASH_SPEC>`"]
pub type PRIV_FLASH = crate::Reg<priv_flash::PRIV_FLASH_SPEC>;
#[doc = ""]
pub mod priv_flash;
#[doc = "WOUNDING register accessor: an alias for `Reg<WOUNDING_SPEC>`"]
pub type WOUNDING = crate::Reg<wounding::WOUNDING_SPEC>;
#[doc = ""]
pub mod wounding;
#[doc = "DFT register accessor: an alias for `Reg<DFT_SPEC>`"]
pub type DFT = crate::Reg<dft::DFT_SPEC>;
#[doc = ""]
pub mod dft;
#[doc = "FLASH_CTL register accessor: an alias for `Reg<FLASH_CTL_SPEC>`"]
pub type FLASH_CTL = crate::Reg<flash_ctl::FLASH_CTL_SPEC>;
#[doc = ""]
pub mod flash_ctl;
#[doc = "ROM_CTL register accessor: an alias for `Reg<ROM_CTL_SPEC>`"]
pub type ROM_CTL = crate::Reg<rom_ctl::ROM_CTL_SPEC>;
#[doc = ""]
pub mod rom_ctl;
#[doc = "BIST_CMD register accessor: an alias for `Reg<BIST_CMD_SPEC>`"]
pub type BIST_CMD = crate::Reg<bist_cmd::BIST_CMD_SPEC>;
#[doc = ""]
pub mod bist_cmd;
#[doc = "BIST_DATA register accessor: an alias for `Reg<BIST_DATA_SPEC>`"]
pub type BIST_DATA = crate::Reg<bist_data::BIST_DATA_SPEC>;
#[doc = ""]
pub mod bist_data;
#[doc = "BIST_CTL register accessor: an alias for `Reg<BIST_CTL_SPEC>`"]
pub type BIST_CTL = crate::Reg<bist_ctl::BIST_CTL_SPEC>;
#[doc = ""]
pub mod bist_ctl;
#[doc = "BIST_STEP0_CTL register accessor: an alias for `Reg<BIST_STEP0_CTL_SPEC>`"]
pub type BIST_STEP0_CTL = crate::Reg<bist_step0_ctl::BIST_STEP0_CTL_SPEC>;
#[doc = ""]
pub mod bist_step0_ctl;
#[doc = "BIST_STATUS register accessor: an alias for `Reg<BIST_STATUS_SPEC>`"]
pub type BIST_STATUS = crate::Reg<bist_status::BIST_STATUS_SPEC>;
#[doc = ""]
pub mod bist_status;
#[doc = "BIST_DATA_ACT register accessor: an alias for `Reg<BIST_DATA_ACT_SPEC>`"]
pub type BIST_DATA_ACT = crate::Reg<bist_data_act::BIST_DATA_ACT_SPEC>;
#[doc = ""]
pub mod bist_data_act;
#[doc = "BIST_DATA_EXP register accessor: an alias for `Reg<BIST_DATA_EXP_SPEC>`"]
pub type BIST_DATA_EXP = crate::Reg<bist_data_exp::BIST_DATA_EXP_SPEC>;
#[doc = ""]
pub mod bist_data_exp;
#[doc = "BIST_ADDR register accessor: an alias for `Reg<BIST_ADDR_SPEC>`"]
pub type BIST_ADDR = crate::Reg<bist_addr::BIST_ADDR_SPEC>;
#[doc = ""]
pub mod bist_addr;
#[doc = "BIST_MISR register accessor: an alias for `Reg<BIST_MISR_SPEC>`"]
pub type BIST_MISR = crate::Reg<bist_misr::BIST_MISR_SPEC>;
#[doc = ""]
pub mod bist_misr;
#[doc = "PTM_CTL register accessor: an alias for `Reg<PTM_CTL_SPEC>`"]
pub type PTM_CTL = crate::Reg<ptm_ctl::PTM_CTL_SPEC>;
#[doc = ""]
pub mod ptm_ctl;
