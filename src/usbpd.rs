#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Generic control register."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Generic status register."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - RX SOP Control for sending GoodCRC Message Hardware will wait for programmable IDLE_COUNTER and then send Good Crc Message."]
    pub rx_sop_good_crc_en_ctrl: crate::Reg<rx_sop_good_crc_en_ctrl::RX_SOP_GOOD_CRC_EN_CTRL_SPEC>,
    #[doc = "0x0c - RX Default SOP GoodCRC Control There are two purposes for this register: 1. Transmit Path: When hardware is done transmitting a packet, it will start the RX_CRC_TIMER. The CRC timer should stop on following conditions: 1. Good CRC Received: With Matching Message ID, Matching Header Sop type. 2. Hard Reset on Receive side 3. Soft Reset on Receive Side: 4. Any other message legal in the current firmware state: Condition 1, condition 2: These two conditions are automatically taken care by hardware and CRC timer is stopped. Condition 3, condition 4: Firmware needs to take care of. Firmware can program what legal messages it is expecting in a particular state and hardware will stop its counter. E.G: If firmware wants the transmit logic to stop its CRC counter and not retry the packet on reception of these following messages: � Soft Reset Control Message: Message Type 1101 � Get Source Cap Control Message: Message Type 0111 � Vendor Defined Data Message: Message Type 1111 Then in that case firmware will program RX_DEFAULT_SOP_GOODCRC_CTRL to: 8000_2080 (Bit 7th, bit 13th and bit 31st) Hardware will stop the timers on reception of these packets and will also automatically send GoodCRC message to these messages if the proper auto bit is set in RX_SOP_GOOD_CRC_EN_CTRL. Other messages received will be logged in RX_Memory but Good CRC will not be returned or timer will not be stopped. 2. Receive Path: Based on one hot encoding of RX_DEFAULT_SOP_GOODCRC_CTRL mapped to message type field in the incoming header, Good CRC will be returned automatically if the correct RX_SOP_GOOD_CRC_EN_CTRL bit is set."]
    pub rx_default_sop_goodcrc_ctrl:
        crate::Reg<rx_default_sop_goodcrc_ctrl::RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>,
    #[doc = "0x10 - RX Prime SOP GoodCRC Control"]
    pub rx_prime_sop_goodcrc_ctrl:
        crate::Reg<rx_prime_sop_goodcrc_ctrl::RX_PRIME_SOP_GOODCRC_CTRL_SPEC>,
    #[doc = "0x14 - RX DBL Prime SOP GoodCRC Control"]
    pub rx_dbl_prime_sop_goodcrc_ctrl:
        crate::Reg<rx_dbl_prime_sop_goodcrc_ctrl::RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>,
    #[doc = "0x18 - RX Excepted good CRC message to stop the CRC timers"]
    pub rx_expect_goodcrc_msg: crate::Reg<rx_expect_goodcrc_msg::RX_EXPECT_GOODCRC_MSG_SPEC>,
    #[doc = "0x1c - The 2-Byte Header of the received GoodCRC Message"]
    pub rx_goodcrc_msg: crate::Reg<rx_goodcrc_msg::RX_GOODCRC_MSG_SPEC>,
    #[doc = "0x20 - The Receive C-Connect registers"]
    pub rx_cc: crate::Reg<rx_cc::RX_CC_SPEC>,
    #[doc = "0x24 - Receive SOPs and RSTs order set control"]
    pub rx_order_set_ctrl: crate::Reg<rx_order_set_ctrl::RX_ORDER_SET_CTRL_SPEC>,
    #[doc = "0x28 - Receive Reserved1 order set"]
    pub rx_reserved1_order_set: crate::Reg<rx_reserved1_order_set::RX_RESERVED1_ORDER_SET_SPEC>,
    #[doc = "0x2c - Receive Reserved2 order set"]
    pub rx_reserved2_order_set: crate::Reg<rx_reserved2_order_set::RX_RESERVED2_ORDER_SET_SPEC>,
    #[doc = "0x30 - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data0: crate::Reg<rx_mem_data0::RX_MEM_DATA0_SPEC>,
    #[doc = "0x34 - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data1: crate::Reg<rx_mem_data1::RX_MEM_DATA1_SPEC>,
    #[doc = "0x38 - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data2: crate::Reg<rx_mem_data2::RX_MEM_DATA2_SPEC>,
    #[doc = "0x3c - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data3: crate::Reg<rx_mem_data3::RX_MEM_DATA3_SPEC>,
    #[doc = "0x40 - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data4: crate::Reg<rx_mem_data4::RX_MEM_DATA4_SPEC>,
    #[doc = "0x44 - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data5: crate::Reg<rx_mem_data5::RX_MEM_DATA5_SPEC>,
    #[doc = "0x48 - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data6: crate::Reg<rx_mem_data6::RX_MEM_DATA6_SPEC>,
    #[doc = "0x4c - RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
    pub rx_mem_data7: crate::Reg<rx_mem_data7::RX_MEM_DATA7_SPEC>,
    #[doc = "0x50 - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data0: crate::Reg<tx_mem_data0::TX_MEM_DATA0_SPEC>,
    #[doc = "0x54 - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data1: crate::Reg<tx_mem_data1::TX_MEM_DATA1_SPEC>,
    #[doc = "0x58 - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data2: crate::Reg<tx_mem_data2::TX_MEM_DATA2_SPEC>,
    #[doc = "0x5c - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data3: crate::Reg<tx_mem_data3::TX_MEM_DATA3_SPEC>,
    #[doc = "0x60 - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data4: crate::Reg<tx_mem_data4::TX_MEM_DATA4_SPEC>,
    #[doc = "0x64 - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data5: crate::Reg<tx_mem_data5::TX_MEM_DATA5_SPEC>,
    #[doc = "0x68 - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data6: crate::Reg<tx_mem_data6::TX_MEM_DATA6_SPEC>,
    #[doc = "0x6c - TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
    pub tx_mem_data7: crate::Reg<tx_mem_data7::TX_MEM_DATA7_SPEC>,
    #[doc = "0x70 - TX Control"]
    pub tx_ctrl: crate::Reg<tx_ctrl::TX_CTRL_SPEC>,
    #[doc = "0x74 - Transmit SOP order set"]
    pub tx_sop_order_set: crate::Reg<tx_sop_order_set::TX_SOP_ORDER_SET_SPEC>,
    #[doc = "0x78 - Transmit Hard/Cable reset order set"]
    pub tx_hard_cable_order_set: crate::Reg<tx_hard_cable_order_set::TX_HARD_CABLE_ORDER_SET_SPEC>,
    #[doc = "0x7c - The CRC timer counters/Bypass configuration Counters used for the timmers needed by this IP"]
    pub crc_counter: crate::Reg<crc_counter::CRC_COUNTER_SPEC>,
    #[doc = "0x80 - The Inter Packet counters Counters used for IDLE/IFG and by this IP All the timers/counters have a resolution of 1 UI (Unit Interval) of a Bit. If transmit rate is 300Khz, then each count will tick for 3.33 usec."]
    pub inter_packet_counter: crate::Reg<inter_packet_counter::INTER_PACKET_COUNTER_SPEC>,
    #[doc = "0x84 - The trigger enable registers The tr_out\\[4:0\\]
