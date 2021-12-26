#[doc = "Register `INTR0_MASKED` reader"]
pub struct R(crate::R<INTR0_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR0_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR0_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR0_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCV_GOOD_PACKET_COMPLETE_MASKED` reader - "]
pub struct RCV_GOOD_PACKET_COMPLETE_MASKED_R(crate::FieldReader<bool, bool>);
impl RCV_GOOD_PACKET_COMPLETE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_GOOD_PACKET_COMPLETE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_GOOD_PACKET_COMPLETE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_BAD_PACKET_COMPLETE_MASKED` reader - "]
pub struct RCV_BAD_PACKET_COMPLETE_MASKED_R(crate::FieldReader<bool, bool>);
impl RCV_BAD_PACKET_COMPLETE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_BAD_PACKET_COMPLETE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_BAD_PACKET_COMPLETE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SOP_MASKED` reader - "]
pub struct RX_SOP_MASKED_R(crate::FieldReader<bool, bool>);
impl RX_SOP_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SOP_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SOP_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_GOODCRC_MSG_COMPLETE_MASKED` reader - "]
pub struct RCV_GOODCRC_MSG_COMPLETE_MASKED_R(crate::FieldReader<bool, bool>);
impl RCV_GOODCRC_MSG_COMPLETE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_GOODCRC_MSG_COMPLETE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_GOODCRC_MSG_COMPLETE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_EXPT_GOODCRC_MSG_COMPLETE_MASKED` reader - "]
pub struct RCV_EXPT_GOODCRC_MSG_COMPLETE_MASKED_R(crate::FieldReader<bool, bool>);
impl RCV_EXPT_GOODCRC_MSG_COMPLETE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_EXPT_GOODCRC_MSG_COMPLETE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_EXPT_GOODCRC_MSG_COMPLETE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOP_ERROR_MASKED` reader - "]
pub struct EOP_ERROR_MASKED_R(crate::FieldReader<bool, bool>);
impl EOP_ERROR_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOP_ERROR_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOP_ERROR_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OVER_RUN_MASKED` reader - "]
pub struct RX_OVER_RUN_MASKED_R(crate::FieldReader<bool, bool>);
impl RX_OVER_RUN_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OVER_RUN_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_OVER_RUN_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PACKET_DONE_MASKED` reader - "]
pub struct TX_PACKET_DONE_MASKED_R(crate::FieldReader<bool, bool>);
impl TX_PACKET_DONE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PACKET_DONE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PACKET_DONE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HARD_RST_DONE_MASKED` reader - "]
pub struct TX_HARD_RST_DONE_MASKED_R(crate::FieldReader<bool, bool>);
impl TX_HARD_RST_DONE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_HARD_RST_DONE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HARD_RST_DONE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_RST_MASKED` reader - "]
pub struct RCV_RST_MASKED_R(crate::FieldReader<bool, bool>);
impl RCV_RST_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_RST_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_RST_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_DONE_MASKED` reader - "]
pub struct SAR_DONE_MASKED_R(crate::FieldReader<bool, bool>);
impl SAR_DONE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_DONE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_DONE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_GOODCRC_MSG_DONE_MASKED` reader - "]
pub struct TX_GOODCRC_MSG_DONE_MASKED_R(crate::FieldReader<bool, bool>);
impl TX_GOODCRC_MSG_DONE_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_GOODCRC_MSG_DONE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_GOODCRC_MSG_DONE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_VALID_DATA_DETECTED_MASKED` reader - "]
pub struct CC_VALID_DATA_DETECTED_MASKED_R(crate::FieldReader<bool, bool>);
impl CC_VALID_DATA_DETECTED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_VALID_DATA_DETECTED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_VALID_DATA_DETECTED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_NO_VALID_DATA_DETECTED_MASKED` reader - "]
pub struct CC_NO_VALID_DATA_DETECTED_MASKED_R(crate::FieldReader<bool, bool>);
impl CC_NO_VALID_DATA_DETECTED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_NO_VALID_DATA_DETECTED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_NO_VALID_DATA_DETECTED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_RX_TIMER_EXP_MASKED` reader - "]
pub struct CRC_RX_TIMER_EXP_MASKED_R(crate::FieldReader<bool, bool>);
impl CRC_RX_TIMER_EXP_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_RX_TIMER_EXP_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_RX_TIMER_EXP_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE1_MASKED` reader - "]
pub struct COLLISION_TYPE1_MASKED_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE1_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE1_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE1_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE2_MASKED` reader - "]
pub struct COLLISION_TYPE2_MASKED_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE2_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE2_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE2_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE3_MASKED` reader - "]
pub struct COLLISION_TYPE3_MASKED_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE3_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE3_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE3_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE4_MASKED` reader - "]
pub struct COLLISION_TYPE4_MASKED_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE4_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE4_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE4_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_OUT_DET_MASKED` reader - "]
pub struct CMP_OUT_DET_MASKED_R(crate::FieldReader<bool, bool>);
impl CMP_OUT_DET_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_OUT_DET_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_OUT_DET_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SRAM_HALF_END_MASKED` reader - "]
pub struct TX_SRAM_HALF_END_MASKED_R(crate::FieldReader<bool, bool>);
impl TX_SRAM_HALF_END_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SRAM_HALF_END_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SRAM_HALF_END_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SRAM_HALF_END_MASKED` reader - "]
pub struct RX_SRAM_HALF_END_MASKED_R(crate::FieldReader<bool, bool>);
impl RX_SRAM_HALF_END_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SRAM_HALF_END_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SRAM_HALF_END_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CC_DATA_OEN_MASKED` reader - "]
pub struct TX_CC_DATA_OEN_MASKED_R(crate::FieldReader<bool, bool>);
impl TX_CC_DATA_OEN_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CC_DATA_OEN_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CC_DATA_OEN_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KCHAR_ERROR_MASKED` reader - "]
pub struct KCHAR_ERROR_MASKED_R(crate::FieldReader<bool, bool>);
impl KCHAR_ERROR_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KCHAR_ERROR_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCHAR_ERROR_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOISE_ON_PKT_MASKED` reader - "]
pub struct NOISE_ON_PKT_MASKED_R(crate::FieldReader<bool, bool>);
impl NOISE_ON_PKT_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOISE_ON_PKT_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOISE_ON_PKT_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_VALID_DATA_NOISE_DETECTED_MASKED` reader - "]
pub struct CC_VALID_DATA_NOISE_DETECTED_MASKED_R(crate::FieldReader<bool, bool>);
impl CC_VALID_DATA_NOISE_DETECTED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_VALID_DATA_NOISE_DETECTED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_VALID_DATA_NOISE_DETECTED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_NO_VALID_DATA_NOISE_DETECTED_MASKED` reader - "]
pub struct CC_NO_VALID_DATA_NOISE_DETECTED_MASKED_R(crate::FieldReader<bool, bool>);
impl CC_NO_VALID_DATA_NOISE_DETECTED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_NO_VALID_DATA_NOISE_DETECTED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_NO_VALID_DATA_NOISE_DETECTED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcv_good_packet_complete_masked(&self) -> RCV_GOOD_PACKET_COMPLETE_MASKED_R {
        RCV_GOOD_PACKET_COMPLETE_MASKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rcv_bad_packet_complete_masked(&self) -> RCV_BAD_PACKET_COMPLETE_MASKED_R {
        RCV_BAD_PACKET_COMPLETE_MASKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_sop_masked(&self) -> RX_SOP_MASKED_R {
        RX_SOP_MASKED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rcv_goodcrc_msg_complete_masked(&self) -> RCV_GOODCRC_MSG_COMPLETE_MASKED_R {
        RCV_GOODCRC_MSG_COMPLETE_MASKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rcv_expt_goodcrc_msg_complete_masked(&self) -> RCV_EXPT_GOODCRC_MSG_COMPLETE_MASKED_R {
        RCV_EXPT_GOODCRC_MSG_COMPLETE_MASKED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn eop_error_masked(&self) -> EOP_ERROR_MASKED_R {
        EOP_ERROR_MASKED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_over_run_masked(&self) -> RX_OVER_RUN_MASKED_R {
        RX_OVER_RUN_MASKED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_packet_done_masked(&self) -> TX_PACKET_DONE_MASKED_R {
        TX_PACKET_DONE_MASKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_hard_rst_done_masked(&self) -> TX_HARD_RST_DONE_MASKED_R {
        TX_HARD_RST_DONE_MASKED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rcv_rst_masked(&self) -> RCV_RST_MASKED_R {
        RCV_RST_MASKED_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sar_done_masked(&self) -> SAR_DONE_MASKED_R {
        SAR_DONE_MASKED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_goodcrc_msg_done_masked(&self) -> TX_GOODCRC_MSG_DONE_MASKED_R {
        TX_GOODCRC_MSG_DONE_MASKED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cc_valid_data_detected_masked(&self) -> CC_VALID_DATA_DETECTED_MASKED_R {
        CC_VALID_DATA_DETECTED_MASKED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cc_no_valid_data_detected_masked(&self) -> CC_NO_VALID_DATA_DETECTED_MASKED_R {
        CC_NO_VALID_DATA_DETECTED_MASKED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn crc_rx_timer_exp_masked(&self) -> CRC_RX_TIMER_EXP_MASKED_R {
        CRC_RX_TIMER_EXP_MASKED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn collision_type1_masked(&self) -> COLLISION_TYPE1_MASKED_R {
        COLLISION_TYPE1_MASKED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn collision_type2_masked(&self) -> COLLISION_TYPE2_MASKED_R {
        COLLISION_TYPE2_MASKED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn collision_type3_masked(&self) -> COLLISION_TYPE3_MASKED_R {
        COLLISION_TYPE3_MASKED_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn collision_type4_masked(&self) -> COLLISION_TYPE4_MASKED_R {
        COLLISION_TYPE4_MASKED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp_out_det_masked(&self) -> CMP_OUT_DET_MASKED_R {
        CMP_OUT_DET_MASKED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_sram_half_end_masked(&self) -> TX_SRAM_HALF_END_MASKED_R {
        TX_SRAM_HALF_END_MASKED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_sram_half_end_masked(&self) -> RX_SRAM_HALF_END_MASKED_R {
        RX_SRAM_HALF_END_MASKED_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tx_cc_data_oen_masked(&self) -> TX_CC_DATA_OEN_MASKED_R {
        TX_CC_DATA_OEN_MASKED_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn kchar_error_masked(&self) -> KCHAR_ERROR_MASKED_R {
        KCHAR_ERROR_MASKED_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn noise_on_pkt_masked(&self) -> NOISE_ON_PKT_MASKED_R {
        NOISE_ON_PKT_MASKED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cc_valid_data_noise_detected_masked(&self) -> CC_VALID_DATA_NOISE_DETECTED_MASKED_R {
        CC_VALID_DATA_NOISE_DETECTED_MASKED_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cc_no_valid_data_noise_detected_masked(
        &self,
    ) -> CC_NO_VALID_DATA_NOISE_DETECTED_MASKED_R {
        CC_NO_VALID_DATA_NOISE_DETECTED_MASKED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr0_masked](index.html) module"]
pub struct INTR0_MASKED_SPEC;
impl crate::RegisterSpec for INTR0_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr0_masked::R](R) reader structure"]
impl crate::Readable for INTR0_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR0_MASKED to value 0"]
impl crate::Resettable for INTR0_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
