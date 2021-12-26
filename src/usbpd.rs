#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - "]
    pub rx_sop_good_crc_en_ctrl: crate::Reg<rx_sop_good_crc_en_ctrl::RX_SOP_GOOD_CRC_EN_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub rx_default_sop_goodcrc_ctrl:
        crate::Reg<rx_default_sop_goodcrc_ctrl::RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>,
    #[doc = "0x10 - "]
    pub rx_prime_sop_goodcrc_ctrl:
        crate::Reg<rx_prime_sop_goodcrc_ctrl::RX_PRIME_SOP_GOODCRC_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub rx_dbl_prime_sop_goodcrc_ctrl:
        crate::Reg<rx_dbl_prime_sop_goodcrc_ctrl::RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>,
    #[doc = "0x18 - "]
    pub rx_expect_goodcrc_msg: crate::Reg<rx_expect_goodcrc_msg::RX_EXPECT_GOODCRC_MSG_SPEC>,
    #[doc = "0x1c - "]
    pub rx_goodcrc_msg: crate::Reg<rx_goodcrc_msg::RX_GOODCRC_MSG_SPEC>,
    #[doc = "0x20 - "]
    pub rx_cc: crate::Reg<rx_cc::RX_CC_SPEC>,
    #[doc = "0x24 - "]
    pub rx_order_set_ctrl: crate::Reg<rx_order_set_ctrl::RX_ORDER_SET_CTRL_SPEC>,
    #[doc = "0x28 - "]
    pub rx_reserved1_order_set: crate::Reg<rx_reserved1_order_set::RX_RESERVED1_ORDER_SET_SPEC>,
    #[doc = "0x2c - "]
    pub rx_reserved2_order_set: crate::Reg<rx_reserved2_order_set::RX_RESERVED2_ORDER_SET_SPEC>,
    #[doc = "0x30 - "]
    pub rx_mem_data0: crate::Reg<rx_mem_data0::RX_MEM_DATA0_SPEC>,
    #[doc = "0x34 - "]
    pub rx_mem_data1: crate::Reg<rx_mem_data1::RX_MEM_DATA1_SPEC>,
    #[doc = "0x38 - "]
    pub rx_mem_data2: crate::Reg<rx_mem_data2::RX_MEM_DATA2_SPEC>,
    #[doc = "0x3c - "]
    pub rx_mem_data3: crate::Reg<rx_mem_data3::RX_MEM_DATA3_SPEC>,
    #[doc = "0x40 - "]
    pub rx_mem_data4: crate::Reg<rx_mem_data4::RX_MEM_DATA4_SPEC>,
    #[doc = "0x44 - "]
    pub rx_mem_data5: crate::Reg<rx_mem_data5::RX_MEM_DATA5_SPEC>,
    #[doc = "0x48 - "]
    pub rx_mem_data6: crate::Reg<rx_mem_data6::RX_MEM_DATA6_SPEC>,
    #[doc = "0x4c - "]
    pub rx_mem_data7: crate::Reg<rx_mem_data7::RX_MEM_DATA7_SPEC>,
    #[doc = "0x50 - "]
    pub tx_mem_data0: crate::Reg<tx_mem_data0::TX_MEM_DATA0_SPEC>,
    #[doc = "0x54 - "]
    pub tx_mem_data1: crate::Reg<tx_mem_data1::TX_MEM_DATA1_SPEC>,
    #[doc = "0x58 - "]
    pub tx_mem_data2: crate::Reg<tx_mem_data2::TX_MEM_DATA2_SPEC>,
    #[doc = "0x5c - "]
    pub tx_mem_data3: crate::Reg<tx_mem_data3::TX_MEM_DATA3_SPEC>,
    #[doc = "0x60 - "]
    pub tx_mem_data4: crate::Reg<tx_mem_data4::TX_MEM_DATA4_SPEC>,
    #[doc = "0x64 - "]
    pub tx_mem_data5: crate::Reg<tx_mem_data5::TX_MEM_DATA5_SPEC>,
    #[doc = "0x68 - "]
    pub tx_mem_data6: crate::Reg<tx_mem_data6::TX_MEM_DATA6_SPEC>,
    #[doc = "0x6c - "]
    pub tx_mem_data7: crate::Reg<tx_mem_data7::TX_MEM_DATA7_SPEC>,
    #[doc = "0x70 - "]
    pub tx_ctrl: crate::Reg<tx_ctrl::TX_CTRL_SPEC>,
    #[doc = "0x74 - "]
    pub tx_sop_order_set: crate::Reg<tx_sop_order_set::TX_SOP_ORDER_SET_SPEC>,
    #[doc = "0x78 - "]
    pub tx_hard_cable_order_set: crate::Reg<tx_hard_cable_order_set::TX_HARD_CABLE_ORDER_SET_SPEC>,
    #[doc = "0x7c - "]
    pub crc_counter: crate::Reg<crc_counter::CRC_COUNTER_SPEC>,
    #[doc = "0x80 - "]
    pub inter_packet_counter: crate::Reg<inter_packet_counter::INTER_PACKET_COUNTER_SPEC>,
    #[doc = "0x84 - "]
    pub timer_trigger: crate::Reg<timer_trigger::TIMER_TRIGGER_SPEC>,
    #[doc = "0x88 - "]
    pub adc_sar_ctrl: crate::Reg<adc_sar_ctrl::ADC_SAR_CTRL_SPEC>,
    #[doc = "0x8c - "]
    pub ddft_mux: crate::Reg<ddft_mux::DDFT_MUX_SPEC>,
    #[doc = "0x90 - "]
    pub intr_0_1_cfg: crate::Reg<intr_0_1_cfg::INTR_0_1_CFG_SPEC>,
    #[doc = "0x94 - "]
    pub intr0: crate::Reg<intr0::INTR0_SPEC>,
    #[doc = "0x98 - "]
    pub intr1: crate::Reg<intr1::INTR1_SPEC>,
    #[doc = "0x9c - "]
    pub intr0_set: crate::Reg<intr0_set::INTR0_SET_SPEC>,
    #[doc = "0xa0 - "]
    pub intr1_set: crate::Reg<intr1_set::INTR1_SET_SPEC>,
    #[doc = "0xa4 - "]
    pub intr0_mask: crate::Reg<intr0_mask::INTR0_MASK_SPEC>,
    #[doc = "0xa8 - "]
    pub intr1_mask: crate::Reg<intr1_mask::INTR1_MASK_SPEC>,
    #[doc = "0xac - "]
    pub intr0_masked: crate::Reg<intr0_masked::INTR0_MASKED_SPEC>,
    #[doc = "0xb0 - "]
    pub intr1_masked: crate::Reg<intr1_masked::INTR1_MASKED_SPEC>,
    #[doc = "0xb4 - "]
    pub debug_ctrl: crate::Reg<debug_ctrl::DEBUG_CTRL_SPEC>,
    #[doc = "0xb8 - "]
    pub debug_cc_0: crate::Reg<debug_cc_0::DEBUG_CC_0_SPEC>,
    #[doc = "0xbc - "]
    pub debug_cc_1: crate::Reg<debug_cc_1::DEBUG_CC_1_SPEC>,
    #[doc = "0xc0 - "]
    pub adc_ctrl: crate::Reg<adc_ctrl::ADC_CTRL_SPEC>,
    #[doc = "0xc4 - "]
    pub cc_ctrl_0: crate::Reg<cc_ctrl_0::CC_CTRL_0_SPEC>,
    #[doc = "0xc8 - "]
    pub cc_ctrl_1: crate::Reg<cc_ctrl_1::CC_CTRL_1_SPEC>,
    #[doc = "0xcc - "]
    pub dpslp_ref_ctrl: crate::Reg<dpslp_ref_ctrl::DPSLP_REF_CTRL_SPEC>,
    #[doc = "0xd0 - "]
    pub vconn_ctrl: crate::Reg<vconn_ctrl::VCONN_CTRL_SPEC>,
    #[doc = "0xd4 - "]
    pub pump_ctrl: crate::Reg<pump_ctrl::PUMP_CTRL_SPEC>,
    _reserved54: [u8; 0x0e28],
    #[doc = "0xf00 - "]
    pub s8usbpd_trim_0: crate::Reg<s8usbpd_trim_0::S8USBPD_TRIM_0_SPEC>,
    #[doc = "0xf04 - "]
    pub s8usbpd_trim_1: crate::Reg<s8usbpd_trim_1::S8USBPD_TRIM_1_SPEC>,
    #[doc = "0xf08 - "]
    pub s8usbpd_trim_2: crate::Reg<s8usbpd_trim_2::S8USBPD_TRIM_2_SPEC>,
    #[doc = "0xf0c - "]
    pub s8usbpd_trim_3: crate::Reg<s8usbpd_trim_3::S8USBPD_TRIM_3_SPEC>,
    #[doc = "0xf10 - "]
    pub s8usbpd_trim_4: crate::Reg<s8usbpd_trim_4::S8USBPD_TRIM_4_SPEC>,
    #[doc = "0xf14 - "]
    pub s8usbpd_trim_5: crate::Reg<s8usbpd_trim_5::S8USBPD_TRIM_5_SPEC>,
    #[doc = "0xf18 - "]
    pub s8usbpd_trim_6: crate::Reg<s8usbpd_trim_6::S8USBPD_TRIM_6_SPEC>,
    #[doc = "0xf1c - "]
    pub s8usbpd_trim_7: crate::Reg<s8usbpd_trim_7::S8USBPD_TRIM_7_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "RX_SOP_GOOD_CRC_EN_CTRL register accessor: an alias for `Reg<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>`"]
