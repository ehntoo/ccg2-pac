#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCPWM control register 0. Used to disbale/enable the counters."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - TCPWM command register. Enables software controlled counter operation."]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x0c - TCPWM Counter interrupt cause register. Enables software to determine the source of the combined interrupt output signal \"interrupt\". The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode."]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved3: [u8; 0xf0],
    #[doc = "0x100..0x140 - CNT0"]
    pub cnt0: CNT0,
    #[doc = "0x140..0x180 - CNT1"]
    pub cnt1: CNT1,
    #[doc = "0x180..0x1c0 - CNT2"]
    pub cnt2: CNT2,
    #[doc = "0x1c0..0x200 - CNT3"]
    pub cnt3: CNT3,
    #[doc = "0x200..0x240 - CNT4"]
    pub cnt4: CNT4,
    #[doc = "0x240..0x280 - CNT5"]
    pub cnt5: CNT5,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT0 {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: crate::Reg<self::cnt0::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter status register"]
    pub status: crate::Reg<self::cnt0::status::STATUS_SPEC>,
    #[doc = "0x08 - Counter count register"]
    pub counter: crate::Reg<self::cnt0::counter::COUNTER_SPEC>,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: crate::Reg<self::cnt0::cc::CC_SPEC>,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: crate::Reg<self::cnt0::cc_buff::CC_BUFF_SPEC>,
    #[doc = "0x14 - Counter period register"]
    pub period: crate::Reg<self::cnt0::period::PERIOD_SPEC>,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: crate::Reg<self::cnt0::period_buff::PERIOD_BUFF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0 Used to select triggers for specific counter events."]
    pub tr_ctrl0: crate::Reg<self::cnt0::tr_ctrl0::TR_CTRL0_SPEC>,
    #[doc = "0x24 - Counter trigger control register 1 Used to determine edge detection for specific counter triggers. Events will only take effect on enabled counters."]
    pub tr_ctrl1: crate::Reg<self::cnt0::tr_ctrl1::TR_CTRL1_SPEC>,
    #[doc = "0x28 - Counter trigger control register 2 Used to control counter \"line_out\", \"dt_line_out\" and \"dt_line_compl_out\" output signals."]
    pub tr_ctrl2: crate::Reg<self::cnt0::tr_ctrl2::TR_CTRL2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register. The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode. HW clears the interrupt causes to '0', when the counter is disabled."]
    pub intr: crate::Reg<self::cnt0::intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt set request register. When read, this register reflects the interrupt request register."]
    pub intr_set: crate::Reg<self::cnt0::intr_set::INTR_SET_SPEC>,
    #[doc = "0x38 - Interrupt mask register."]
    pub intr_mask: crate::Reg<self::cnt0::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x3c - Interrupt masked request register When read, this register reflects a bitwise AND between the interrupt request and mask registers."]
    pub intr_masked: crate::Reg<self::cnt0::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CNT0"]
pub mod cnt0;
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT1 {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: crate::Reg<self::cnt1::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter status register"]
    pub status: crate::Reg<self::cnt1::status::STATUS_SPEC>,
    #[doc = "0x08 - Counter count register"]
    pub counter: crate::Reg<self::cnt1::counter::COUNTER_SPEC>,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: crate::Reg<self::cnt1::cc::CC_SPEC>,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: crate::Reg<self::cnt1::cc_buff::CC_BUFF_SPEC>,
    #[doc = "0x14 - Counter period register"]
    pub period: crate::Reg<self::cnt1::period::PERIOD_SPEC>,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: crate::Reg<self::cnt1::period_buff::PERIOD_BUFF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0 Used to select triggers for specific counter events."]
    pub tr_ctrl0: crate::Reg<self::cnt1::tr_ctrl0::TR_CTRL0_SPEC>,
    #[doc = "0x24 - Counter trigger control register 1 Used to determine edge detection for specific counter triggers. Events will only take effect on enabled counters."]
    pub tr_ctrl1: crate::Reg<self::cnt1::tr_ctrl1::TR_CTRL1_SPEC>,
    #[doc = "0x28 - Counter trigger control register 2 Used to control counter \"line_out\", \"dt_line_out\" and \"dt_line_compl_out\" output signals."]
    pub tr_ctrl2: crate::Reg<self::cnt1::tr_ctrl2::TR_CTRL2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register. The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode. HW clears the interrupt causes to '0', when the counter is disabled."]
    pub intr: crate::Reg<self::cnt1::intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt set request register. When read, this register reflects the interrupt request register."]
    pub intr_set: crate::Reg<self::cnt1::intr_set::INTR_SET_SPEC>,
    #[doc = "0x38 - Interrupt mask register."]
    pub intr_mask: crate::Reg<self::cnt1::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x3c - Interrupt masked request register When read, this register reflects a bitwise AND between the interrupt request and mask registers."]
    pub intr_masked: crate::Reg<self::cnt1::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CNT1"]
pub mod cnt1;
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT2 {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: crate::Reg<self::cnt2::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter status register"]
    pub status: crate::Reg<self::cnt2::status::STATUS_SPEC>,
    #[doc = "0x08 - Counter count register"]
    pub counter: crate::Reg<self::cnt2::counter::COUNTER_SPEC>,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: crate::Reg<self::cnt2::cc::CC_SPEC>,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: crate::Reg<self::cnt2::cc_buff::CC_BUFF_SPEC>,
    #[doc = "0x14 - Counter period register"]
    pub period: crate::Reg<self::cnt2::period::PERIOD_SPEC>,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: crate::Reg<self::cnt2::period_buff::PERIOD_BUFF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0 Used to select triggers for specific counter events."]
    pub tr_ctrl0: crate::Reg<self::cnt2::tr_ctrl0::TR_CTRL0_SPEC>,
    #[doc = "0x24 - Counter trigger control register 1 Used to determine edge detection for specific counter triggers. Events will only take effect on enabled counters."]
    pub tr_ctrl1: crate::Reg<self::cnt2::tr_ctrl1::TR_CTRL1_SPEC>,
    #[doc = "0x28 - Counter trigger control register 2 Used to control counter \"line_out\", \"dt_line_out\" and \"dt_line_compl_out\" output signals."]
    pub tr_ctrl2: crate::Reg<self::cnt2::tr_ctrl2::TR_CTRL2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register. The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode. HW clears the interrupt causes to '0', when the counter is disabled."]
    pub intr: crate::Reg<self::cnt2::intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt set request register. When read, this register reflects the interrupt request register."]
    pub intr_set: crate::Reg<self::cnt2::intr_set::INTR_SET_SPEC>,
    #[doc = "0x38 - Interrupt mask register."]
    pub intr_mask: crate::Reg<self::cnt2::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x3c - Interrupt masked request register When read, this register reflects a bitwise AND between the interrupt request and mask registers."]
    pub intr_masked: crate::Reg<self::cnt2::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CNT2"]
pub mod cnt2;
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT3 {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: crate::Reg<self::cnt3::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter status register"]
    pub status: crate::Reg<self::cnt3::status::STATUS_SPEC>,
    #[doc = "0x08 - Counter count register"]
    pub counter: crate::Reg<self::cnt3::counter::COUNTER_SPEC>,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: crate::Reg<self::cnt3::cc::CC_SPEC>,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: crate::Reg<self::cnt3::cc_buff::CC_BUFF_SPEC>,
    #[doc = "0x14 - Counter period register"]
    pub period: crate::Reg<self::cnt3::period::PERIOD_SPEC>,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: crate::Reg<self::cnt3::period_buff::PERIOD_BUFF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0 Used to select triggers for specific counter events."]
    pub tr_ctrl0: crate::Reg<self::cnt3::tr_ctrl0::TR_CTRL0_SPEC>,
    #[doc = "0x24 - Counter trigger control register 1 Used to determine edge detection for specific counter triggers. Events will only take effect on enabled counters."]
    pub tr_ctrl1: crate::Reg<self::cnt3::tr_ctrl1::TR_CTRL1_SPEC>,
    #[doc = "0x28 - Counter trigger control register 2 Used to control counter \"line_out\", \"dt_line_out\" and \"dt_line_compl_out\" output signals."]
    pub tr_ctrl2: crate::Reg<self::cnt3::tr_ctrl2::TR_CTRL2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register. The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode. HW clears the interrupt causes to '0', when the counter is disabled."]
    pub intr: crate::Reg<self::cnt3::intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt set request register. When read, this register reflects the interrupt request register."]
    pub intr_set: crate::Reg<self::cnt3::intr_set::INTR_SET_SPEC>,
    #[doc = "0x38 - Interrupt mask register."]
    pub intr_mask: crate::Reg<self::cnt3::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x3c - Interrupt masked request register When read, this register reflects a bitwise AND between the interrupt request and mask registers."]
    pub intr_masked: crate::Reg<self::cnt3::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CNT3"]
pub mod cnt3;
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT4 {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: crate::Reg<self::cnt4::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter status register"]
    pub status: crate::Reg<self::cnt4::status::STATUS_SPEC>,
    #[doc = "0x08 - Counter count register"]
    pub counter: crate::Reg<self::cnt4::counter::COUNTER_SPEC>,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: crate::Reg<self::cnt4::cc::CC_SPEC>,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: crate::Reg<self::cnt4::cc_buff::CC_BUFF_SPEC>,
    #[doc = "0x14 - Counter period register"]
    pub period: crate::Reg<self::cnt4::period::PERIOD_SPEC>,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: crate::Reg<self::cnt4::period_buff::PERIOD_BUFF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0 Used to select triggers for specific counter events."]
    pub tr_ctrl0: crate::Reg<self::cnt4::tr_ctrl0::TR_CTRL0_SPEC>,
    #[doc = "0x24 - Counter trigger control register 1 Used to determine edge detection for specific counter triggers. Events will only take effect on enabled counters."]
    pub tr_ctrl1: crate::Reg<self::cnt4::tr_ctrl1::TR_CTRL1_SPEC>,
    #[doc = "0x28 - Counter trigger control register 2 Used to control counter \"line_out\", \"dt_line_out\" and \"dt_line_compl_out\" output signals."]
    pub tr_ctrl2: crate::Reg<self::cnt4::tr_ctrl2::TR_CTRL2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register. The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode. HW clears the interrupt causes to '0', when the counter is disabled."]
    pub intr: crate::Reg<self::cnt4::intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt set request register. When read, this register reflects the interrupt request register."]
    pub intr_set: crate::Reg<self::cnt4::intr_set::INTR_SET_SPEC>,
    #[doc = "0x38 - Interrupt mask register."]
    pub intr_mask: crate::Reg<self::cnt4::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x3c - Interrupt masked request register When read, this register reflects a bitwise AND between the interrupt request and mask registers."]
    pub intr_masked: crate::Reg<self::cnt4::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CNT4"]
pub mod cnt4;
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT5 {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: crate::Reg<self::cnt5::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter status register"]
    pub status: crate::Reg<self::cnt5::status::STATUS_SPEC>,
    #[doc = "0x08 - Counter count register"]
    pub counter: crate::Reg<self::cnt5::counter::COUNTER_SPEC>,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: crate::Reg<self::cnt5::cc::CC_SPEC>,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: crate::Reg<self::cnt5::cc_buff::CC_BUFF_SPEC>,
    #[doc = "0x14 - Counter period register"]
    pub period: crate::Reg<self::cnt5::period::PERIOD_SPEC>,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: crate::Reg<self::cnt5::period_buff::PERIOD_BUFF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0 Used to select triggers for specific counter events."]
    pub tr_ctrl0: crate::Reg<self::cnt5::tr_ctrl0::TR_CTRL0_SPEC>,
    #[doc = "0x24 - Counter trigger control register 1 Used to determine edge detection for specific counter triggers. Events will only take effect on enabled counters."]
    pub tr_ctrl1: crate::Reg<self::cnt5::tr_ctrl1::TR_CTRL1_SPEC>,
    #[doc = "0x28 - Counter trigger control register 2 Used to control counter \"line_out\", \"dt_line_out\" and \"dt_line_compl_out\" output signals."]
    pub tr_ctrl2: crate::Reg<self::cnt5::tr_ctrl2::TR_CTRL2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register. The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode. HW clears the interrupt causes to '0', when the counter is disabled."]
    pub intr: crate::Reg<self::cnt5::intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt set request register. When read, this register reflects the interrupt request register."]
    pub intr_set: crate::Reg<self::cnt5::intr_set::INTR_SET_SPEC>,
    #[doc = "0x38 - Interrupt mask register."]
    pub intr_mask: crate::Reg<self::cnt5::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x3c - Interrupt masked request register When read, this register reflects a bitwise AND between the interrupt request and mask registers."]
    pub intr_masked: crate::Reg<self::cnt5::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CNT5"]
pub mod cnt5;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TCPWM control register 0. Used to disbale/enable the counters."]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "TCPWM command register. Enables software controlled counter operation."]
pub mod cmd;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "TCPWM Counter interrupt cause register. Enables software to determine the source of the combined interrupt output signal \"interrupt\". The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode."]
pub mod intr_cause;
