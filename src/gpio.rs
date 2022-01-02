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
    #[doc = "0x1000 - Interrupt port cause register"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1010 - IO SELF TEST control register for DfT purposes only This register is used to significantly reduce the test time for IO cells. It also avoids the need to develop a large amount of chip specific functional test vectors. With ATPG it is not possible to get full stuck-at fault coverage for some IO cell inputs (�hld_ovr, oe_n, analog_en, analog_sel, analog_pol�). This register gives direct controlabilty on those inputs of all IO cells by bypassing the functional paths. That allows generic (not chip specific) ROOS code (also SWD IO cells are included) to get DfT fault-coverage for these signals. This register is used in conjunction with the GPIO.PC.dm\\[2:0\\], GPIO.DR.out and SRSS.CORE.PWR_STOP.FREEZE for control and results are observed through GPIO.PS.data. Only one IO cell on the whole chip gets IO_TEST_0 and only one gets IO_TEST_1 and default values of IO_TEST_0 is '0' and IO_TEST_1 is '1'. Also only one IO cell on the whole chip gets asssigned ADFT-0 and only one gets asssigned ADFT-1. All four IO_TEST_0/1 and ADFT-0/1 pins are assigned in the product pin spreadsheet."]
    pub dft_io_test: crate::Reg<dft_io_test::DFT_IO_TEST_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT0 {
    #[doc = "0x00 - Port output data register Used to read and write the output data for the IO pads in the port. A DR register write changes the output data to the written value. A DR register read reflects the output data (and not the current state of the input data for the IO pads). Using this DR register, Read-Modify-Write sequences are safely performed on a port with some IO pads configured as inputs."]
    pub dr: crate::Reg<self::prt0::dr::DR_SPEC>,
    #[doc = "0x04 - Port IO pad state register Used to read. Writes to this register have no effect. If the drive mode for the pin is set to high Z Analog, the state will read 0 independent of the voltage on the pin."]
    pub ps: crate::Reg<self::prt0::ps::PS_SPEC>,
    #[doc = "0x08 - Port configuration register Configures the output drive and input buffer state for each pin, and the slew rate and input threshold selection for the whole port. One register is provided per port."]
    pub pc: crate::Reg<self::prt0::pc::PC_SPEC>,
    #[doc = "0x0c - Port interrupt configuration register This register configures the IRQ configuration for all pins in a port, with the IRQ type being individually pin-configurable."]
    pub intr_cfg: crate::Reg<self::prt0::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x10 - Port interrupt status register An interrupt cause is cleared (set to '0') by writing a '1' to the corresponding bit field. It is not recommended to write 0xff to clear all interrupt causes, as a new interrupt cause may have occurred between reading the register and clearing. Note that the interrupt cause fields and the associated interrupt provide Hibernate functionality (interrupt causes can be set to '1' and the interrupt can be activated in Hibernate power mode). The PS_DATA fields reflect the logical IO pad states of the port (also found in the PS register)."]
    pub intr: crate::Reg<self::prt0::intr::INTR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Port configuration register 2 Configures the input disable for each pin."]
    pub pc2: crate::Reg<self::prt0::pc2::PC2_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - Port output data set register Used to set output data of specific IO pads in the corresponding port to '1', without affecting the output data of the other IO pads in the port. A DR_SET register read returns the same value as a DR register read."]
    pub dr_set: crate::Reg<self::prt0::dr_set::DR_SET_SPEC>,
    #[doc = "0x44 - Port output data clear register Used to clear output data of specific IO pads in the corresponding port to '0', without affecting the output data of the other IO pads in the port. A DR_CLR register read returns the same value as a DR register read."]
    pub dr_clr: crate::Reg<self::prt0::dr_clr::DR_CLR_SPEC>,
    #[doc = "0x48 - Port output data invert register Used to invert output data of specific IO pads in the corresponding port, without affecting the output data of the other IO pads in the port. A DR_INV register read returns the same value as a DR register read."]
    pub dr_inv: crate::Reg<self::prt0::dr_inv::DR_INV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRT0"]
pub mod prt0;
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT1 {
    #[doc = "0x00 - Port output data register Used to read and write the output data for the IO pads in the port. A DR register write changes the output data to the written value. A DR register read reflects the output data (and not the current state of the input data for the IO pads). Using this DR register, Read-Modify-Write sequences are safely performed on a port with some IO pads configured as inputs."]
    pub dr: crate::Reg<self::prt1::dr::DR_SPEC>,
    #[doc = "0x04 - Port IO pad state register Used to read. Writes to this register have no effect. If the drive mode for the pin is set to high Z Analog, the state will read 0 independent of the voltage on the pin."]
    pub ps: crate::Reg<self::prt1::ps::PS_SPEC>,
    #[doc = "0x08 - Port configuration register Configures the output drive and input buffer state for each pin, and the slew rate and input threshold selection for the whole port. One register is provided per port."]
    pub pc: crate::Reg<self::prt1::pc::PC_SPEC>,
    #[doc = "0x0c - Port interrupt configuration register This register configures the IRQ configuration for all pins in a port, with the IRQ type being individually pin-configurable."]
    pub intr_cfg: crate::Reg<self::prt1::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x10 - Port interrupt status register An interrupt cause is cleared (set to '0') by writing a '1' to the corresponding bit field. It is not recommended to write 0xff to clear all interrupt causes, as a new interrupt cause may have occurred between reading the register and clearing. Note that the interrupt cause fields and the associated interrupt provide Hibernate functionality (interrupt causes can be set to '1' and the interrupt can be activated in Hibernate power mode). The PS_DATA fields reflect the logical IO pad states of the port (also found in the PS register)."]
    pub intr: crate::Reg<self::prt1::intr::INTR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Port configuration register 2 Configures the input disable for each pin."]
    pub pc2: crate::Reg<self::prt1::pc2::PC2_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - Port output data set register Used to set output data of specific IO pads in the corresponding port to '1', without affecting the output data of the other IO pads in the port. A DR_SET register read returns the same value as a DR register read."]
    pub dr_set: crate::Reg<self::prt1::dr_set::DR_SET_SPEC>,
    #[doc = "0x44 - Port output data clear register Used to clear output data of specific IO pads in the corresponding port to '0', without affecting the output data of the other IO pads in the port. A DR_CLR register read returns the same value as a DR register read."]
    pub dr_clr: crate::Reg<self::prt1::dr_clr::DR_CLR_SPEC>,
    #[doc = "0x48 - Port output data invert register Used to invert output data of specific IO pads in the corresponding port, without affecting the output data of the other IO pads in the port. A DR_INV register read returns the same value as a DR register read."]
    pub dr_inv: crate::Reg<self::prt1::dr_inv::DR_INV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRT1"]
pub mod prt1;
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT2 {
    #[doc = "0x00 - Port output data register Used to read and write the output data for the IO pads in the port. A DR register write changes the output data to the written value. A DR register read reflects the output data (and not the current state of the input data for the IO pads). Using this DR register, Read-Modify-Write sequences are safely performed on a port with some IO pads configured as inputs."]
    pub dr: crate::Reg<self::prt2::dr::DR_SPEC>,
    #[doc = "0x04 - Port IO pad state register Used to read. Writes to this register have no effect. If the drive mode for the pin is set to high Z Analog, the state will read 0 independent of the voltage on the pin."]
    pub ps: crate::Reg<self::prt2::ps::PS_SPEC>,
    #[doc = "0x08 - Port configuration register Configures the output drive and input buffer state for each pin, and the slew rate and input threshold selection for the whole port. One register is provided per port."]
    pub pc: crate::Reg<self::prt2::pc::PC_SPEC>,
    #[doc = "0x0c - Port interrupt configuration register This register configures the IRQ configuration for all pins in a port, with the IRQ type being individually pin-configurable."]
    pub intr_cfg: crate::Reg<self::prt2::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x10 - Port interrupt status register An interrupt cause is cleared (set to '0') by writing a '1' to the corresponding bit field. It is not recommended to write 0xff to clear all interrupt causes, as a new interrupt cause may have occurred between reading the register and clearing. Note that the interrupt cause fields and the associated interrupt provide Hibernate functionality (interrupt causes can be set to '1' and the interrupt can be activated in Hibernate power mode). The PS_DATA fields reflect the logical IO pad states of the port (also found in the PS register)."]
    pub intr: crate::Reg<self::prt2::intr::INTR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Port configuration register 2 Configures the input disable for each pin."]
    pub pc2: crate::Reg<self::prt2::pc2::PC2_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - Port output data set register Used to set output data of specific IO pads in the corresponding port to '1', without affecting the output data of the other IO pads in the port. A DR_SET register read returns the same value as a DR register read."]
    pub dr_set: crate::Reg<self::prt2::dr_set::DR_SET_SPEC>,
    #[doc = "0x44 - Port output data clear register Used to clear output data of specific IO pads in the corresponding port to '0', without affecting the output data of the other IO pads in the port. A DR_CLR register read returns the same value as a DR register read."]
    pub dr_clr: crate::Reg<self::prt2::dr_clr::DR_CLR_SPEC>,
    #[doc = "0x48 - Port output data invert register Used to invert output data of specific IO pads in the corresponding port, without affecting the output data of the other IO pads in the port. A DR_INV register read returns the same value as a DR register read."]
    pub dr_inv: crate::Reg<self::prt2::dr_inv::DR_INV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRT2"]
pub mod prt2;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt port cause register"]
pub mod intr_cause;
#[doc = "DFT_IO_TEST register accessor: an alias for `Reg<DFT_IO_TEST_SPEC>`"]
pub type DFT_IO_TEST = crate::Reg<dft_io_test::DFT_IO_TEST_SPEC>;
#[doc = "IO SELF TEST control register for DfT purposes only This register is used to significantly reduce the test time for IO cells. It also avoids the need to develop a large amount of chip specific functional test vectors. With ATPG it is not possible to get full stuck-at fault coverage for some IO cell inputs (�hld_ovr, oe_n, analog_en, analog_sel, analog_pol�). This register gives direct controlabilty on those inputs of all IO cells by bypassing the functional paths. That allows generic (not chip specific) ROOS code (also SWD IO cells are included) to get DfT fault-coverage for these signals. This register is used in conjunction with the GPIO.PC.dm\\[2:0\\], GPIO.DR.out and SRSS.CORE.PWR_STOP.FREEZE for control and results are observed through GPIO.PS.data. Only one IO cell on the whole chip gets IO_TEST_0 and only one gets IO_TEST_1 and default values of IO_TEST_0 is '0' and IO_TEST_1 is '1'. Also only one IO cell on the whole chip gets asssigned ADFT-0 and only one gets asssigned ADFT-1. All four IO_TEST_0/1 and ADFT-0/1 pins are assigned in the product pin spreadsheet."]
pub mod dft_io_test;