pub type RX_SOP_GOOD_CRC_EN_CTRL =
    crate::Reg<rx_sop_good_crc_en_ctrl::RX_SOP_GOOD_CRC_EN_CTRL_SPEC>;
#[doc = ""]
pub mod rx_sop_good_crc_en_ctrl;
#[doc = "RX_DEFAULT_SOP_GOODCRC_CTRL register accessor: an alias for `Reg<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>`"]
pub type RX_DEFAULT_SOP_GOODCRC_CTRL =
    crate::Reg<rx_default_sop_goodcrc_ctrl::RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>;
#[doc = ""]
pub mod rx_default_sop_goodcrc_ctrl;
#[doc = "RX_PRIME_SOP_GOODCRC_CTRL register accessor: an alias for `Reg<RX_PRIME_SOP_GOODCRC_CTRL_SPEC>`"]
pub type RX_PRIME_SOP_GOODCRC_CTRL =
    crate::Reg<rx_prime_sop_goodcrc_ctrl::RX_PRIME_SOP_GOODCRC_CTRL_SPEC>;
#[doc = ""]
pub mod rx_prime_sop_goodcrc_ctrl;
#[doc = "RX_DBL_PRIME_SOP_GOODCRC_CTRL register accessor: an alias for `Reg<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>`"]
pub type RX_DBL_PRIME_SOP_GOODCRC_CTRL =
    crate::Reg<rx_dbl_prime_sop_goodcrc_ctrl::RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>;
