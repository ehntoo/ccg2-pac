#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - "]
    pub cmd_resp_ctrl: crate::Reg<cmd_resp_ctrl::CMD_RESP_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub cmd_resp_status: crate::Reg<cmd_resp_status::CMD_RESP_STATUS_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - "]
    pub spi_ctrl: crate::Reg<spi_ctrl::SPI_CTRL_SPEC>,
    #[doc = "0x24 - "]
    pub spi_status: crate::Reg<spi_status::SPI_STATUS_SPEC>,
    _reserved6: [u8; 0x18],
    #[doc = "0x40 - "]
    pub uart_ctrl: crate::Reg<uart_ctrl::UART_CTRL_SPEC>,
    #[doc = "0x44 - "]
    pub uart_tx_ctrl: crate::Reg<uart_tx_ctrl::UART_TX_CTRL_SPEC>,
    #[doc = "0x48 - "]
    pub uart_rx_ctrl: crate::Reg<uart_rx_ctrl::UART_RX_CTRL_SPEC>,
    #[doc = "0x4c - "]
    pub uart_rx_status: crate::Reg<uart_rx_status::UART_RX_STATUS_SPEC>,
    #[doc = "0x50 - "]
    pub uart_flow_ctrl: crate::Reg<uart_flow_ctrl::UART_FLOW_CTRL_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x60 - "]
    pub i2c_ctrl: crate::Reg<i2c_ctrl::I2C_CTRL_SPEC>,
    #[doc = "0x64 - "]
    pub i2c_status: crate::Reg<i2c_status::I2C_STATUS_SPEC>,
    #[doc = "0x68 - "]
    pub i2c_m_cmd: crate::Reg<i2c_m_cmd::I2C_M_CMD_SPEC>,
    #[doc = "0x6c - "]
    pub i2c_s_cmd: crate::Reg<i2c_s_cmd::I2C_S_CMD_SPEC>,
    #[doc = "0x70 - "]
    pub i2c_cfg: crate::Reg<i2c_cfg::I2C_CFG_SPEC>,
    _reserved16: [u8; 0x018c],
    #[doc = "0x200 - "]
    pub tx_ctrl: crate::Reg<tx_ctrl::TX_CTRL_SPEC>,
    #[doc = "0x204 - "]
    pub tx_fifo_ctrl: crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>,
    #[doc = "0x208 - "]
    pub tx_fifo_status: crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>,
    _reserved19: [u8; 0x34],
    #[doc = "0x240 - "]
    pub tx_fifo_wr: crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>,
    _reserved20: [u8; 0xbc],
    #[doc = "0x300 - "]
    pub rx_ctrl: crate::Reg<rx_ctrl::RX_CTRL_SPEC>,
    #[doc = "0x304 - "]
    pub rx_fifo_ctrl: crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>,
    #[doc = "0x308 - "]
    pub rx_fifo_status: crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x310 - "]
    pub rx_match: crate::Reg<rx_match::RX_MATCH_SPEC>,
    _reserved24: [u8; 0x2c],
    #[doc = "0x340 - "]
    pub rx_fifo_rd: crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>,
    #[doc = "0x344 - "]
    pub rx_fifo_rd_silent: crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>,
    _reserved26: [u8; 0xb8],
    #[doc = "0x400 - "]
    pub ez_data0: crate::Reg<ez_data0::EZ_DATA0_SPEC>,
    #[doc = "0x404 - "]
    pub ez_data1: crate::Reg<ez_data1::EZ_DATA1_SPEC>,
    #[doc = "0x408 - "]
    pub ez_data2: crate::Reg<ez_data2::EZ_DATA2_SPEC>,
    #[doc = "0x40c - "]
    pub ez_data3: crate::Reg<ez_data3::EZ_DATA3_SPEC>,
    #[doc = "0x410 - "]
    pub ez_data4: crate::Reg<ez_data4::EZ_DATA4_SPEC>,
    #[doc = "0x414 - "]
    pub ez_data5: crate::Reg<ez_data5::EZ_DATA5_SPEC>,
    #[doc = "0x418 - "]
    pub ez_data6: crate::Reg<ez_data6::EZ_DATA6_SPEC>,
    #[doc = "0x41c - "]
    pub ez_data7: crate::Reg<ez_data7::EZ_DATA7_SPEC>,
    #[doc = "0x420 - "]
    pub ez_data8: crate::Reg<ez_data8::EZ_DATA8_SPEC>,
    #[doc = "0x424 - "]
    pub ez_data9: crate::Reg<ez_data9::EZ_DATA9_SPEC>,
    #[doc = "0x428 - "]
    pub ez_data10: crate::Reg<ez_data10::EZ_DATA10_SPEC>,
    #[doc = "0x42c - "]
    pub ez_data11: crate::Reg<ez_data11::EZ_DATA11_SPEC>,
    #[doc = "0x430 - "]
    pub ez_data12: crate::Reg<ez_data12::EZ_DATA12_SPEC>,
    #[doc = "0x434 - "]
    pub ez_data13: crate::Reg<ez_data13::EZ_DATA13_SPEC>,
    #[doc = "0x438 - "]
    pub ez_data14: crate::Reg<ez_data14::EZ_DATA14_SPEC>,
    #[doc = "0x43c - "]
    pub ez_data15: crate::Reg<ez_data15::EZ_DATA15_SPEC>,
    #[doc = "0x440 - "]
    pub ez_data16: crate::Reg<ez_data16::EZ_DATA16_SPEC>,
    #[doc = "0x444 - "]
    pub ez_data17: crate::Reg<ez_data17::EZ_DATA17_SPEC>,
    #[doc = "0x448 - "]
    pub ez_data18: crate::Reg<ez_data18::EZ_DATA18_SPEC>,
    #[doc = "0x44c - "]
    pub ez_data19: crate::Reg<ez_data19::EZ_DATA19_SPEC>,
    #[doc = "0x450 - "]
    pub ez_data20: crate::Reg<ez_data20::EZ_DATA20_SPEC>,
    #[doc = "0x454 - "]
    pub ez_data21: crate::Reg<ez_data21::EZ_DATA21_SPEC>,
    #[doc = "0x458 - "]
    pub ez_data22: crate::Reg<ez_data22::EZ_DATA22_SPEC>,
    #[doc = "0x45c - "]
    pub ez_data23: crate::Reg<ez_data23::EZ_DATA23_SPEC>,
    #[doc = "0x460 - "]
    pub ez_data24: crate::Reg<ez_data24::EZ_DATA24_SPEC>,
    #[doc = "0x464 - "]
    pub ez_data25: crate::Reg<ez_data25::EZ_DATA25_SPEC>,
    #[doc = "0x468 - "]
    pub ez_data26: crate::Reg<ez_data26::EZ_DATA26_SPEC>,
    #[doc = "0x46c - "]
    pub ez_data27: crate::Reg<ez_data27::EZ_DATA27_SPEC>,
    #[doc = "0x470 - "]
    pub ez_data28: crate::Reg<ez_data28::EZ_DATA28_SPEC>,
    #[doc = "0x474 - "]
    pub ez_data29: crate::Reg<ez_data29::EZ_DATA29_SPEC>,
    #[doc = "0x478 - "]
    pub ez_data30: crate::Reg<ez_data30::EZ_DATA30_SPEC>,
    #[doc = "0x47c - "]
    pub ez_data31: crate::Reg<ez_data31::EZ_DATA31_SPEC>,
    _reserved58: [u8; 0x0980],
    #[doc = "0xe00 - "]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved59: [u8; 0x7c],
    #[doc = "0xe80 - "]
    pub intr_i2c_ec: crate::Reg<intr_i2c_ec::INTR_I2C_EC_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0xe88 - "]
    pub intr_i2c_ec_mask: crate::Reg<intr_i2c_ec_mask::INTR_I2C_EC_MASK_SPEC>,
    #[doc = "0xe8c - "]
    pub intr_i2c_ec_masked: crate::Reg<intr_i2c_ec_masked::INTR_I2C_EC_MASKED_SPEC>,
    _reserved62: [u8; 0x30],
    #[doc = "0xec0 - "]
    pub intr_spi_ec: crate::Reg<intr_spi_ec::INTR_SPI_EC_SPEC>,
    _reserved63: [u8; 0x04],
    #[doc = "0xec8 - "]
    pub intr_spi_ec_mask: crate::Reg<intr_spi_ec_mask::INTR_SPI_EC_MASK_SPEC>,
    #[doc = "0xecc - "]
    pub intr_spi_ec_masked: crate::Reg<intr_spi_ec_masked::INTR_SPI_EC_MASKED_SPEC>,
    _reserved65: [u8; 0x30],
    #[doc = "0xf00 - "]
    pub intr_m: crate::Reg<intr_m::INTR_M_SPEC>,
    #[doc = "0xf04 - "]
    pub intr_m_set: crate::Reg<intr_m_set::INTR_M_SET_SPEC>,
    #[doc = "0xf08 - "]
    pub intr_m_mask: crate::Reg<intr_m_mask::INTR_M_MASK_SPEC>,
    #[doc = "0xf0c - "]
    pub intr_m_masked: crate::Reg<intr_m_masked::INTR_M_MASKED_SPEC>,
    _reserved69: [u8; 0x30],
    #[doc = "0xf40 - "]
    pub intr_s: crate::Reg<intr_s::INTR_S_SPEC>,
    #[doc = "0xf44 - "]
    pub intr_s_set: crate::Reg<intr_s_set::INTR_S_SET_SPEC>,
    #[doc = "0xf48 - "]
    pub intr_s_mask: crate::Reg<intr_s_mask::INTR_S_MASK_SPEC>,
    #[doc = "0xf4c - "]
    pub intr_s_masked: crate::Reg<intr_s_masked::INTR_S_MASKED_SPEC>,
    _reserved73: [u8; 0x30],
    #[doc = "0xf80 - "]
    pub intr_tx: crate::Reg<intr_tx::INTR_TX_SPEC>,
    #[doc = "0xf84 - "]
    pub intr_tx_set: crate::Reg<intr_tx_set::INTR_TX_SET_SPEC>,
    #[doc = "0xf88 - "]
    pub intr_tx_mask: crate::Reg<intr_tx_mask::INTR_TX_MASK_SPEC>,
    #[doc = "0xf8c - "]
    pub intr_tx_masked: crate::Reg<intr_tx_masked::INTR_TX_MASKED_SPEC>,
    _reserved77: [u8; 0x30],
    #[doc = "0xfc0 - "]
    pub intr_rx: crate::Reg<intr_rx::INTR_RX_SPEC>,
    #[doc = "0xfc4 - "]
    pub intr_rx_set: crate::Reg<intr_rx_set::INTR_RX_SET_SPEC>,
    #[doc = "0xfc8 - "]
    pub intr_rx_mask: crate::Reg<intr_rx_mask::INTR_RX_MASK_SPEC>,
    #[doc = "0xfcc - "]
    pub intr_rx_masked: crate::Reg<intr_rx_masked::INTR_RX_MASKED_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "CMD_RESP_CTRL register accessor: an alias for `Reg<CMD_RESP_CTRL_SPEC>`"]