pins of this IP will be connected to the tr_in pin of m0s8tcpwm_ver2 IP at the full chip. The mapping of the these signals is SoC depended and it is defined in SAS."]
    pub timer_trigger: crate::Reg<timer_trigger::TIMER_TRIGGER_SPEC>,
    #[doc = "0x88 - ADC SAR Control Register General Purpose voltgae measurement, Temperature Sceining"]
    pub adc_sar_ctrl: crate::Reg<adc_sar_ctrl::ADC_SAR_CTRL_SPEC>,
    #[doc = "0x8c - DDFT Selections"]
    pub ddft_mux: crate::Reg<ddft_mux::DDFT_MUX_SPEC>,
    #[doc = "0x90 - Wakeup Interrupts edge and filter configuration and Intr0 configuration"]
    pub intr_0_1_cfg: crate::Reg<intr_0_1_cfg::INTR_0_1_CFG_SPEC>,
    #[doc = "0x94 - INTR0 Cause"]
    pub intr0: crate::Reg<intr0::INTR0_SPEC>,
    #[doc = "0x98 - INTR1 Cause The configurations for using the comparators: DFP waiting for attach: vcmp_up connected to CC1: HI = no connect, LO = attach vcmp_dn connected to CC2: HI = no connect, LO = attach DFP after attach: vcmup_up at detach threshold: HI = detach, LO = attach vcmp_dn at Rd/Ra threshold: HI = Rd connected, LO = Ra connected vcmp_la at CC line activity threshold: HI = no activity, LO/Toggling = activity UFP (with VBUS present): vcmup_up at Default/1.5A threshold: HI = Default, LO = 1.5A vcmp_dn at 1.5A/3.0A threshold: HI = 1.5A, LO = 3.0A vcmp_la at CC line activity threshold: HI = no activity, LO/Toggling = activity For detecting the difference between Rd/Ra, firmware will have to check the �DFP after attach� state above to determine it."]
    pub intr1: crate::Reg<intr1::INTR1_SPEC>,
    #[doc = "0x9c - INTR0 Set"]
    pub intr0_set: crate::Reg<intr0_set::INTR0_SET_SPEC>,
    #[doc = "0xa0 - INTR1 Set"]
    pub intr1_set: crate::Reg<intr1_set::INTR1_SET_SPEC>,
    #[doc = "0xa4 - INTR0 Mask"]
    pub intr0_mask: crate::Reg<intr0_mask::INTR0_MASK_SPEC>,
    #[doc = "0xa8 - INTR1 Mask"]
    pub intr1_mask: crate::Reg<intr1_mask::INTR1_MASK_SPEC>,
    #[doc = "0xac - INTR0 Masked"]
    pub intr0_masked: crate::Reg<intr0_masked::INTR0_MASKED_SPEC>,
    #[doc = "0xb0 - INTR1 Masked"]
    pub intr1_masked: crate::Reg<intr1_masked::INTR1_MASKED_SPEC>,
    #[doc = "0xb4 - Debug Control Register"]
    pub debug_ctrl: crate::Reg<debug_ctrl::DEBUG_CTRL_SPEC>,
    #[doc = "0xb8 - C-Connector Debug control register 0"]
    pub debug_cc_0: crate::Reg<debug_cc_0::DEBUG_CC_0_SPEC>,
    #[doc = "0xbc - C-Connector Debug control register 1"]
    pub debug_cc_1: crate::Reg<debug_cc_1::DEBUG_CC_1_SPEC>,
    #[doc = "0xc0 - S8USBPD DAC Control Register"]
    pub adc_ctrl: crate::Reg<adc_ctrl::ADC_CTRL_SPEC>,
    #[doc = "0xc4 - S8USBPD C-connector Control Register 0"]
    pub cc_ctrl_0: crate::Reg<cc_ctrl_0::CC_CTRL_0_SPEC>,
    #[doc = "0xc8 - S8USBPD C-connector Control Register 1"]
    pub cc_ctrl_1: crate::Reg<cc_ctrl_1::CC_CTRL_1_SPEC>,
    #[doc = "0xcc - S8USBPD DeepSleep-Reference Control Register"]
    pub dpslp_ref_ctrl: crate::Reg<dpslp_ref_ctrl::DPSLP_REF_CTRL_SPEC>,
    #[doc = "0xd0 - S8USBPD VCONN control Register"]
    pub vconn_ctrl: crate::Reg<vconn_ctrl::VCONN_CTRL_SPEC>,
    #[doc = "0xd4 - S8USBPD PUMP control Register"]
    pub pump_ctrl: crate::Reg<pump_ctrl::PUMP_CTRL_SPEC>,
    _reserved54: [u8; 0x0e28],
    #[doc = "0xf00 - S8USBPD Trim Register0 . Production trims stored in flash"]
    pub s8usbpd_trim_0: crate::Reg<s8usbpd_trim_0::S8USBPD_TRIM_0_SPEC>,
    #[doc = "0xf04 - S8USBPD Trim Register1 . Production trims stored in flash"]
    pub s8usbpd_trim_1: crate::Reg<s8usbpd_trim_1::S8USBPD_TRIM_1_SPEC>,
    #[doc = "0xf08 - S8USBPD Trim Register2 . Production trims stored in flash"]
    pub s8usbpd_trim_2: crate::Reg<s8usbpd_trim_2::S8USBPD_TRIM_2_SPEC>,
    #[doc = "0xf0c - S8USBPD C-connector Trim Register3. Production trims stored in flash"]
    pub s8usbpd_trim_3: crate::Reg<s8usbpd_trim_3::S8USBPD_TRIM_3_SPEC>,
    #[doc = "0xf10 - S8USBPD C-connector Trim Register4. Production trims stored in flash"]
    pub s8usbpd_trim_4: crate::Reg<s8usbpd_trim_4::S8USBPD_TRIM_4_SPEC>,
    #[doc = "0xf14 - S8USBPD C-connector Trim Register5. Production trims stored in flash"]
    pub s8usbpd_trim_5: crate::Reg<s8usbpd_trim_5::S8USBPD_TRIM_5_SPEC>,
    #[doc = "0xf18 - S8USBPD C-connector Trim Register6. Production trims stored in flash"]
    pub s8usbpd_trim_6: crate::Reg<s8usbpd_trim_6::S8USBPD_TRIM_6_SPEC>,
    #[doc = "0xf1c - S8USBPD C-connector Trim Register7. Production trims stored in flash"]
    pub s8usbpd_trim_7: crate::Reg<s8usbpd_trim_7::S8USBPD_TRIM_7_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Generic control register."]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Generic status register."]
