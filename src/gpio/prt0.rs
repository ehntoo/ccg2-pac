#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Port output data register Used to read and write the output data for the IO pads in the port. A DR register write changes the output data to the written value. A DR register read reflects the output data (and not the current state of the input data for the IO pads). Using this DR register, Read-Modify-Write sequences are safely performed on a port with some IO pads configured as inputs."]
pub mod dr;
#[doc = "PS register accessor: an alias for `Reg<PS_SPEC>`"]
pub type PS = crate::Reg<ps::PS_SPEC>;
#[doc = "Port IO pad state register Used to read. Writes to this register have no effect. If the drive mode for the pin is set to high Z Analog, the state will read 0 independent of the voltage on the pin."]
pub mod ps;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Port configuration register Configures the output drive and input buffer state for each pin, and the slew rate and input threshold selection for the whole port. One register is provided per port."]
pub mod pc;
#[doc = "INTR_CFG register accessor: an alias for `Reg<INTR_CFG_SPEC>`"]
pub type INTR_CFG = crate::Reg<intr_cfg::INTR_CFG_SPEC>;
#[doc = "Port interrupt configuration register This register configures the IRQ configuration for all pins in a port, with the IRQ type being individually pin-configurable."]
pub mod intr_cfg;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Port interrupt status register An interrupt cause is cleared (set to '0') by writing a '1' to the corresponding bit field. It is not recommended to write 0xff to clear all interrupt causes, as a new interrupt cause may have occurred between reading the register and clearing. Note that the interrupt cause fields and the associated interrupt provide Hibernate functionality (interrupt causes can be set to '1' and the interrupt can be activated in Hibernate power mode). The PS_DATA fields reflect the logical IO pad states of the port (also found in the PS register)."]
pub mod intr;
#[doc = "PC2 register accessor: an alias for `Reg<PC2_SPEC>`"]
pub type PC2 = crate::Reg<pc2::PC2_SPEC>;
#[doc = "Port configuration register 2 Configures the input disable for each pin."]
pub mod pc2;
#[doc = "DR_SET register accessor: an alias for `Reg<DR_SET_SPEC>`"]
pub type DR_SET = crate::Reg<dr_set::DR_SET_SPEC>;
#[doc = "Port output data set register Used to set output data of specific IO pads in the corresponding port to '1', without affecting the output data of the other IO pads in the port. A DR_SET register read returns the same value as a DR register read."]
pub mod dr_set;
#[doc = "DR_CLR register accessor: an alias for `Reg<DR_CLR_SPEC>`"]
pub type DR_CLR = crate::Reg<dr_clr::DR_CLR_SPEC>;
#[doc = "Port output data clear register Used to clear output data of specific IO pads in the corresponding port to '0', without affecting the output data of the other IO pads in the port. A DR_CLR register read returns the same value as a DR register read."]
pub mod dr_clr;
#[doc = "DR_INV register accessor: an alias for `Reg<DR_INV_SPEC>`"]
pub type DR_INV = crate::Reg<dr_inv::DR_INV_SPEC>;
#[doc = "Port output data invert register Used to invert output data of specific IO pads in the corresponding port, without affecting the output data of the other IO pads in the port. A DR_INV register read returns the same value as a DR register read."]
pub mod dr_inv;
