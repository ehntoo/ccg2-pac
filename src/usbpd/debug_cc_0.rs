#[doc = "Register `DEBUG_CC_0` reader"]
pub struct R(crate::R<DEBUG_CC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CC_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_CC_0` writer"]
pub struct W(crate::W<DEBUG_CC_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DEBUG_CC_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CC_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_FIRST_BIT_LEVEL` reader - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
pub struct TX_FIRST_BIT_LEVEL_R(crate::FieldReader<bool, bool>);
impl TX_FIRST_BIT_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIRST_BIT_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIRST_BIT_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIRST_BIT_LEVEL` writer - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
pub struct TX_FIRST_BIT_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIRST_BIT_LEVEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `LOOP_BACK_NO_BMC` reader - When set enables TX to RX loopback before CC encoding/Decoding data."]
pub struct LOOP_BACK_NO_BMC_R(crate::FieldReader<bool, bool>);
impl LOOP_BACK_NO_BMC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOP_BACK_NO_BMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOP_BACK_NO_BMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOP_BACK_NO_BMC` writer - When set enables TX to RX loopback before CC encoding/Decoding data."]
pub struct LOOP_BACK_NO_BMC_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_BACK_NO_BMC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LOOP_BACK_WITH_BMC` reader - Loobback after data encdoing. When set, the BMC encoded tx output will loop back into cc_rx module."]
pub struct LOOP_BACK_WITH_BMC_R(crate::FieldReader<bool, bool>);
impl LOOP_BACK_WITH_BMC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOP_BACK_WITH_BMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOP_BACK_WITH_BMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOP_BACK_WITH_BMC` writer - Loobback after data encdoing. When set, the BMC encoded tx output will loop back into cc_rx module."]
pub struct LOOP_BACK_WITH_BMC_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_BACK_WITH_BMC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `EXT_LOOP_BACK` reader - When set, enables rx module to decode cc_rx line all the time. (Including during transmission)."]
pub struct EXT_LOOP_BACK_R(crate::FieldReader<bool, bool>);
impl EXT_LOOP_BACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_LOOP_BACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_LOOP_BACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_LOOP_BACK` writer - When set, enables rx module to decode cc_rx line all the time. (Including during transmission)."]
pub struct EXT_LOOP_BACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_LOOP_BACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RX_CLEAR` reader - When set to one, clears the BMC decoder RX state machines and counters. It has to be set back to zero for normal operations. RX_RESET is not required to be set"]
pub struct RX_CLEAR_R(crate::FieldReader<bool, bool>);
impl RX_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLEAR` writer - When set to one, clears the BMC decoder RX state machines and counters. It has to be set back to zero for normal operations. RX_RESET is not required to be set"]
pub struct RX_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLEAR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TX_CLEAR` reader - When set to one, clears the TX state machines and counters. It has to be set back to zero for normal operations."]
pub struct TX_CLEAR_R(crate::FieldReader<bool, bool>);
impl TX_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLEAR` writer - When set to one, clears the TX state machines and counters. It has to be set back to zero for normal operations."]
pub struct TX_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLEAR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TX_CC_DRIVE_SRC` reader - This will selects either the m0s8usbpd_cc_tx or FW to control the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 0: Hardware (m0s8usbpd_cc_tx) controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 1: This option is for Testing/Char. FW controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP."]
pub struct TX_CC_DRIVE_SRC_R(crate::FieldReader<bool, bool>);
impl TX_CC_DRIVE_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CC_DRIVE_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CC_DRIVE_SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CC_DRIVE_SRC` writer - This will selects either the m0s8usbpd_cc_tx or FW to control the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 0: Hardware (m0s8usbpd_cc_tx) controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 1: This option is for Testing/Char. FW controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP."]
pub struct TX_CC_DRIVE_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CC_DRIVE_SRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TX_CC_DATA` reader - FW can use this bit to dirve the CC data line when the TX_CC_DRIVE_SRC=1 and CC_DPSLP_REF_CTRL.TX_EN=1 When TX_CC_DRIVE_SRC is set to one: - TX_EN port of the s8usbpd_cc_top is controlled by CC_DPSLP_REF_CTRL.TX_EN - TX_DATA port of s8usbpd_cc_top Hard IP is controlled by TX_CC_DATA"]
pub struct TX_CC_DATA_R(crate::FieldReader<bool, bool>);
impl TX_CC_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CC_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CC_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CC_DATA` writer - FW can use this bit to dirve the CC data line when the TX_CC_DRIVE_SRC=1 and CC_DPSLP_REF_CTRL.TX_EN=1 When TX_CC_DRIVE_SRC is set to one: - TX_EN port of the s8usbpd_cc_top is controlled by CC_DPSLP_REF_CTRL.TX_EN - TX_DATA port of s8usbpd_cc_top Hard IP is controlled by TX_CC_DATA"]
pub struct TX_CC_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CC_DATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DEBUG_SEL` reader - Selects the inputs to CC_DEBUG_OUT. Used for debug. 0. RX clk_cnt_q 1. RX cnt_max_q 2. RX cnt_min_q 3. Not defined yet. 4. RX {rx_state_q, cc_rx_data_del_q, one_detect_q} 5. RX {4'h0, cq_2, cq_3, cq_4, cq_5} 6. RX {2'h0, cc_rx_data, cc_rx_bit, cc_rx_valid, cc_rx_data_pin, cc_tx_data_pin, cc_data_pin_oe} 7. Not defined yet. 8. Not defined yet. 9. TX {2'h0, tx_state_q, new_data_q} 10. TX {2'h0, cq_0, cq_3, cc_tx_data_lat_q, cc_tx_eof_lat_q, cc_tx_data_valid_lat_q, cc_new_data_lat_q} 11. TX {3'h0, cc_tx_data_pin, cc_data_pin_oe, cc_tx_data, cc_tx_eof, cc_tx_data_valid} 12-15 not defined yet."]
pub struct DEBUG_SEL_R(crate::FieldReader<u8, u8>);
impl DEBUG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_SEL` writer - Selects the inputs to CC_DEBUG_OUT. Used for debug. 0. RX clk_cnt_q 1. RX cnt_max_q 2. RX cnt_min_q 3. Not defined yet. 4. RX {rx_state_q, cc_rx_data_del_q, one_detect_q} 5. RX {4'h0, cq_2, cq_3, cq_4, cq_5} 6. RX {2'h0, cc_rx_data, cc_rx_bit, cc_rx_valid, cc_rx_data_pin, cc_tx_data_pin, cc_data_pin_oe} 7. Not defined yet. 8. Not defined yet. 9. TX {2'h0, tx_state_q, new_data_q} 10. TX {2'h0, cq_0, cq_3, cc_tx_data_lat_q, cc_tx_eof_lat_q, cc_tx_data_valid_lat_q, cc_new_data_lat_q} 11. TX {3'h0, cc_tx_data_pin, cc_data_pin_oe, cc_tx_data, cc_tx_eof, cc_tx_data_valid} 12-15 not defined yet."]
pub struct DEBUG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RX_CC_DATA_VALID_DIS` reader - 0: RX_CC_DATA_VALID signal is not disabled. 1: RX_CC_DATA_VALID signal is disabled."]
pub struct RX_CC_DATA_VALID_DIS_R(crate::FieldReader<bool, bool>);
impl RX_CC_DATA_VALID_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_CC_DATA_VALID_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CC_DATA_VALID_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CC_DATA_VALID_DIS` writer - 0: RX_CC_DATA_VALID signal is not disabled. 1: RX_CC_DATA_VALID signal is disabled."]
pub struct RX_CC_DATA_VALID_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CC_DATA_VALID_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DEBUG_OUT` reader - Debug output register. Its inputs are selected by CC_DEBUG_SEL"]
pub struct DEBUG_OUT_R(crate::FieldReader<u8, u8>);
impl DEBUG_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREF_SEL` reader - Selection bit for deepsleep vs. active current reference for Rp pull-up termination 0 - Select deepsleep current reference 1 - Select active current reference"]
pub struct IREF_SEL_R(crate::FieldReader<bool, bool>);
impl IREF_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IREF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IREF_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREF_SEL` writer - Selection bit for deepsleep vs. active current reference for Rp pull-up termination 0 - Select deepsleep current reference 1 - Select active current reference"]
pub struct IREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IREF_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `VCONN2_CMP_OUT_SEL` reader - Selection bit for source of wakeup between vconn2_changed and cmp_out. 0: vconn2_changed is the source 1: cmp_out is the source"]
pub struct VCONN2_CMP_OUT_SEL_R(crate::FieldReader<bool, bool>);
impl VCONN2_CMP_OUT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN2_CMP_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN2_CMP_OUT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN2_CMP_OUT_SEL` writer - Selection bit for source of wakeup between vconn2_changed and cmp_out. 0: vconn2_changed is the source 1: cmp_out is the source"]
pub struct VCONN2_CMP_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCONN2_CMP_OUT_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `TX_CNT` reader - Used only for FPGA 0: Disabled Other values: BMC encoder in m0s8usbpd_cc_tx module uses clk_tx and clk_tx_half to transmit data at 300 KHz and clock transitions at 600 KHz. This scheme provides a 50% duty cycle at all time. However receivers are required to tolerate duty cycles of 35% to 65%. To help with debugging of the receiver logic, the duty cycle of transmit data can become variable. This feature is achieved by adding mmio_cc_tx_cnt<7:0> register"]
pub struct TX_CNT_R(crate::FieldReader<u8, u8>);
impl TX_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CNT` writer - Used only for FPGA 0: Disabled Other values: BMC encoder in m0s8usbpd_cc_tx module uses clk_tx and clk_tx_half to transmit data at 300 KHz and clock transitions at 600 KHz. This scheme provides a 50% duty cycle at all time. However receivers are required to tolerate duty cycles of 35% to 65%. To help with debugging of the receiver logic, the duty cycle of transmit data can become variable. This feature is achieved by adding mmio_cc_tx_cnt<7:0> register"]
pub struct TX_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
    #[inline(always)]
    pub fn tx_first_bit_level(&self) -> TX_FIRST_BIT_LEVEL_R {
        TX_FIRST_BIT_LEVEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When set enables TX to RX loopback before CC encoding/Decoding data."]
    #[inline(always)]
    pub fn loop_back_no_bmc(&self) -> LOOP_BACK_NO_BMC_R {
        LOOP_BACK_NO_BMC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loobback after data encdoing. When set, the BMC encoded tx output will loop back into cc_rx module."]
    #[inline(always)]
    pub fn loop_back_with_bmc(&self) -> LOOP_BACK_WITH_BMC_R {
        LOOP_BACK_WITH_BMC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When set, enables rx module to decode cc_rx line all the time. (Including during transmission)."]
    #[inline(always)]
    pub fn ext_loop_back(&self) -> EXT_LOOP_BACK_R {
        EXT_LOOP_BACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When set to one, clears the BMC decoder RX state machines and counters. It has to be set back to zero for normal operations. RX_RESET is not required to be set"]
    #[inline(always)]
    pub fn rx_clear(&self) -> RX_CLEAR_R {
        RX_CLEAR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When set to one, clears the TX state machines and counters. It has to be set back to zero for normal operations."]
    #[inline(always)]
    pub fn tx_clear(&self) -> TX_CLEAR_R {
        TX_CLEAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This will selects either the m0s8usbpd_cc_tx or FW to control the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 0: Hardware (m0s8usbpd_cc_tx) controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 1: This option is for Testing/Char. FW controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP."]
    #[inline(always)]
    pub fn tx_cc_drive_src(&self) -> TX_CC_DRIVE_SRC_R {
        TX_CC_DRIVE_SRC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FW can use this bit to dirve the CC data line when the TX_CC_DRIVE_SRC=1 and CC_DPSLP_REF_CTRL.TX_EN=1 When TX_CC_DRIVE_SRC is set to one: - TX_EN port of the s8usbpd_cc_top is controlled by CC_DPSLP_REF_CTRL.TX_EN - TX_DATA port of s8usbpd_cc_top Hard IP is controlled by TX_CC_DATA"]
    #[inline(always)]
    pub fn tx_cc_data(&self) -> TX_CC_DATA_R {
        TX_CC_DATA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Selects the inputs to CC_DEBUG_OUT. Used for debug. 0. RX clk_cnt_q 1. RX cnt_max_q 2. RX cnt_min_q 3. Not defined yet. 4. RX {rx_state_q, cc_rx_data_del_q, one_detect_q} 5. RX {4'h0, cq_2, cq_3, cq_4, cq_5} 6. RX {2'h0, cc_rx_data, cc_rx_bit, cc_rx_valid, cc_rx_data_pin, cc_tx_data_pin, cc_data_pin_oe} 7. Not defined yet. 8. Not defined yet. 9. TX {2'h0, tx_state_q, new_data_q} 10. TX {2'h0, cq_0, cq_3, cc_tx_data_lat_q, cc_tx_eof_lat_q, cc_tx_data_valid_lat_q, cc_new_data_lat_q} 11. TX {3'h0, cc_tx_data_pin, cc_data_pin_oe, cc_tx_data, cc_tx_eof, cc_tx_data_valid} 12-15 not defined yet."]
    #[inline(always)]
    pub fn debug_sel(&self) -> DEBUG_SEL_R {
        DEBUG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 0: RX_CC_DATA_VALID signal is not disabled. 1: RX_CC_DATA_VALID signal is disabled."]
    #[inline(always)]
    pub fn rx_cc_data_valid_dis(&self) -> RX_CC_DATA_VALID_DIS_R {
        RX_CC_DATA_VALID_DIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Debug output register. Its inputs are selected by CC_DEBUG_SEL"]
    #[inline(always)]
    pub fn debug_out(&self) -> DEBUG_OUT_R {
        DEBUG_OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Selection bit for deepsleep vs. active current reference for Rp pull-up termination 0 - Select deepsleep current reference 1 - Select active current reference"]
    #[inline(always)]
    pub fn iref_sel(&self) -> IREF_SEL_R {
        IREF_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Selection bit for source of wakeup between vconn2_changed and cmp_out. 0: vconn2_changed is the source 1: cmp_out is the source"]
    #[inline(always)]
    pub fn vconn2_cmp_out_sel(&self) -> VCONN2_CMP_OUT_SEL_R {
        VCONN2_CMP_OUT_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Used only for FPGA 0: Disabled Other values: BMC encoder in m0s8usbpd_cc_tx module uses clk_tx and clk_tx_half to transmit data at 300 KHz and clock transitions at 600 KHz. This scheme provides a 50% duty cycle at all time. However receivers are required to tolerate duty cycles of 35% to 65%. To help with debugging of the receiver logic, the duty cycle of transmit data can become variable. This feature is achieved by adding mmio_cc_tx_cnt<7:0> register"]
    #[inline(always)]
    pub fn tx_cnt(&self) -> TX_CNT_R {
        TX_CNT_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
    #[inline(always)]
    pub fn tx_first_bit_level(&mut self) -> TX_FIRST_BIT_LEVEL_W {
        TX_FIRST_BIT_LEVEL_W { w: self }
    }
    #[doc = "Bit 1 - When set enables TX to RX loopback before CC encoding/Decoding data."]
    #[inline(always)]
    pub fn loop_back_no_bmc(&mut self) -> LOOP_BACK_NO_BMC_W {
        LOOP_BACK_NO_BMC_W { w: self }
    }
    #[doc = "Bit 2 - Loobback after data encdoing. When set, the BMC encoded tx output will loop back into cc_rx module."]
    #[inline(always)]
    pub fn loop_back_with_bmc(&mut self) -> LOOP_BACK_WITH_BMC_W {
        LOOP_BACK_WITH_BMC_W { w: self }
    }
    #[doc = "Bit 3 - When set, enables rx module to decode cc_rx line all the time. (Including during transmission)."]
    #[inline(always)]
    pub fn ext_loop_back(&mut self) -> EXT_LOOP_BACK_W {
        EXT_LOOP_BACK_W { w: self }
    }
    #[doc = "Bit 4 - When set to one, clears the BMC decoder RX state machines and counters. It has to be set back to zero for normal operations. RX_RESET is not required to be set"]
    #[inline(always)]
    pub fn rx_clear(&mut self) -> RX_CLEAR_W {
        RX_CLEAR_W { w: self }
    }
    #[doc = "Bit 5 - When set to one, clears the TX state machines and counters. It has to be set back to zero for normal operations."]
    #[inline(always)]
    pub fn tx_clear(&mut self) -> TX_CLEAR_W {
        TX_CLEAR_W { w: self }
    }
    #[doc = "Bit 6 - This will selects either the m0s8usbpd_cc_tx or FW to control the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 0: Hardware (m0s8usbpd_cc_tx) controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP. 1: This option is for Testing/Char. FW controls the TX_EN/TX_DATA ports of the s8usbpd_cc_top Hard IP."]
    #[inline(always)]
    pub fn tx_cc_drive_src(&mut self) -> TX_CC_DRIVE_SRC_W {
        TX_CC_DRIVE_SRC_W { w: self }
    }
    #[doc = "Bit 7 - FW can use this bit to dirve the CC data line when the TX_CC_DRIVE_SRC=1 and CC_DPSLP_REF_CTRL.TX_EN=1 When TX_CC_DRIVE_SRC is set to one: - TX_EN port of the s8usbpd_cc_top is controlled by CC_DPSLP_REF_CTRL.TX_EN - TX_DATA port of s8usbpd_cc_top Hard IP is controlled by TX_CC_DATA"]
    #[inline(always)]
    pub fn tx_cc_data(&mut self) -> TX_CC_DATA_W {
        TX_CC_DATA_W { w: self }
    }
    #[doc = "Bits 8:11 - Selects the inputs to CC_DEBUG_OUT. Used for debug. 0. RX clk_cnt_q 1. RX cnt_max_q 2. RX cnt_min_q 3. Not defined yet. 4. RX {rx_state_q, cc_rx_data_del_q, one_detect_q} 5. RX {4'h0, cq_2, cq_3, cq_4, cq_5} 6. RX {2'h0, cc_rx_data, cc_rx_bit, cc_rx_valid, cc_rx_data_pin, cc_tx_data_pin, cc_data_pin_oe} 7. Not defined yet. 8. Not defined yet. 9. TX {2'h0, tx_state_q, new_data_q} 10. TX {2'h0, cq_0, cq_3, cc_tx_data_lat_q, cc_tx_eof_lat_q, cc_tx_data_valid_lat_q, cc_new_data_lat_q} 11. TX {3'h0, cc_tx_data_pin, cc_data_pin_oe, cc_tx_data, cc_tx_eof, cc_tx_data_valid} 12-15 not defined yet."]
    #[inline(always)]
    pub fn debug_sel(&mut self) -> DEBUG_SEL_W {
        DEBUG_SEL_W { w: self }
    }
    #[doc = "Bit 12 - 0: RX_CC_DATA_VALID signal is not disabled. 1: RX_CC_DATA_VALID signal is disabled."]
    #[inline(always)]
    pub fn rx_cc_data_valid_dis(&mut self) -> RX_CC_DATA_VALID_DIS_W {
        RX_CC_DATA_VALID_DIS_W { w: self }
    }
    #[doc = "Bit 28 - Selection bit for deepsleep vs. active current reference for Rp pull-up termination 0 - Select deepsleep current reference 1 - Select active current reference"]
    #[inline(always)]
    pub fn iref_sel(&mut self) -> IREF_SEL_W {
        IREF_SEL_W { w: self }
    }
    #[doc = "Bit 29 - Selection bit for source of wakeup between vconn2_changed and cmp_out. 0: vconn2_changed is the source 1: cmp_out is the source"]
    #[inline(always)]
    pub fn vconn2_cmp_out_sel(&mut self) -> VCONN2_CMP_OUT_SEL_W {
        VCONN2_CMP_OUT_SEL_W { w: self }
    }
    #[doc = "Bits 30:31 - Used only for FPGA 0: Disabled Other values: BMC encoder in m0s8usbpd_cc_tx module uses clk_tx and clk_tx_half to transmit data at 300 KHz and clock transitions at 600 KHz. This scheme provides a 50% duty cycle at all time. However receivers are required to tolerate duty cycles of 35% to 65%. To help with debugging of the receiver logic, the duty cycle of transmit data can become variable. This feature is achieved by adding mmio_cc_tx_cnt<7:0> register"]
    #[inline(always)]
    pub fn tx_cnt(&mut self) -> TX_CNT_W {
        TX_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "C-Connector Debug control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_cc_0](index.html) module"]
pub struct DEBUG_CC_0_SPEC;
impl crate::RegisterSpec for DEBUG_CC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_cc_0::R](R) reader structure"]
impl crate::Readable for DEBUG_CC_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_cc_0::W](W) writer structure"]
impl crate::Writable for DEBUG_CC_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_CC_0 to value 0"]
impl crate::Resettable for DEBUG_CC_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