#[doc = ""]
pub mod rx_dbl_prime_sop_goodcrc_ctrl;
#[doc = "RX_EXPECT_GOODCRC_MSG register accessor: an alias for `Reg<RX_EXPECT_GOODCRC_MSG_SPEC>`"]
pub type RX_EXPECT_GOODCRC_MSG = crate::Reg<rx_expect_goodcrc_msg::RX_EXPECT_GOODCRC_MSG_SPEC>;
#[doc = ""]
pub mod rx_expect_goodcrc_msg;
#[doc = "RX_GOODCRC_MSG register accessor: an alias for `Reg<RX_GOODCRC_MSG_SPEC>`"]
pub type RX_GOODCRC_MSG = crate::Reg<rx_goodcrc_msg::RX_GOODCRC_MSG_SPEC>;
#[doc = ""]
pub mod rx_goodcrc_msg;
#[doc = "RX_CC register accessor: an alias for `Reg<RX_CC_SPEC>`"]
pub type RX_CC = crate::Reg<rx_cc::RX_CC_SPEC>;
#[doc = ""]
pub mod rx_cc;
#[doc = "RX_ORDER_SET_CTRL register accessor: an alias for `Reg<RX_ORDER_SET_CTRL_SPEC>`"]
pub type RX_ORDER_SET_CTRL = crate::Reg<rx_order_set_ctrl::RX_ORDER_SET_CTRL_SPEC>;
#[doc = ""]
pub mod rx_order_set_ctrl;
#[doc = "RX_RESERVED1_ORDER_SET register accessor: an alias for `Reg<RX_RESERVED1_ORDER_SET_SPEC>`"]
pub type RX_RESERVED1_ORDER_SET = crate::Reg<rx_reserved1_order_set::RX_RESERVED1_ORDER_SET_SPEC>;
#[doc = ""]
pub mod rx_reserved1_order_set;
#[doc = "RX_RESERVED2_ORDER_SET register accessor: an alias for `Reg<RX_RESERVED2_ORDER_SET_SPEC>`"]
pub type RX_RESERVED2_ORDER_SET = crate::Reg<rx_reserved2_order_set::RX_RESERVED2_ORDER_SET_SPEC>;
#[doc = ""]
pub mod rx_reserved2_order_set;
#[doc = "RX_MEM_DATA0 register accessor: an alias for `Reg<RX_MEM_DATA0_SPEC>`"]
pub type RX_MEM_DATA0 = crate::Reg<rx_mem_data0::RX_MEM_DATA0_SPEC>;
#[doc = ""]
pub mod rx_mem_data0;
#[doc = "RX_MEM_DATA1 register accessor: an alias for `Reg<RX_MEM_DATA1_SPEC>`"]
pub type RX_MEM_DATA1 = crate::Reg<rx_mem_data1::RX_MEM_DATA1_SPEC>;
#[doc = ""]
pub mod rx_mem_data1;
#[doc = "RX_MEM_DATA2 register accessor: an alias for `Reg<RX_MEM_DATA2_SPEC>`"]
pub type RX_MEM_DATA2 = crate::Reg<rx_mem_data2::RX_MEM_DATA2_SPEC>;
#[doc = ""]
pub mod rx_mem_data2;
#[doc = "RX_MEM_DATA3 register accessor: an alias for `Reg<RX_MEM_DATA3_SPEC>`"]
pub type RX_MEM_DATA3 = crate::Reg<rx_mem_data3::RX_MEM_DATA3_SPEC>;
#[doc = ""]
pub mod rx_mem_data3;
#[doc = "RX_MEM_DATA4 register accessor: an alias for `Reg<RX_MEM_DATA4_SPEC>`"]
pub type RX_MEM_DATA4 = crate::Reg<rx_mem_data4::RX_MEM_DATA4_SPEC>;
#[doc = ""]
pub mod rx_mem_data4;
#[doc = "RX_MEM_DATA5 register accessor: an alias for `Reg<RX_MEM_DATA5_SPEC>`"]
pub type RX_MEM_DATA5 = crate::Reg<rx_mem_data5::RX_MEM_DATA5_SPEC>;
#[doc = ""]
pub mod rx_mem_data5;
#[doc = "RX_MEM_DATA6 register accessor: an alias for `Reg<RX_MEM_DATA6_SPEC>`"]
pub type RX_MEM_DATA6 = crate::Reg<rx_mem_data6::RX_MEM_DATA6_SPEC>;
#[doc = ""]
pub mod rx_mem_data6;
#[doc = "RX_MEM_DATA7 register accessor: an alias for `Reg<RX_MEM_DATA7_SPEC>`"]
pub type RX_MEM_DATA7 = crate::Reg<rx_mem_data7::RX_MEM_DATA7_SPEC>;
#[doc = ""]
pub mod rx_mem_data7;
#[doc = "TX_MEM_DATA0 register accessor: an alias for `Reg<TX_MEM_DATA0_SPEC>`"]
pub type TX_MEM_DATA0 = crate::Reg<tx_mem_data0::TX_MEM_DATA0_SPEC>;
#[doc = ""]
pub mod tx_mem_data0;
#[doc = "TX_MEM_DATA1 register accessor: an alias for `Reg<TX_MEM_DATA1_SPEC>`"]
pub type TX_MEM_DATA1 = crate::Reg<tx_mem_data1::TX_MEM_DATA1_SPEC>;
#[doc = ""]
pub mod tx_mem_data1;
#[doc = "TX_MEM_DATA2 register accessor: an alias for `Reg<TX_MEM_DATA2_SPEC>`"]
pub type TX_MEM_DATA2 = crate::Reg<tx_mem_data2::TX_MEM_DATA2_SPEC>;
#[doc = ""]
pub mod tx_mem_data2;
#[doc = "TX_MEM_DATA3 register accessor: an alias for `Reg<TX_MEM_DATA3_SPEC>`"]
pub type TX_MEM_DATA3 = crate::Reg<tx_mem_data3::TX_MEM_DATA3_SPEC>;
#[doc = ""]
pub mod tx_mem_data3;
#[doc = "TX_MEM_DATA4 register accessor: an alias for `Reg<TX_MEM_DATA4_SPEC>`"]
pub type TX_MEM_DATA4 = crate::Reg<tx_mem_data4::TX_MEM_DATA4_SPEC>;
#[doc = ""]
pub mod tx_mem_data4;
#[doc = "TX_MEM_DATA5 register accessor: an alias for `Reg<TX_MEM_DATA5_SPEC>`"]
pub type TX_MEM_DATA5 = crate::Reg<tx_mem_data5::TX_MEM_DATA5_SPEC>;
#[doc = ""]
pub mod tx_mem_data5;
#[doc = "TX_MEM_DATA6 register accessor: an alias for `Reg<TX_MEM_DATA6_SPEC>`"]
pub type TX_MEM_DATA6 = crate::Reg<tx_mem_data6::TX_MEM_DATA6_SPEC>;
#[doc = ""]
pub mod tx_mem_data6;
#[doc = "TX_MEM_DATA7 register accessor: an alias for `Reg<TX_MEM_DATA7_SPEC>`"]
pub type TX_MEM_DATA7 = crate::Reg<tx_mem_data7::TX_MEM_DATA7_SPEC>;
#[doc = ""]
pub mod tx_mem_data7;
#[doc = "TX_CTRL register accessor: an alias for `Reg<TX_CTRL_SPEC>`"]
pub type TX_CTRL = crate::Reg<tx_ctrl::TX_CTRL_SPEC>;
#[doc = ""]
pub mod tx_ctrl;
#[doc = "TX_SOP_ORDER_SET register accessor: an alias for `Reg<TX_SOP_ORDER_SET_SPEC>`"]
pub type TX_SOP_ORDER_SET = crate::Reg<tx_sop_order_set::TX_SOP_ORDER_SET_SPEC>;
#[doc = ""]
pub mod tx_sop_order_set;
#[doc = "TX_HARD_CABLE_ORDER_SET register accessor: an alias for `Reg<TX_HARD_CABLE_ORDER_SET_SPEC>`"]
pub type TX_HARD_CABLE_ORDER_SET =
    crate::Reg<tx_hard_cable_order_set::TX_HARD_CABLE_ORDER_SET_SPEC>;