pub mod status;
#[doc = "RX_SOP_GOOD_CRC_EN_CTRL register accessor: an alias for `Reg<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>`"]
pub type RX_SOP_GOOD_CRC_EN_CTRL =
    crate::Reg<rx_sop_good_crc_en_ctrl::RX_SOP_GOOD_CRC_EN_CTRL_SPEC>;
#[doc = "RX SOP Control for sending GoodCRC Message Hardware will wait for programmable IDLE_COUNTER and then send Good Crc Message."]
pub mod rx_sop_good_crc_en_ctrl;
#[doc = "RX_DEFAULT_SOP_GOODCRC_CTRL register accessor: an alias for `Reg<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>`"]
pub type RX_DEFAULT_SOP_GOODCRC_CTRL =
    crate::Reg<rx_default_sop_goodcrc_ctrl::RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>;
#[doc = "RX Default SOP GoodCRC Control There are two purposes for this register: 1. Transmit Path: When hardware is done transmitting a packet, it will start the RX_CRC_TIMER. The CRC timer should stop on following conditions: 1. Good CRC Received: With Matching Message ID, Matching Header Sop type. 2. Hard Reset on Receive side 3. Soft Reset on Receive Side: 4. Any other message legal in the current firmware state: Condition 1, condition 2: These two conditions are automatically taken care by hardware and CRC timer is stopped. Condition 3, condition 4: Firmware needs to take care of. Firmware can program what legal messages it is expecting in a particular state and hardware will stop its counter. E.G: If firmware wants the transmit logic to stop its CRC counter and not retry the packet on reception of these following messages: � Soft Reset Control Message: Message Type 1101 � Get Source Cap Control Message: Message Type 0111 � Vendor Defined Data Message: Message Type 1111 Then in that case firmware will program RX_DEFAULT_SOP_GOODCRC_CTRL to: 8000_2080 (Bit 7th, bit 13th and bit 31st) Hardware will stop the timers on reception of these packets and will also automatically send GoodCRC message to these messages if the proper auto bit is set in RX_SOP_GOOD_CRC_EN_CTRL. Other messages received will be logged in RX_Memory but Good CRC will not be returned or timer will not be stopped. 2. Receive Path: Based on one hot encoding of RX_DEFAULT_SOP_GOODCRC_CTRL mapped to message type field in the incoming header, Good CRC will be returned automatically if the correct RX_SOP_GOOD_CRC_EN_CTRL bit is set."]
pub mod rx_default_sop_goodcrc_ctrl;
#[doc = "RX_PRIME_SOP_GOODCRC_CTRL register accessor: an alias for `Reg<RX_PRIME_SOP_GOODCRC_CTRL_SPEC>`"]
pub type RX_PRIME_SOP_GOODCRC_CTRL =
    crate::Reg<rx_prime_sop_goodcrc_ctrl::RX_PRIME_SOP_GOODCRC_CTRL_SPEC>;
