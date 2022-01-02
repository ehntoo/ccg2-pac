#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - SYSCALL control register Used to make system requests to SROM code. System Requests transition from User Mode to Privileged Mode. See SAS for more details. Firmware/ATE should write CPUSS_SYSARG first and CPUSS_SYSREQ register next."]
    pub sysreq: crate::Reg<sysreq::SYSREQ_SPEC>,
    #[doc = "0x08 - SYSARG control register Used to make system requests to SROM code. System Requests transition from User Mode to Privileged Mode. See SAS for more details. Firmware/ATE should write CPUSS_SYSARG first and CPUSS_SYSREQ register next."]
    pub sysarg: crate::Reg<sysarg::SYSARG_SPEC>,
    #[doc = "0x0c - Protection control register"]
    pub protection: crate::Reg<protection::PROTECTION_SPEC>,
    #[doc = "0x10 - ROM privilege register ROM memory has a maximum capacity of 128 KByte. ROM is partitioned into Boot ROM and System ROM. These two partitions are located back-to-back in the system address space. The Boot ROM capacity is 4, 8, 16, or 32 KByte. The System ROM partition capacity equals the ROM capacity minus the Boot ROM partition capacity. User mode accesses to a privileged address result in an AHB-Lite bus error. If the ROM memory capacity is not a power of two, the ROM memory region has an unpopulated/unaccounted memory are (at the end of the ROM memory region). A user mode access to an unpopulated, privileged area (as indicated by the LIMIT field(s)) address results in an AHB-Lite bus error. A user mode access to a unpopulated area, without any access violations, behaves as follows: Reads return \"0\" and writes are ignore (RZWI)."]
    pub priv_rom: crate::Reg<priv_rom::PRIV_ROM_SPEC>,
    #[doc = "0x14 - RAM privilege register User mode accesses to a privileged address result in an AHB-Lite bus error. If the RAM memory capacity is not a power of two, the RAM memory region has an unpopulated/unaccounted memory are (at the end of the RAM memory region). A user mode access to an unpopulated, privileged area (as indicated by the LIMIT field(s)) address results in an AHB-Lite bus error. A user mode access to a unpopulated area, without any access violations, behaves as follows: Reads return \"0\" and writes are ignore (RZWI)."]
    pub priv_ram: crate::Reg<priv_ram::PRIV_RAM_SPEC>,
    #[doc = "0x18 - ROM privilege register User mode accesses to a privileged address result in an+C48 AHB-Lite bus error. If the regular flash memory capacity is not a power of two, the regular flash memory region has an unpopulated/unaccounted memory are (at the end of the regular flash memory region). A user mode access to an unpopulated, privileged area (as indicated by the LIMIT field(s)) address results in an AHB-Lite bus error. A user mode access to a unpopulated area, without any access violations, behaves as follows: Reads return \"0\" and writes are ignore (RZWI)."]
    pub priv_flash: crate::Reg<priv_flash::PRIV_FLASH_SPEC>,
    #[doc = "0x1c - Wounding register Wounding is based on the FLASH/SRAM memory address range. This range is always the next power of 2 multiple of the FLASH/SRAM memory capacity. E.g., a 48 KByte SRAM capacity has a 64 KByte memory address range. With RAM_WOUND is \"0\", all 48 KByte SRAM capacity is accessible. With RAM_WOUND is \"1\", the first 32 KByte SRAM capacity is accessible. With RAM_WOUND is \"2\", the first 16 KByte SRAM capacity is accessible."]
    pub wounding: crate::Reg<wounding::WOUNDING_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x2c - DFT Select Register Controls power switches of FLASH, ROM, SRAM macro(s) in CPUMEMSS. Note that for StreetFighther this functionality is NOT available due to an omission during IP development."]
    pub dft: crate::Reg<dft::DFT_SPEC>,
    #[doc = "0x30 - FLASH control register"]
    pub flash_ctl: crate::Reg<flash_ctl::FLASH_CTL_SPEC>,
    #[doc = "0x34 - ROM control register"]
    pub rom_ctl: crate::Reg<rom_ctl::ROM_CTL_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x40 - Bist command register"]
    pub bist_cmd: crate::Reg<bist_cmd::BIST_CMD_SPEC>,
    #[doc = "0x44 - BIST data register"]
    pub bist_data: crate::Reg<bist_data::BIST_DATA_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - BIST control register"]
    pub bist_ctl: crate::Reg<bist_ctl::BIST_CTL_SPEC>,
    #[doc = "0x50 - BIST step 0 control register"]
    pub bist_step0_ctl: crate::Reg<bist_step0_ctl::BIST_STEP0_CTL_SPEC>,
    _reserved15: [u8; 0x2c],
    #[doc = "0x80 - BIST status register"]
    pub bist_status: crate::Reg<bist_status::BIST_STATUS_SPEC>,
    #[doc = "0x84 - BIST data expected register"]
    pub bist_data_act: crate::Reg<bist_data_act::BIST_DATA_ACT_SPEC>,
    #[doc = "0x88 - BIST data actual register"]
    pub bist_data_exp: crate::Reg<bist_data_exp::BIST_DATA_EXP_SPEC>,
    #[doc = "0x8c - BIST address register"]
    pub bist_addr: crate::Reg<bist_addr::BIST_ADDR_SPEC>,
    #[doc = "0x90 - BIST SROM Multiple Input Shift Register (MISR)"]
    pub bist_misr: crate::Reg<bist_misr::BIST_MISR_SPEC>,
    _reserved20: [u8; 0x2c],
    #[doc = "0xc0 - Parallel Test Mode Control Register This register determines the test interface (SWD or PTM) that connects the ATE to the device."]
    pub ptm_ctl: crate::Reg<ptm_ctl::PTM_CTL_SPEC>,
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "SYSREQ register accessor: an alias for `Reg<SYSREQ_SPEC>`"]
pub type SYSREQ = crate::Reg<sysreq::SYSREQ_SPEC>;
#[doc = "SYSCALL control register Used to make system requests to SROM code. System Requests transition from User Mode to Privileged Mode. See SAS for more details. Firmware/ATE should write CPUSS_SYSARG first and CPUSS_SYSREQ register next."]
pub mod sysreq;
#[doc = "SYSARG register accessor: an alias for `Reg<SYSARG_SPEC>`"]
pub type SYSARG = crate::Reg<sysarg::SYSARG_SPEC>;
#[doc = "SYSARG control register Used to make system requests to SROM code. System Requests transition from User Mode to Privileged Mode. See SAS for more details. Firmware/ATE should write CPUSS_SYSARG first and CPUSS_SYSREQ register next."]
pub mod sysarg;
#[doc = "PROTECTION register accessor: an alias for `Reg<PROTECTION_SPEC>`"]
pub type PROTECTION = crate::Reg<protection::PROTECTION_SPEC>;
#[doc = "Protection control register"]
pub mod protection;
#[doc = "PRIV_ROM register accessor: an alias for `Reg<PRIV_ROM_SPEC>`"]
pub type PRIV_ROM = crate::Reg<priv_rom::PRIV_ROM_SPEC>;
#[doc = "ROM privilege register ROM memory has a maximum capacity of 128 KByte. ROM is partitioned into Boot ROM and System ROM. These two partitions are located back-to-back in the system address space. The Boot ROM capacity is 4, 8, 16, or 32 KByte. The System ROM partition capacity equals the ROM capacity minus the Boot ROM partition capacity. User mode accesses to a privileged address result in an AHB-Lite bus error. If the ROM memory capacity is not a power of two, the ROM memory region has an unpopulated/unaccounted memory are (at the end of the ROM memory region). A user mode access to an unpopulated, privileged area (as indicated by the LIMIT field(s)) address results in an AHB-Lite bus error. A user mode access to a unpopulated area, without any access violations, behaves as follows: Reads return \"0\" and writes are ignore (RZWI)."]
pub mod priv_rom;
#[doc = "PRIV_RAM register accessor: an alias for `Reg<PRIV_RAM_SPEC>`"]
pub type PRIV_RAM = crate::Reg<priv_ram::PRIV_RAM_SPEC>;
#[doc = "RAM privilege register User mode accesses to a privileged address result in an AHB-Lite bus error. If the RAM memory capacity is not a power of two, the RAM memory region has an unpopulated/unaccounted memory are (at the end of the RAM memory region). A user mode access to an unpopulated, privileged area (as indicated by the LIMIT field(s)) address results in an AHB-Lite bus error. A user mode access to a unpopulated area, without any access violations, behaves as follows: Reads return \"0\" and writes are ignore (RZWI)."]
pub mod priv_ram;
#[doc = "PRIV_FLASH register accessor: an alias for `Reg<PRIV_FLASH_SPEC>`"]
pub type PRIV_FLASH = crate::Reg<priv_flash::PRIV_FLASH_SPEC>;
#[doc = "ROM privilege register User mode accesses to a privileged address result in an+C48 AHB-Lite bus error. If the regular flash memory capacity is not a power of two, the regular flash memory region has an unpopulated/unaccounted memory are (at the end of the regular flash memory region). A user mode access to an unpopulated, privileged area (as indicated by the LIMIT field(s)) address results in an AHB-Lite bus error. A user mode access to a unpopulated area, without any access violations, behaves as follows: Reads return \"0\" and writes are ignore (RZWI)."]
pub mod priv_flash;
#[doc = "WOUNDING register accessor: an alias for `Reg<WOUNDING_SPEC>`"]
pub type WOUNDING = crate::Reg<wounding::WOUNDING_SPEC>;
#[doc = "Wounding register Wounding is based on the FLASH/SRAM memory address range. This range is always the next power of 2 multiple of the FLASH/SRAM memory capacity. E.g., a 48 KByte SRAM capacity has a 64 KByte memory address range. With RAM_WOUND is \"0\", all 48 KByte SRAM capacity is accessible. With RAM_WOUND is \"1\", the first 32 KByte SRAM capacity is accessible. With RAM_WOUND is \"2\", the first 16 KByte SRAM capacity is accessible."]
pub mod wounding;
#[doc = "DFT register accessor: an alias for `Reg<DFT_SPEC>`"]
pub type DFT = crate::Reg<dft::DFT_SPEC>;
#[doc = "DFT Select Register Controls power switches of FLASH, ROM, SRAM macro(s) in CPUMEMSS. Note that for StreetFighther this functionality is NOT available due to an omission during IP development."]
pub mod dft;
#[doc = "FLASH_CTL register accessor: an alias for `Reg<FLASH_CTL_SPEC>`"]
pub type FLASH_CTL = crate::Reg<flash_ctl::FLASH_CTL_SPEC>;
#[doc = "FLASH control register"]
pub mod flash_ctl;
#[doc = "ROM_CTL register accessor: an alias for `Reg<ROM_CTL_SPEC>`"]
pub type ROM_CTL = crate::Reg<rom_ctl::ROM_CTL_SPEC>;
#[doc = "ROM control register"]
pub mod rom_ctl;
#[doc = "BIST_CMD register accessor: an alias for `Reg<BIST_CMD_SPEC>`"]
pub type BIST_CMD = crate::Reg<bist_cmd::BIST_CMD_SPEC>;
#[doc = "Bist command register"]
pub mod bist_cmd;
#[doc = "BIST_DATA register accessor: an alias for `Reg<BIST_DATA_SPEC>`"]
pub type BIST_DATA = crate::Reg<bist_data::BIST_DATA_SPEC>;
#[doc = "BIST data register"]
pub mod bist_data;
#[doc = "BIST_CTL register accessor: an alias for `Reg<BIST_CTL_SPEC>`"]
pub type BIST_CTL = crate::Reg<bist_ctl::BIST_CTL_SPEC>;
#[doc = "BIST control register"]
pub mod bist_ctl;
#[doc = "BIST_STEP0_CTL register accessor: an alias for `Reg<BIST_STEP0_CTL_SPEC>`"]
pub type BIST_STEP0_CTL = crate::Reg<bist_step0_ctl::BIST_STEP0_CTL_SPEC>;
#[doc = "BIST step 0 control register"]
pub mod bist_step0_ctl;
#[doc = "BIST_STATUS register accessor: an alias for `Reg<BIST_STATUS_SPEC>`"]
pub type BIST_STATUS = crate::Reg<bist_status::BIST_STATUS_SPEC>;
#[doc = "BIST status register"]
pub mod bist_status;
#[doc = "BIST_DATA_ACT register accessor: an alias for `Reg<BIST_DATA_ACT_SPEC>`"]
pub type BIST_DATA_ACT = crate::Reg<bist_data_act::BIST_DATA_ACT_SPEC>;
#[doc = "BIST data expected register"]
pub mod bist_data_act;
#[doc = "BIST_DATA_EXP register accessor: an alias for `Reg<BIST_DATA_EXP_SPEC>`"]
pub type BIST_DATA_EXP = crate::Reg<bist_data_exp::BIST_DATA_EXP_SPEC>;
#[doc = "BIST data actual register"]
pub mod bist_data_exp;
#[doc = "BIST_ADDR register accessor: an alias for `Reg<BIST_ADDR_SPEC>`"]
pub type BIST_ADDR = crate::Reg<bist_addr::BIST_ADDR_SPEC>;
#[doc = "BIST address register"]
pub mod bist_addr;
#[doc = "BIST_MISR register accessor: an alias for `Reg<BIST_MISR_SPEC>`"]
pub type BIST_MISR = crate::Reg<bist_misr::BIST_MISR_SPEC>;
#[doc = "BIST SROM Multiple Input Shift Register (MISR)"]
pub mod bist_misr;
#[doc = "PTM_CTL register accessor: an alias for `Reg<PTM_CTL_SPEC>`"]
pub type PTM_CTL = crate::Reg<ptm_ctl::PTM_CTL_SPEC>;
#[doc = "Parallel Test Mode Control Register This register determines the test interface (SWD or PTM) that connects the ATE to the device."]
pub mod ptm_ctl;