#[doc = ""]
pub mod tx_hard_cable_order_set;
#[doc = "CRC_COUNTER register accessor: an alias for `Reg<CRC_COUNTER_SPEC>`"]
pub type CRC_COUNTER = crate::Reg<crc_counter::CRC_COUNTER_SPEC>;
#[doc = ""]
pub mod crc_counter;
#[doc = "INTER_PACKET_COUNTER register accessor: an alias for `Reg<INTER_PACKET_COUNTER_SPEC>`"]
pub type INTER_PACKET_COUNTER = crate::Reg<inter_packet_counter::INTER_PACKET_COUNTER_SPEC>;
#[doc = ""]
pub mod inter_packet_counter;
#[doc = "TIMER_TRIGGER register accessor: an alias for `Reg<TIMER_TRIGGER_SPEC>`"]
pub type TIMER_TRIGGER = crate::Reg<timer_trigger::TIMER_TRIGGER_SPEC>;
#[doc = ""]
pub mod timer_trigger;
#[doc = "ADC_SAR_CTRL register accessor: an alias for `Reg<ADC_SAR_CTRL_SPEC>`"]
pub type ADC_SAR_CTRL = crate::Reg<adc_sar_ctrl::ADC_SAR_CTRL_SPEC>;
#[doc = ""]
pub mod adc_sar_ctrl;
#[doc = "DDFT_MUX register accessor: an alias for `Reg<DDFT_MUX_SPEC>`"]
pub type DDFT_MUX = crate::Reg<ddft_mux::DDFT_MUX_SPEC>;
#[doc = ""]
pub mod ddft_mux;
#[doc = "INTR_0_1_CFG register accessor: an alias for `Reg<INTR_0_1_CFG_SPEC>`"]
pub type INTR_0_1_CFG = crate::Reg<intr_0_1_cfg::INTR_0_1_CFG_SPEC>;
#[doc = ""]
pub mod intr_0_1_cfg;
#[doc = "INTR0 register accessor: an alias for `Reg<INTR0_SPEC>`"]
pub type INTR0 = crate::Reg<intr0::INTR0_SPEC>;
#[doc = ""]
pub mod intr0;
#[doc = "INTR1 register accessor: an alias for `Reg<INTR1_SPEC>`"]
pub type INTR1 = crate::Reg<intr1::INTR1_SPEC>;
#[doc = ""]
pub mod intr1;
#[doc = "INTR0_SET register accessor: an alias for `Reg<INTR0_SET_SPEC>`"]
pub type INTR0_SET = crate::Reg<intr0_set::INTR0_SET_SPEC>;
#[doc = ""]
pub mod intr0_set;
#[doc = "INTR1_SET register accessor: an alias for `Reg<INTR1_SET_SPEC>`"]
pub type INTR1_SET = crate::Reg<intr1_set::INTR1_SET_SPEC>;
#[doc = ""]
pub mod intr1_set;
#[doc = "INTR0_MASK register accessor: an alias for `Reg<INTR0_MASK_SPEC>`"]
pub type INTR0_MASK = crate::Reg<intr0_mask::INTR0_MASK_SPEC>;
#[doc = ""]
pub mod intr0_mask;
#[doc = "INTR1_MASK register accessor: an alias for `Reg<INTR1_MASK_SPEC>`"]
pub type INTR1_MASK = crate::Reg<intr1_mask::INTR1_MASK_SPEC>;
#[doc = ""]
pub mod intr1_mask;
#[doc = "INTR0_MASKED register accessor: an alias for `Reg<INTR0_MASKED_SPEC>`"]
pub type INTR0_MASKED = crate::Reg<intr0_masked::INTR0_MASKED_SPEC>;
#[doc = ""]
pub mod intr0_masked;
#[doc = "INTR1_MASKED register accessor: an alias for `Reg<INTR1_MASKED_SPEC>`"]
pub type INTR1_MASKED = crate::Reg<intr1_masked::INTR1_MASKED_SPEC>;
#[doc = ""]
pub mod intr1_masked;
#[doc = "DEBUG_CTRL register accessor: an alias for `Reg<DEBUG_CTRL_SPEC>`"]
pub type DEBUG_CTRL = crate::Reg<debug_ctrl::DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod debug_ctrl;
#[doc = "DEBUG_CC_0 register accessor: an alias for `Reg<DEBUG_CC_0_SPEC>`"]
pub type DEBUG_CC_0 = crate::Reg<debug_cc_0::DEBUG_CC_0_SPEC>;
#[doc = ""]
pub mod debug_cc_0;
#[doc = "DEBUG_CC_1 register accessor: an alias for `Reg<DEBUG_CC_1_SPEC>`"]
pub type DEBUG_CC_1 = crate::Reg<debug_cc_1::DEBUG_CC_1_SPEC>;
#[doc = ""]
pub mod debug_cc_1;
#[doc = "ADC_CTRL register accessor: an alias for `Reg<ADC_CTRL_SPEC>`"]
pub type ADC_CTRL = crate::Reg<adc_ctrl::ADC_CTRL_SPEC>;
#[doc = ""]
pub mod adc_ctrl;
#[doc = "CC_CTRL_0 register accessor: an alias for `Reg<CC_CTRL_0_SPEC>`"]
pub type CC_CTRL_0 = crate::Reg<cc_ctrl_0::CC_CTRL_0_SPEC>;
#[doc = ""]
pub mod cc_ctrl_0;
#[doc = "CC_CTRL_1 register accessor: an alias for `Reg<CC_CTRL_1_SPEC>`"]
pub type CC_CTRL_1 = crate::Reg<cc_ctrl_1::CC_CTRL_1_SPEC>;
#[doc = ""]
pub mod cc_ctrl_1;
#[doc = "DPSLP_REF_CTRL register accessor: an alias for `Reg<DPSLP_REF_CTRL_SPEC>`"]
pub type DPSLP_REF_CTRL = crate::Reg<dpslp_ref_ctrl::DPSLP_REF_CTRL_SPEC>;
#[doc = ""]
pub mod dpslp_ref_ctrl;
#[doc = "VCONN_CTRL register accessor: an alias for `Reg<VCONN_CTRL_SPEC>`"]
pub type VCONN_CTRL = crate::Reg<vconn_ctrl::VCONN_CTRL_SPEC>;
#[doc = ""]
pub mod vconn_ctrl;
#[doc = "PUMP_CTRL register accessor: an alias for `Reg<PUMP_CTRL_SPEC>`"]
pub type PUMP_CTRL = crate::Reg<pump_ctrl::PUMP_CTRL_SPEC>;
#[doc = ""]
pub mod pump_ctrl;
#[doc = "S8USBPD_TRIM_0 register accessor: an alias for `Reg<S8USBPD_TRIM_0_SPEC>`"]
pub type S8USBPD_TRIM_0 = crate::Reg<s8usbpd_trim_0::S8USBPD_TRIM_0_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_0;
#[doc = "S8USBPD_TRIM_1 register accessor: an alias for `Reg<S8USBPD_TRIM_1_SPEC>`"]
pub type S8USBPD_TRIM_1 = crate::Reg<s8usbpd_trim_1::S8USBPD_TRIM_1_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_1;
#[doc = "S8USBPD_TRIM_2 register accessor: an alias for `Reg<S8USBPD_TRIM_2_SPEC>`"]
pub type S8USBPD_TRIM_2 = crate::Reg<s8usbpd_trim_2::S8USBPD_TRIM_2_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_2;
#[doc = "S8USBPD_TRIM_3 register accessor: an alias for `Reg<S8USBPD_TRIM_3_SPEC>`"]
pub type S8USBPD_TRIM_3 = crate::Reg<s8usbpd_trim_3::S8USBPD_TRIM_3_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_3;
#[doc = "S8USBPD_TRIM_4 register accessor: an alias for `Reg<S8USBPD_TRIM_4_SPEC>`"]
pub type S8USBPD_TRIM_4 = crate::Reg<s8usbpd_trim_4::S8USBPD_TRIM_4_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_4;
#[doc = "S8USBPD_TRIM_5 register accessor: an alias for `Reg<S8USBPD_TRIM_5_SPEC>`"]
pub type S8USBPD_TRIM_5 = crate::Reg<s8usbpd_trim_5::S8USBPD_TRIM_5_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_5;
#[doc = "S8USBPD_TRIM_6 register accessor: an alias for `Reg<S8USBPD_TRIM_6_SPEC>`"]
pub type S8USBPD_TRIM_6 = crate::Reg<s8usbpd_trim_6::S8USBPD_TRIM_6_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_6;
#[doc = "S8USBPD_TRIM_7 register accessor: an alias for `Reg<S8USBPD_TRIM_7_SPEC>`"]
pub type S8USBPD_TRIM_7 = crate::Reg<s8usbpd_trim_7::S8USBPD_TRIM_7_SPEC>;
#[doc = ""]
pub mod s8usbpd_trim_7;