#[doc = "RX Prime SOP GoodCRC Control"]
pub mod rx_prime_sop_goodcrc_ctrl;
#[doc = "RX_DBL_PRIME_SOP_GOODCRC_CTRL register accessor: an alias for `Reg<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>`"]
pub type RX_DBL_PRIME_SOP_GOODCRC_CTRL =
    crate::Reg<rx_dbl_prime_sop_goodcrc_ctrl::RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>;
#[doc = "RX DBL Prime SOP GoodCRC Control"]
pub mod rx_dbl_prime_sop_goodcrc_ctrl;
#[doc = "RX_EXPECT_GOODCRC_MSG register accessor: an alias for `Reg<RX_EXPECT_GOODCRC_MSG_SPEC>`"]
pub type RX_EXPECT_GOODCRC_MSG = crate::Reg<rx_expect_goodcrc_msg::RX_EXPECT_GOODCRC_MSG_SPEC>;
#[doc = "RX Excepted good CRC message to stop the CRC timers"]
pub mod rx_expect_goodcrc_msg;
#[doc = "RX_GOODCRC_MSG register accessor: an alias for `Reg<RX_GOODCRC_MSG_SPEC>`"]
pub type RX_GOODCRC_MSG = crate::Reg<rx_goodcrc_msg::RX_GOODCRC_MSG_SPEC>;
#[doc = "The 2-Byte Header of the received GoodCRC Message"]
pub mod rx_goodcrc_msg;
#[doc = "RX_CC register accessor: an alias for `Reg<RX_CC_SPEC>`"]
pub type RX_CC = crate::Reg<rx_cc::RX_CC_SPEC>;
#[doc = "The Receive C-Connect registers"]
pub mod rx_cc;
#[doc = "RX_ORDER_SET_CTRL register accessor: an alias for `Reg<RX_ORDER_SET_CTRL_SPEC>`"]
pub type RX_ORDER_SET_CTRL = crate::Reg<rx_order_set_ctrl::RX_ORDER_SET_CTRL_SPEC>;
#[doc = "Receive SOPs and RSTs order set control"]
pub mod rx_order_set_ctrl;
#[doc = "RX_RESERVED1_ORDER_SET register accessor: an alias for `Reg<RX_RESERVED1_ORDER_SET_SPEC>`"]
pub type RX_RESERVED1_ORDER_SET = crate::Reg<rx_reserved1_order_set::RX_RESERVED1_ORDER_SET_SPEC>;
#[doc = "Receive Reserved1 order set"]
pub mod rx_reserved1_order_set;
#[doc = "RX_RESERVED2_ORDER_SET register accessor: an alias for `Reg<RX_RESERVED2_ORDER_SET_SPEC>`"]
pub type RX_RESERVED2_ORDER_SET = crate::Reg<rx_reserved2_order_set::RX_RESERVED2_ORDER_SET_SPEC>;
#[doc = "Receive Reserved2 order set"]
pub mod rx_reserved2_order_set;
#[doc = "RX_MEM_DATA0 register accessor: an alias for `Reg<RX_MEM_DATA0_SPEC>`"]
pub type RX_MEM_DATA0 = crate::Reg<rx_mem_data0::RX_MEM_DATA0_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data0;
#[doc = "RX_MEM_DATA1 register accessor: an alias for `Reg<RX_MEM_DATA1_SPEC>`"]
pub type RX_MEM_DATA1 = crate::Reg<rx_mem_data1::RX_MEM_DATA1_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data1;
#[doc = "RX_MEM_DATA2 register accessor: an alias for `Reg<RX_MEM_DATA2_SPEC>`"]
pub type RX_MEM_DATA2 = crate::Reg<rx_mem_data2::RX_MEM_DATA2_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data2;
#[doc = "RX_MEM_DATA3 register accessor: an alias for `Reg<RX_MEM_DATA3_SPEC>`"]
pub type RX_MEM_DATA3 = crate::Reg<rx_mem_data3::RX_MEM_DATA3_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data3;
#[doc = "RX_MEM_DATA4 register accessor: an alias for `Reg<RX_MEM_DATA4_SPEC>`"]
pub type RX_MEM_DATA4 = crate::Reg<rx_mem_data4::RX_MEM_DATA4_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data4;
#[doc = "RX_MEM_DATA5 register accessor: an alias for `Reg<RX_MEM_DATA5_SPEC>`"]
pub type RX_MEM_DATA5 = crate::Reg<rx_mem_data5::RX_MEM_DATA5_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data5;
#[doc = "RX_MEM_DATA6 register accessor: an alias for `Reg<RX_MEM_DATA6_SPEC>`"]
pub type RX_MEM_DATA6 = crate::Reg<rx_mem_data6::RX_MEM_DATA6_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data6;
#[doc = "RX_MEM_DATA7 register accessor: an alias for `Reg<RX_MEM_DATA7_SPEC>`"]
pub type RX_MEM_DATA7 = crate::Reg<rx_mem_data7::RX_MEM_DATA7_SPEC>;
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15"]
pub mod rx_mem_data7;
#[doc = "TX_MEM_DATA0 register accessor: an alias for `Reg<TX_MEM_DATA0_SPEC>`"]
pub type TX_MEM_DATA0 = crate::Reg<tx_mem_data0::TX_MEM_DATA0_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data0;
#[doc = "TX_MEM_DATA1 register accessor: an alias for `Reg<TX_MEM_DATA1_SPEC>`"]
pub type TX_MEM_DATA1 = crate::Reg<tx_mem_data1::TX_MEM_DATA1_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data1;
#[doc = "TX_MEM_DATA2 register accessor: an alias for `Reg<TX_MEM_DATA2_SPEC>`"]
pub type TX_MEM_DATA2 = crate::Reg<tx_mem_data2::TX_MEM_DATA2_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data2;
#[doc = "TX_MEM_DATA3 register accessor: an alias for `Reg<TX_MEM_DATA3_SPEC>`"]
pub type TX_MEM_DATA3 = crate::Reg<tx_mem_data3::TX_MEM_DATA3_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data3;
#[doc = "TX_MEM_DATA4 register accessor: an alias for `Reg<TX_MEM_DATA4_SPEC>`"]
pub type TX_MEM_DATA4 = crate::Reg<tx_mem_data4::TX_MEM_DATA4_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data4;
#[doc = "TX_MEM_DATA5 register accessor: an alias for `Reg<TX_MEM_DATA5_SPEC>`"]
pub type TX_MEM_DATA5 = crate::Reg<tx_mem_data5::TX_MEM_DATA5_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data5;
#[doc = "TX_MEM_DATA6 register accessor: an alias for `Reg<TX_MEM_DATA6_SPEC>`"]
pub type TX_MEM_DATA6 = crate::Reg<tx_mem_data6::TX_MEM_DATA6_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data6;
#[doc = "TX_MEM_DATA7 register accessor: an alias for `Reg<TX_MEM_DATA7_SPEC>`"]
pub type TX_MEM_DATA7 = crate::Reg<tx_mem_data7::TX_MEM_DATA7_SPEC>;
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15"]
pub mod tx_mem_data7;
#[doc = "TX_CTRL register accessor: an alias for `Reg<TX_CTRL_SPEC>`"]
pub type TX_CTRL = crate::Reg<tx_ctrl::TX_CTRL_SPEC>;
#[doc = "TX Control"]
pub mod tx_ctrl;
#[doc = "TX_SOP_ORDER_SET register accessor: an alias for `Reg<TX_SOP_ORDER_SET_SPEC>`"]
pub type TX_SOP_ORDER_SET = crate::Reg<tx_sop_order_set::TX_SOP_ORDER_SET_SPEC>;
#[doc = "Transmit SOP order set"]
pub mod tx_sop_order_set;
#[doc = "TX_HARD_CABLE_ORDER_SET register accessor: an alias for `Reg<TX_HARD_CABLE_ORDER_SET_SPEC>`"]
pub type TX_HARD_CABLE_ORDER_SET =
    crate::Reg<tx_hard_cable_order_set::TX_HARD_CABLE_ORDER_SET_SPEC>;