pub type CMD_RESP_CTRL = crate::Reg<cmd_resp_ctrl::CMD_RESP_CTRL_SPEC>;
#[doc = ""]
pub mod cmd_resp_ctrl;
#[doc = "CMD_RESP_STATUS register accessor: an alias for `Reg<CMD_RESP_STATUS_SPEC>`"]
pub type CMD_RESP_STATUS = crate::Reg<cmd_resp_status::CMD_RESP_STATUS_SPEC>;
#[doc = ""]
pub mod cmd_resp_status;
#[doc = "SPI_CTRL register accessor: an alias for `Reg<SPI_CTRL_SPEC>`"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = ""]
pub mod spi_ctrl;
#[doc = "SPI_STATUS register accessor: an alias for `Reg<SPI_STATUS_SPEC>`"]
pub type SPI_STATUS = crate::Reg<spi_status::SPI_STATUS_SPEC>;
#[doc = ""]
pub mod spi_status;
#[doc = "UART_CTRL register accessor: an alias for `Reg<UART_CTRL_SPEC>`"]
pub type UART_CTRL = crate::Reg<uart_ctrl::UART_CTRL_SPEC>;
#[doc = ""]
pub mod uart_ctrl;
#[doc = "UART_TX_CTRL register accessor: an alias for `Reg<UART_TX_CTRL_SPEC>`"]
pub type UART_TX_CTRL = crate::Reg<uart_tx_ctrl::UART_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uart_tx_ctrl;
#[doc = "UART_RX_CTRL register accessor: an alias for `Reg<UART_RX_CTRL_SPEC>`"]
pub type UART_RX_CTRL = crate::Reg<uart_rx_ctrl::UART_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uart_rx_ctrl;
#[doc = "UART_RX_STATUS register accessor: an alias for `Reg<UART_RX_STATUS_SPEC>`"]
pub type UART_RX_STATUS = crate::Reg<uart_rx_status::UART_RX_STATUS_SPEC>;
#[doc = ""]
pub mod uart_rx_status;
#[doc = "UART_FLOW_CTRL register accessor: an alias for `Reg<UART_FLOW_CTRL_SPEC>`"]
pub type UART_FLOW_CTRL = crate::Reg<uart_flow_ctrl::UART_FLOW_CTRL_SPEC>;
#[doc = ""]
pub mod uart_flow_ctrl;
#[doc = "I2C_CTRL register accessor: an alias for `Reg<I2C_CTRL_SPEC>`"]
pub type I2C_CTRL = crate::Reg<i2c_ctrl::I2C_CTRL_SPEC>;
#[doc = ""]
pub mod i2c_ctrl;
#[doc = "I2C_STATUS register accessor: an alias for `Reg<I2C_STATUS_SPEC>`"]
pub type I2C_STATUS = crate::Reg<i2c_status::I2C_STATUS_SPEC>;
#[doc = ""]
pub mod i2c_status;
#[doc = "I2C_M_CMD register accessor: an alias for `Reg<I2C_M_CMD_SPEC>`"]
pub type I2C_M_CMD = crate::Reg<i2c_m_cmd::I2C_M_CMD_SPEC>;
#[doc = ""]
pub mod i2c_m_cmd;
#[doc = "I2C_S_CMD register accessor: an alias for `Reg<I2C_S_CMD_SPEC>`"]
pub type I2C_S_CMD = crate::Reg<i2c_s_cmd::I2C_S_CMD_SPEC>;
#[doc = ""]
pub mod i2c_s_cmd;
#[doc = "I2C_CFG register accessor: an alias for `Reg<I2C_CFG_SPEC>`"]
pub type I2C_CFG = crate::Reg<i2c_cfg::I2C_CFG_SPEC>;
#[doc = ""]
pub mod i2c_cfg;
#[doc = "TX_CTRL register accessor: an alias for `Reg<TX_CTRL_SPEC>`"]
pub type TX_CTRL = crate::Reg<tx_ctrl::TX_CTRL_SPEC>;
#[doc = ""]
pub mod tx_ctrl;
#[doc = "TX_FIFO_CTRL register accessor: an alias for `Reg<TX_FIFO_CTRL_SPEC>`"]
pub type TX_FIFO_CTRL = crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>;
#[doc = ""]
pub mod tx_fifo_ctrl;
#[doc = "TX_FIFO_STATUS register accessor: an alias for `Reg<TX_FIFO_STATUS_SPEC>`"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = ""]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR register accessor: an alias for `Reg<TX_FIFO_WR_SPEC>`"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = ""]
pub mod tx_fifo_wr;
#[doc = "RX_CTRL register accessor: an alias for `Reg<RX_CTRL_SPEC>`"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = ""]
pub mod rx_ctrl;
#[doc = "RX_FIFO_CTRL register accessor: an alias for `Reg<RX_FIFO_CTRL_SPEC>`"]
pub type RX_FIFO_CTRL = crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>;
#[doc = ""]
pub mod rx_fifo_ctrl;
#[doc = "RX_FIFO_STATUS register accessor: an alias for `Reg<RX_FIFO_STATUS_SPEC>`"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = ""]
pub mod rx_fifo_status;
#[doc = "RX_MATCH register accessor: an alias for `Reg<RX_MATCH_SPEC>`"]
pub type RX_MATCH = crate::Reg<rx_match::RX_MATCH_SPEC>;
#[doc = ""]
pub mod rx_match;
#[doc = "RX_FIFO_RD register accessor: an alias for `Reg<RX_FIFO_RD_SPEC>`"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = ""]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT register accessor: an alias for `Reg<RX_FIFO_RD_SILENT_SPEC>`"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = ""]
pub mod rx_fifo_rd_silent;
#[doc = "EZ_DATA0 register accessor: an alias for `Reg<EZ_DATA0_SPEC>`"]
pub type EZ_DATA0 = crate::Reg<ez_data0::EZ_DATA0_SPEC>;
#[doc = ""]
pub mod ez_data0;
#[doc = "EZ_DATA1 register accessor: an alias for `Reg<EZ_DATA1_SPEC>`"]
pub type EZ_DATA1 = crate::Reg<ez_data1::EZ_DATA1_SPEC>;
#[doc = ""]
pub mod ez_data1;
#[doc = "EZ_DATA2 register accessor: an alias for `Reg<EZ_DATA2_SPEC>`"]
pub type EZ_DATA2 = crate::Reg<ez_data2::EZ_DATA2_SPEC>;
#[doc = ""]
pub mod ez_data2;
#[doc = "EZ_DATA3 register accessor: an alias for `Reg<EZ_DATA3_SPEC>`"]
pub type EZ_DATA3 = crate::Reg<ez_data3::EZ_DATA3_SPEC>;
#[doc = ""]
pub mod ez_data3;
#[doc = "EZ_DATA4 register accessor: an alias for `Reg<EZ_DATA4_SPEC>`"]
pub type EZ_DATA4 = crate::Reg<ez_data4::EZ_DATA4_SPEC>;
#[doc = ""]
pub mod ez_data4;
#[doc = "EZ_DATA5 register accessor: an alias for `Reg<EZ_DATA5_SPEC>`"]
pub type EZ_DATA5 = crate::Reg<ez_data5::EZ_DATA5_SPEC>;
#[doc = ""]
pub mod ez_data5;
#[doc = "EZ_DATA6 register accessor: an alias for `Reg<EZ_DATA6_SPEC>`"]
pub type EZ_DATA6 = crate::Reg<ez_data6::EZ_DATA6_SPEC>;
#[doc = ""]
pub mod ez_data6;
#[doc = "EZ_DATA7 register accessor: an alias for `Reg<EZ_DATA7_SPEC>`"]
pub type EZ_DATA7 = crate::Reg<ez_data7::EZ_DATA7_SPEC>;
#[doc = ""]
pub mod ez_data7;
#[doc = "EZ_DATA8 register accessor: an alias for `Reg<EZ_DATA8_SPEC>`"]
pub type EZ_DATA8 = crate::Reg<ez_data8::EZ_DATA8_SPEC>;
#[doc = ""]
pub mod ez_data8;
#[doc = "EZ_DATA9 register accessor: an alias for `Reg<EZ_DATA9_SPEC>`"]
pub type EZ_DATA9 = crate::Reg<ez_data9::EZ_DATA9_SPEC>;
#[doc = ""]
pub mod ez_data9;
#[doc = "EZ_DATA10 register accessor: an alias for `Reg<EZ_DATA10_SPEC>`"]
pub type EZ_DATA10 = crate::Reg<ez_data10::EZ_DATA10_SPEC>;
#[doc = ""]
pub mod ez_data10;
#[doc = "EZ_DATA11 register accessor: an alias for `Reg<EZ_DATA11_SPEC>`"]
pub type EZ_DATA11 = crate::Reg<ez_data11::EZ_DATA11_SPEC>;
#[doc = ""]
pub mod ez_data11;
#[doc = "EZ_DATA12 register accessor: an alias for `Reg<EZ_DATA12_SPEC>`"]
pub type EZ_DATA12 = crate::Reg<ez_data12::EZ_DATA12_SPEC>;
#[doc = ""]
pub mod ez_data12;
#[doc = "EZ_DATA13 register accessor: an alias for `Reg<EZ_DATA13_SPEC>`"]
pub type EZ_DATA13 = crate::Reg<ez_data13::EZ_DATA13_SPEC>;
#[doc = ""]
pub mod ez_data13;
#[doc = "EZ_DATA14 register accessor: an alias for `Reg<EZ_DATA14_SPEC>`"]
pub type EZ_DATA14 = crate::Reg<ez_data14::EZ_DATA14_SPEC>;
#[doc = ""]
pub mod ez_data14;
#[doc = "EZ_DATA15 register accessor: an alias for `Reg<EZ_DATA15_SPEC>`"]
pub type EZ_DATA15 = crate::Reg<ez_data15::EZ_DATA15_SPEC>;
#[doc = ""]
pub mod ez_data15;
#[doc = "EZ_DATA16 register accessor: an alias for `Reg<EZ_DATA16_SPEC>`"]
pub type EZ_DATA16 = crate::Reg<ez_data16::EZ_DATA16_SPEC>;
#[doc = ""]
pub mod ez_data16;
#[doc = "EZ_DATA17 register accessor: an alias for `Reg<EZ_DATA17_SPEC>`"]
pub type EZ_DATA17 = crate::Reg<ez_data17::EZ_DATA17_SPEC>;
#[doc = ""]
pub mod ez_data17;
#[doc = "EZ_DATA18 register accessor: an alias for `Reg<EZ_DATA18_SPEC>`"]
pub type EZ_DATA18 = crate::Reg<ez_data18::EZ_DATA18_SPEC>;
#[doc = ""]
pub mod ez_data18;
#[doc = "EZ_DATA19 register accessor: an alias for `Reg<EZ_DATA19_SPEC>`"]
pub type EZ_DATA19 = crate::Reg<ez_data19::EZ_DATA19_SPEC>;
#[doc = ""]
pub mod ez_data19;
#[doc = "EZ_DATA20 register accessor: an alias for `Reg<EZ_DATA20_SPEC>`"]
pub type EZ_DATA20 = crate::Reg<ez_data20::EZ_DATA20_SPEC>;
#[doc = ""]
pub mod ez_data20;
#[doc = "EZ_DATA21 register accessor: an alias for `Reg<EZ_DATA21_SPEC>`"]
pub type EZ_DATA21 = crate::Reg<ez_data21::EZ_DATA21_SPEC>;
#[doc = ""]
pub mod ez_data21;
#[doc = "EZ_DATA22 register accessor: an alias for `Reg<EZ_DATA22_SPEC>`"]
pub type EZ_DATA22 = crate::Reg<ez_data22::EZ_DATA22_SPEC>;
#[doc = ""]
pub mod ez_data22;
#[doc = "EZ_DATA23 register accessor: an alias for `Reg<EZ_DATA23_SPEC>`"]
pub type EZ_DATA23 = crate::Reg<ez_data23::EZ_DATA23_SPEC>;
#[doc = ""]
pub mod ez_data23;
#[doc = "EZ_DATA24 register accessor: an alias for `Reg<EZ_DATA24_SPEC>`"]
pub type EZ_DATA24 = crate::Reg<ez_data24::EZ_DATA24_SPEC>;
#[doc = ""]
pub mod ez_data24;
#[doc = "EZ_DATA25 register accessor: an alias for `Reg<EZ_DATA25_SPEC>`"]
pub type EZ_DATA25 = crate::Reg<ez_data25::EZ_DATA25_SPEC>;
#[doc = ""]
pub mod ez_data25;
#[doc = "EZ_DATA26 register accessor: an alias for `Reg<EZ_DATA26_SPEC>`"]
pub type EZ_DATA26 = crate::Reg<ez_data26::EZ_DATA26_SPEC>;
#[doc = ""]
pub mod ez_data26;
#[doc = "EZ_DATA27 register accessor: an alias for `Reg<EZ_DATA27_SPEC>`"]
pub type EZ_DATA27 = crate::Reg<ez_data27::EZ_DATA27_SPEC>;
#[doc = ""]
pub mod ez_data27;
#[doc = "EZ_DATA28 register accessor: an alias for `Reg<EZ_DATA28_SPEC>`"]
pub type EZ_DATA28 = crate::Reg<ez_data28::EZ_DATA28_SPEC>;
#[doc = ""]
pub mod ez_data28;
#[doc = "EZ_DATA29 register accessor: an alias for `Reg<EZ_DATA29_SPEC>`"]
pub type EZ_DATA29 = crate::Reg<ez_data29::EZ_DATA29_SPEC>;
#[doc = ""]
pub mod ez_data29;
#[doc = "EZ_DATA30 register accessor: an alias for `Reg<EZ_DATA30_SPEC>`"]
pub type EZ_DATA30 = crate::Reg<ez_data30::EZ_DATA30_SPEC>;
#[doc = ""]
pub mod ez_data30;
#[doc = "EZ_DATA31 register accessor: an alias for `Reg<EZ_DATA31_SPEC>`"]
pub type EZ_DATA31 = crate::Reg<ez_data31::EZ_DATA31_SPEC>;
#[doc = ""]
pub mod ez_data31;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = ""]
pub mod intr_cause;
#[doc = "INTR_I2C_EC register accessor: an alias for `Reg<INTR_I2C_EC_SPEC>`"]
pub type INTR_I2C_EC = crate::Reg<intr_i2c_ec::INTR_I2C_EC_SPEC>;
#[doc = ""]
pub mod intr_i2c_ec;
#[doc = "INTR_I2C_EC_MASK register accessor: an alias for `Reg<INTR_I2C_EC_MASK_SPEC>`"]
pub type INTR_I2C_EC_MASK = crate::Reg<intr_i2c_ec_mask::INTR_I2C_EC_MASK_SPEC>;
#[doc = ""]
pub mod intr_i2c_ec_mask;
#[doc = "INTR_I2C_EC_MASKED register accessor: an alias for `Reg<INTR_I2C_EC_MASKED_SPEC>`"]
pub type INTR_I2C_EC_MASKED = crate::Reg<intr_i2c_ec_masked::INTR_I2C_EC_MASKED_SPEC>;
#[doc = ""]
pub mod intr_i2c_ec_masked;
#[doc = "INTR_SPI_EC register accessor: an alias for `Reg<INTR_SPI_EC_SPEC>`"]
pub type INTR_SPI_EC = crate::Reg<intr_spi_ec::INTR_SPI_EC_SPEC>;
#[doc = ""]
pub mod intr_spi_ec;
#[doc = "INTR_SPI_EC_MASK register accessor: an alias for `Reg<INTR_SPI_EC_MASK_SPEC>`"]
pub type INTR_SPI_EC_MASK = crate::Reg<intr_spi_ec_mask::INTR_SPI_EC_MASK_SPEC>;
#[doc = ""]
pub mod intr_spi_ec_mask;
#[doc = "INTR_SPI_EC_MASKED register accessor: an alias for `Reg<INTR_SPI_EC_MASKED_SPEC>`"]
pub type INTR_SPI_EC_MASKED = crate::Reg<intr_spi_ec_masked::INTR_SPI_EC_MASKED_SPEC>;
#[doc = ""]
pub mod intr_spi_ec_masked;
#[doc = "INTR_M register accessor: an alias for `Reg<INTR_M_SPEC>`"]
pub type INTR_M = crate::Reg<intr_m::INTR_M_SPEC>;
#[doc = ""]
pub mod intr_m;
#[doc = "INTR_M_SET register accessor: an alias for `Reg<INTR_M_SET_SPEC>`"]
pub type INTR_M_SET = crate::Reg<intr_m_set::INTR_M_SET_SPEC>;
#[doc = ""]
pub mod intr_m_set;
#[doc = "INTR_M_MASK register accessor: an alias for `Reg<INTR_M_MASK_SPEC>`"]
pub type INTR_M_MASK = crate::Reg<intr_m_mask::INTR_M_MASK_SPEC>;
#[doc = ""]
pub mod intr_m_mask;
#[doc = "INTR_M_MASKED register accessor: an alias for `Reg<INTR_M_MASKED_SPEC>`"]
pub type INTR_M_MASKED = crate::Reg<intr_m_masked::INTR_M_MASKED_SPEC>;
#[doc = ""]
pub mod intr_m_masked;
#[doc = "INTR_S register accessor: an alias for `Reg<INTR_S_SPEC>`"]
pub type INTR_S = crate::Reg<intr_s::INTR_S_SPEC>;
#[doc = ""]
pub mod intr_s;
#[doc = "INTR_S_SET register accessor: an alias for `Reg<INTR_S_SET_SPEC>`"]
pub type INTR_S_SET = crate::Reg<intr_s_set::INTR_S_SET_SPEC>;
#[doc = ""]
pub mod intr_s_set;
#[doc = "INTR_S_MASK register accessor: an alias for `Reg<INTR_S_MASK_SPEC>`"]
pub type INTR_S_MASK = crate::Reg<intr_s_mask::INTR_S_MASK_SPEC>;
#[doc = ""]
pub mod intr_s_mask;
#[doc = "INTR_S_MASKED register accessor: an alias for `Reg<INTR_S_MASKED_SPEC>`"]
pub type INTR_S_MASKED = crate::Reg<intr_s_masked::INTR_S_MASKED_SPEC>;
#[doc = ""]
pub mod intr_s_masked;
#[doc = "INTR_TX register accessor: an alias for `Reg<INTR_TX_SPEC>`"]
pub type INTR_TX = crate::Reg<intr_tx::INTR_TX_SPEC>;
#[doc = ""]
pub mod intr_tx;
#[doc = "INTR_TX_SET register accessor: an alias for `Reg<INTR_TX_SET_SPEC>`"]
pub type INTR_TX_SET = crate::Reg<intr_tx_set::INTR_TX_SET_SPEC>;
#[doc = ""]
pub mod intr_tx_set;
#[doc = "INTR_TX_MASK register accessor: an alias for `Reg<INTR_TX_MASK_SPEC>`"]
pub type INTR_TX_MASK = crate::Reg<intr_tx_mask::INTR_TX_MASK_SPEC>;
#[doc = ""]
pub mod intr_tx_mask;
#[doc = "INTR_TX_MASKED register accessor: an alias for `Reg<INTR_TX_MASKED_SPEC>`"]
pub type INTR_TX_MASKED = crate::Reg<intr_tx_masked::INTR_TX_MASKED_SPEC>;
#[doc = ""]
pub mod intr_tx_masked;
#[doc = "INTR_RX register accessor: an alias for `Reg<INTR_RX_SPEC>`"]
pub type INTR_RX = crate::Reg<intr_rx::INTR_RX_SPEC>;
#[doc = ""]
pub mod intr_rx;
#[doc = "INTR_RX_SET register accessor: an alias for `Reg<INTR_RX_SET_SPEC>`"]
pub type INTR_RX_SET = crate::Reg<intr_rx_set::INTR_RX_SET_SPEC>;
#[doc = ""]
pub mod intr_rx_set;
#[doc = "INTR_RX_MASK register accessor: an alias for `Reg<INTR_RX_MASK_SPEC>`"]
pub type INTR_RX_MASK = crate::Reg<intr_rx_mask::INTR_RX_MASK_SPEC>;
#[doc = ""]
pub mod intr_rx_mask;
#[doc = "INTR_RX_MASKED register accessor: an alias for `Reg<INTR_RX_MASKED_SPEC>`"]
pub type INTR_RX_MASKED = crate::Reg<intr_rx_masked::INTR_RX_MASKED_SPEC>;
#[doc = ""]
pub mod intr_rx_masked;