#[doc = "Transmit Hard/Cable reset order set"]
pub mod tx_hard_cable_order_set;
#[doc = "CRC_COUNTER register accessor: an alias for `Reg<CRC_COUNTER_SPEC>`"]
pub type CRC_COUNTER = crate::Reg<crc_counter::CRC_COUNTER_SPEC>;
#[doc = "The CRC timer counters/Bypass configuration Counters used for the timmers needed by this IP"]
pub mod crc_counter;
#[doc = "INTER_PACKET_COUNTER register accessor: an alias for `Reg<INTER_PACKET_COUNTER_SPEC>`"]
pub type INTER_PACKET_COUNTER = crate::Reg<inter_packet_counter::INTER_PACKET_COUNTER_SPEC>;
#[doc = "The Inter Packet counters Counters used for IDLE/IFG and by this IP All the timers/counters have a resolution of 1 UI (Unit Interval) of a Bit. If transmit rate is 300Khz, then each count will tick for 3.33 usec."]
pub mod inter_packet_counter;
#[doc = "TIMER_TRIGGER register accessor: an alias for `Reg<TIMER_TRIGGER_SPEC>`"]
pub type TIMER_TRIGGER = crate::Reg<timer_trigger::TIMER_TRIGGER_SPEC>;
#[doc = "The trigger enable registers The tr_out\\[4:0\\]
pins of this IP will be connected to the tr_in pin of m0s8tcpwm_ver2 IP at the full chip. The mapping of the these signals is SoC depended and it is defined in SAS."]
pub mod timer_trigger;
#[doc = "ADC_SAR_CTRL register accessor: an alias for `Reg<ADC_SAR_CTRL_SPEC>`"]
pub type ADC_SAR_CTRL = crate::Reg<adc_sar_ctrl::ADC_SAR_CTRL_SPEC>;
#[doc = "ADC SAR Control Register General Purpose voltgae measurement, Temperature Sceining"]
pub mod adc_sar_ctrl;
#[doc = "DDFT_MUX register accessor: an alias for `Reg<DDFT_MUX_SPEC>`"]
pub type DDFT_MUX = crate::Reg<ddft_mux::DDFT_MUX_SPEC>;
#[doc = "DDFT Selections"]
pub mod ddft_mux;
#[doc = "INTR_0_1_CFG register accessor: an alias for `Reg<INTR_0_1_CFG_SPEC>`"]
pub type INTR_0_1_CFG = crate::Reg<intr_0_1_cfg::INTR_0_1_CFG_SPEC>;
#[doc = "Wakeup Interrupts edge and filter configuration and Intr0 configuration"]
pub mod intr_0_1_cfg;
#[doc = "INTR0 register accessor: an alias for `Reg<INTR0_SPEC>`"]
pub type INTR0 = crate::Reg<intr0::INTR0_SPEC>;
#[doc = "INTR0 Cause"]
pub mod intr0;
#[doc = "INTR1 register accessor: an alias for `Reg<INTR1_SPEC>`"]
pub type INTR1 = crate::Reg<intr1::INTR1_SPEC>;
#[doc = "INTR1 Cause The configurations for using the comparators: DFP waiting for attach: vcmp_up connected to CC1: HI = no connect, LO = attach vcmp_dn connected to CC2: HI = no connect, LO = attach DFP after attach: vcmup_up at detach threshold: HI = detach, LO = attach vcmp_dn at Rd/Ra threshold: HI = Rd connected, LO = Ra connected vcmp_la at CC line activity threshold: HI = no activity, LO/Toggling = activity UFP (with VBUS present): vcmup_up at Default/1.5A threshold: HI = Default, LO = 1.5A vcmp_dn at 1.5A/3.0A threshold: HI = 1.5A, LO = 3.0A vcmp_la at CC line activity threshold: HI = no activity, LO/Toggling = activity For detecting the difference between Rd/Ra, firmware will have to check the �DFP after attach� state above to determine it."]
pub mod intr1;
#[doc = "INTR0_SET register accessor: an alias for `Reg<INTR0_SET_SPEC>`"]
pub type INTR0_SET = crate::Reg<intr0_set::INTR0_SET_SPEC>;
#[doc = "INTR0 Set"]
pub mod intr0_set;
#[doc = "INTR1_SET register accessor: an alias for `Reg<INTR1_SET_SPEC>`"]
pub type INTR1_SET = crate::Reg<intr1_set::INTR1_SET_SPEC>;
#[doc = "INTR1 Set"]
pub mod intr1_set;
#[doc = "INTR0_MASK register accessor: an alias for `Reg<INTR0_MASK_SPEC>`"]
pub type INTR0_MASK = crate::Reg<intr0_mask::INTR0_MASK_SPEC>;
#[doc = "INTR0 Mask"]
pub mod intr0_mask;
#[doc = "INTR1_MASK register accessor: an alias for `Reg<INTR1_MASK_SPEC>`"]
pub type INTR1_MASK = crate::Reg<intr1_mask::INTR1_MASK_SPEC>;
#[doc = "INTR1 Mask"]
pub mod intr1_mask;
#[doc = "INTR0_MASKED register accessor: an alias for `Reg<INTR0_MASKED_SPEC>`"]
pub type INTR0_MASKED = crate::Reg<intr0_masked::INTR0_MASKED_SPEC>;
#[doc = "INTR0 Masked"]
pub mod intr0_masked;
#[doc = "INTR1_MASKED register accessor: an alias for `Reg<INTR1_MASKED_SPEC>`"]
pub type INTR1_MASKED = crate::Reg<intr1_masked::INTR1_MASKED_SPEC>;
#[doc = "INTR1 Masked"]
pub mod intr1_masked;
#[doc = "DEBUG_CTRL register accessor: an alias for `Reg<DEBUG_CTRL_SPEC>`"]
pub type DEBUG_CTRL = crate::Reg<debug_ctrl::DEBUG_CTRL_SPEC>;
#[doc = "Debug Control Register"]
pub mod debug_ctrl;
#[doc = "DEBUG_CC_0 register accessor: an alias for `Reg<DEBUG_CC_0_SPEC>`"]
pub type DEBUG_CC_0 = crate::Reg<debug_cc_0::DEBUG_CC_0_SPEC>;
#[doc = "C-Connector Debug control register 0"]
pub mod debug_cc_0;
#[doc = "DEBUG_CC_1 register accessor: an alias for `Reg<DEBUG_CC_1_SPEC>`"]
pub type DEBUG_CC_1 = crate::Reg<debug_cc_1::DEBUG_CC_1_SPEC>;
#[doc = "C-Connector Debug control register 1"]
pub mod debug_cc_1;
#[doc = "ADC_CTRL register accessor: an alias for `Reg<ADC_CTRL_SPEC>`"]
pub type ADC_CTRL = crate::Reg<adc_ctrl::ADC_CTRL_SPEC>;
#[doc = "S8USBPD DAC Control Register"]
pub mod adc_ctrl;
#[doc = "CC_CTRL_0 register accessor: an alias for `Reg<CC_CTRL_0_SPEC>`"]
pub type CC_CTRL_0 = crate::Reg<cc_ctrl_0::CC_CTRL_0_SPEC>;
#[doc = "S8USBPD C-connector Control Register 0"]
pub mod cc_ctrl_0;
#[doc = "CC_CTRL_1 register accessor: an alias for `Reg<CC_CTRL_1_SPEC>`"]
pub type CC_CTRL_1 = crate::Reg<cc_ctrl_1::CC_CTRL_1_SPEC>;
#[doc = "S8USBPD C-connector Control Register 1"]
pub mod cc_ctrl_1;
#[doc = "DPSLP_REF_CTRL register accessor: an alias for `Reg<DPSLP_REF_CTRL_SPEC>`"]
pub type DPSLP_REF_CTRL = crate::Reg<dpslp_ref_ctrl::DPSLP_REF_CTRL_SPEC>;
#[doc = "S8USBPD DeepSleep-Reference Control Register"]
pub mod dpslp_ref_ctrl;
#[doc = "VCONN_CTRL register accessor: an alias for `Reg<VCONN_CTRL_SPEC>`"]
pub type VCONN_CTRL = crate::Reg<vconn_ctrl::VCONN_CTRL_SPEC>;
#[doc = "S8USBPD VCONN control Register"]
pub mod vconn_ctrl;
#[doc = "PUMP_CTRL register accessor: an alias for `Reg<PUMP_CTRL_SPEC>`"]
pub type PUMP_CTRL = crate::Reg<pump_ctrl::PUMP_CTRL_SPEC>;
#[doc = "S8USBPD PUMP control Register"]
pub mod pump_ctrl;
#[doc = "S8USBPD_TRIM_0 register accessor: an alias for `Reg<S8USBPD_TRIM_0_SPEC>`"]
pub type S8USBPD_TRIM_0 = crate::Reg<s8usbpd_trim_0::S8USBPD_TRIM_0_SPEC>;
#[doc = "S8USBPD Trim Register0 . Production trims stored in flash"]
pub mod s8usbpd_trim_0;
#[doc = "S8USBPD_TRIM_1 register accessor: an alias for `Reg<S8USBPD_TRIM_1_SPEC>`"]
pub type S8USBPD_TRIM_1 = crate::Reg<s8usbpd_trim_1::S8USBPD_TRIM_1_SPEC>;
#[doc = "S8USBPD Trim Register1 . Production trims stored in flash"]
pub mod s8usbpd_trim_1;
#[doc = "S8USBPD_TRIM_2 register accessor: an alias for `Reg<S8USBPD_TRIM_2_SPEC>`"]
pub type S8USBPD_TRIM_2 = crate::Reg<s8usbpd_trim_2::S8USBPD_TRIM_2_SPEC>;
#[doc = "S8USBPD Trim Register2 . Production trims stored in flash"]
pub mod s8usbpd_trim_2;
#[doc = "S8USBPD_TRIM_3 register accessor: an alias for `Reg<S8USBPD_TRIM_3_SPEC>`"]
pub type S8USBPD_TRIM_3 = crate::Reg<s8usbpd_trim_3::S8USBPD_TRIM_3_SPEC>;
#[doc = "S8USBPD C-connector Trim Register3. Production trims stored in flash"]
pub mod s8usbpd_trim_3;
#[doc = "S8USBPD_TRIM_4 register accessor: an alias for `Reg<S8USBPD_TRIM_4_SPEC>`"]
pub type S8USBPD_TRIM_4 = crate::Reg<s8usbpd_trim_4::S8USBPD_TRIM_4_SPEC>;
#[doc = "S8USBPD C-connector Trim Register4. Production trims stored in flash"]
pub mod s8usbpd_trim_4;
#[doc = "S8USBPD_TRIM_5 register accessor: an alias for `Reg<S8USBPD_TRIM_5_SPEC>`"]
pub type S8USBPD_TRIM_5 = crate::Reg<s8usbpd_trim_5::S8USBPD_TRIM_5_SPEC>;
#[doc = "S8USBPD C-connector Trim Register5. Production trims stored in flash"]
pub mod s8usbpd_trim_5;
#[doc = "S8USBPD_TRIM_6 register accessor: an alias for `Reg<S8USBPD_TRIM_6_SPEC>`"]
pub type S8USBPD_TRIM_6 = crate::Reg<s8usbpd_trim_6::S8USBPD_TRIM_6_SPEC>;
#[doc = "S8USBPD C-connector Trim Register6. Production trims stored in flash"]
pub mod s8usbpd_trim_6;
#[doc = "S8USBPD_TRIM_7 register accessor: an alias for `Reg<S8USBPD_TRIM_7_SPEC>`"]
pub type S8USBPD_TRIM_7 = crate::Reg<s8usbpd_trim_7::S8USBPD_TRIM_7_SPEC>;
#[doc = "S8USBPD C-connector Trim Register7. Production trims stored in flash"]
pub mod s8usbpd_trim_7;
