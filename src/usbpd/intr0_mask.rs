#[doc = "Register `INTR0_MASK` reader"]
pub struct R(crate::R<INTR0_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR0_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR0_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR0_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR0_MASK` writer"]
pub struct W(crate::W<INTR0_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR0_MASK_SPEC>;
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
impl From<crate::W<INTR0_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR0_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCV_GOOD_PACKET_COMPLETE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_GOOD_PACKET_COMPLETE_MASK_R(crate::FieldReader<bool, bool>);
impl RCV_GOOD_PACKET_COMPLETE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_GOOD_PACKET_COMPLETE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_GOOD_PACKET_COMPLETE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_GOOD_PACKET_COMPLETE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_GOOD_PACKET_COMPLETE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_GOOD_PACKET_COMPLETE_MASK_W<'a> {
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
#[doc = "Field `RCV_BAD_PACKET_COMPLETE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_BAD_PACKET_COMPLETE_MASK_R(crate::FieldReader<bool, bool>);
impl RCV_BAD_PACKET_COMPLETE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_BAD_PACKET_COMPLETE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_BAD_PACKET_COMPLETE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_BAD_PACKET_COMPLETE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_BAD_PACKET_COMPLETE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_BAD_PACKET_COMPLETE_MASK_W<'a> {
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
#[doc = "Field `RX_SOP_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RX_SOP_MASK_R(crate::FieldReader<bool, bool>);
impl RX_SOP_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SOP_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SOP_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SOP_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RX_SOP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SOP_MASK_W<'a> {
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
#[doc = "Field `RCV_GOODCRC_MSG_COMPLETE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_GOODCRC_MSG_COMPLETE_MASK_R(crate::FieldReader<bool, bool>);
impl RCV_GOODCRC_MSG_COMPLETE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_GOODCRC_MSG_COMPLETE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_GOODCRC_MSG_COMPLETE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_GOODCRC_MSG_COMPLETE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_GOODCRC_MSG_COMPLETE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_GOODCRC_MSG_COMPLETE_MASK_W<'a> {
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
#[doc = "Field `RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_R(crate::FieldReader<bool, bool>);
impl RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_W<'a> {
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
#[doc = "Field `EOP_ERROR_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct EOP_ERROR_MASK_R(crate::FieldReader<bool, bool>);
impl EOP_ERROR_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOP_ERROR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOP_ERROR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOP_ERROR_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct EOP_ERROR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_ERROR_MASK_W<'a> {
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
#[doc = "Field `RX_OVER_RUN_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RX_OVER_RUN_MASK_R(crate::FieldReader<bool, bool>);
impl RX_OVER_RUN_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OVER_RUN_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_OVER_RUN_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OVER_RUN_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RX_OVER_RUN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVER_RUN_MASK_W<'a> {
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
#[doc = "Field `TX_PACKET_DONE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_PACKET_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl TX_PACKET_DONE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PACKET_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PACKET_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PACKET_DONE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_PACKET_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PACKET_DONE_MASK_W<'a> {
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
#[doc = "Field `TX_HARD_RST_DONE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_HARD_RST_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl TX_HARD_RST_DONE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_HARD_RST_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HARD_RST_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HARD_RST_DONE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_HARD_RST_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HARD_RST_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RCV_RST_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_RST_MASK_R(crate::FieldReader<bool, bool>);
impl RCV_RST_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCV_RST_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_RST_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_RST_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RCV_RST_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_RST_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SAR_DONE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct SAR_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl SAR_DONE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_DONE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct SAR_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TX_GOODCRC_MSG_DONE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_GOODCRC_MSG_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl TX_GOODCRC_MSG_DONE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_GOODCRC_MSG_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_GOODCRC_MSG_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_GOODCRC_MSG_DONE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_GOODCRC_MSG_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_GOODCRC_MSG_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CC_VALID_DATA_DETECTED_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_VALID_DATA_DETECTED_MASK_R(crate::FieldReader<bool, bool>);
impl CC_VALID_DATA_DETECTED_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_VALID_DATA_DETECTED_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_VALID_DATA_DETECTED_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_VALID_DATA_DETECTED_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_VALID_DATA_DETECTED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_VALID_DATA_DETECTED_MASK_W<'a> {
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
#[doc = "Field `CC_NO_VALID_DATA_DETECTED_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_NO_VALID_DATA_DETECTED_MASK_R(crate::FieldReader<bool, bool>);
impl CC_NO_VALID_DATA_DETECTED_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_NO_VALID_DATA_DETECTED_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_NO_VALID_DATA_DETECTED_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_NO_VALID_DATA_DETECTED_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_NO_VALID_DATA_DETECTED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_NO_VALID_DATA_DETECTED_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CRC_RX_TIMER_EXP_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct CRC_RX_TIMER_EXP_MASK_R(crate::FieldReader<bool, bool>);
impl CRC_RX_TIMER_EXP_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_RX_TIMER_EXP_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_RX_TIMER_EXP_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_RX_TIMER_EXP_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct CRC_RX_TIMER_EXP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_RX_TIMER_EXP_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `COLLISION_TYPE1_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE1_MASK_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE1_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE1_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE1_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE1_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE1_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLISION_TYPE1_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `COLLISION_TYPE2_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE2_MASK_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE2_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE2_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE2_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE2_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE2_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLISION_TYPE2_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `COLLISION_TYPE3_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE3_MASK_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE3_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE3_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE3_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE3_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE3_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLISION_TYPE3_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `COLLISION_TYPE4_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE4_MASK_R(crate::FieldReader<bool, bool>);
impl COLLISION_TYPE4_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_TYPE4_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLISION_TYPE4_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION_TYPE4_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct COLLISION_TYPE4_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLISION_TYPE4_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CMP_OUT_DET_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct CMP_OUT_DET_MASK_R(crate::FieldReader<bool, bool>);
impl CMP_OUT_DET_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_OUT_DET_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_OUT_DET_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_OUT_DET_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct CMP_OUT_DET_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_OUT_DET_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TX_SRAM_HALF_END_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_SRAM_HALF_END_MASK_R(crate::FieldReader<bool, bool>);
impl TX_SRAM_HALF_END_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SRAM_HALF_END_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SRAM_HALF_END_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SRAM_HALF_END_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_SRAM_HALF_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SRAM_HALF_END_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RX_SRAM_HALF_END_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct RX_SRAM_HALF_END_MASK_R(crate::FieldReader<bool, bool>);
impl RX_SRAM_HALF_END_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SRAM_HALF_END_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SRAM_HALF_END_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SRAM_HALF_END_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct RX_SRAM_HALF_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SRAM_HALF_END_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TX_CC_DATA_OEN_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_CC_DATA_OEN_MASK_R(crate::FieldReader<bool, bool>);
impl TX_CC_DATA_OEN_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CC_DATA_OEN_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CC_DATA_OEN_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CC_DATA_OEN_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct TX_CC_DATA_OEN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CC_DATA_OEN_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `KCHAR_ERROR_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct KCHAR_ERROR_MASK_R(crate::FieldReader<bool, bool>);
impl KCHAR_ERROR_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KCHAR_ERROR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCHAR_ERROR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KCHAR_ERROR_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct KCHAR_ERROR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> KCHAR_ERROR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `NOISE_ON_PKT_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct NOISE_ON_PKT_MASK_R(crate::FieldReader<bool, bool>);
impl NOISE_ON_PKT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOISE_ON_PKT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOISE_ON_PKT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOISE_ON_PKT_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct NOISE_ON_PKT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> NOISE_ON_PKT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CC_VALID_DATA_NOISE_DETECTED_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_VALID_DATA_NOISE_DETECTED_MASK_R(crate::FieldReader<bool, bool>);
impl CC_VALID_DATA_NOISE_DETECTED_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_VALID_DATA_NOISE_DETECTED_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_VALID_DATA_NOISE_DETECTED_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_VALID_DATA_NOISE_DETECTED_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_VALID_DATA_NOISE_DETECTED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_VALID_DATA_NOISE_DETECTED_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CC_NO_VALID_DATA_NOISE_DETECTED_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_NO_VALID_DATA_NOISE_DETECTED_MASK_R(crate::FieldReader<bool, bool>);
impl CC_NO_VALID_DATA_NOISE_DETECTED_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_NO_VALID_DATA_NOISE_DETECTED_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_NO_VALID_DATA_NOISE_DETECTED_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_NO_VALID_DATA_NOISE_DETECTED_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub struct CC_NO_VALID_DATA_NOISE_DETECTED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_NO_VALID_DATA_NOISE_DETECTED_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_good_packet_complete_mask(&self) -> RCV_GOOD_PACKET_COMPLETE_MASK_R {
        RCV_GOOD_PACKET_COMPLETE_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_bad_packet_complete_mask(&self) -> RCV_BAD_PACKET_COMPLETE_MASK_R {
        RCV_BAD_PACKET_COMPLETE_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_sop_mask(&self) -> RX_SOP_MASK_R {
        RX_SOP_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_goodcrc_msg_complete_mask(&self) -> RCV_GOODCRC_MSG_COMPLETE_MASK_R {
        RCV_GOODCRC_MSG_COMPLETE_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_expt_goodcrc_msg_complete_mask(&self) -> RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_R {
        RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eop_error_mask(&self) -> EOP_ERROR_MASK_R {
        EOP_ERROR_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_over_run_mask(&self) -> RX_OVER_RUN_MASK_R {
        RX_OVER_RUN_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_packet_done_mask(&self) -> TX_PACKET_DONE_MASK_R {
        TX_PACKET_DONE_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_hard_rst_done_mask(&self) -> TX_HARD_RST_DONE_MASK_R {
        TX_HARD_RST_DONE_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_rst_mask(&self) -> RCV_RST_MASK_R {
        RCV_RST_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sar_done_mask(&self) -> SAR_DONE_MASK_R {
        SAR_DONE_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_goodcrc_msg_done_mask(&self) -> TX_GOODCRC_MSG_DONE_MASK_R {
        TX_GOODCRC_MSG_DONE_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_valid_data_detected_mask(&self) -> CC_VALID_DATA_DETECTED_MASK_R {
        CC_VALID_DATA_DETECTED_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_no_valid_data_detected_mask(&self) -> CC_NO_VALID_DATA_DETECTED_MASK_R {
        CC_NO_VALID_DATA_DETECTED_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn crc_rx_timer_exp_mask(&self) -> CRC_RX_TIMER_EXP_MASK_R {
        CRC_RX_TIMER_EXP_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type1_mask(&self) -> COLLISION_TYPE1_MASK_R {
        COLLISION_TYPE1_MASK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type2_mask(&self) -> COLLISION_TYPE2_MASK_R {
        COLLISION_TYPE2_MASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type3_mask(&self) -> COLLISION_TYPE3_MASK_R {
        COLLISION_TYPE3_MASK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type4_mask(&self) -> COLLISION_TYPE4_MASK_R {
        COLLISION_TYPE4_MASK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cmp_out_det_mask(&self) -> CMP_OUT_DET_MASK_R {
        CMP_OUT_DET_MASK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_sram_half_end_mask(&self) -> TX_SRAM_HALF_END_MASK_R {
        TX_SRAM_HALF_END_MASK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_sram_half_end_mask(&self) -> RX_SRAM_HALF_END_MASK_R {
        RX_SRAM_HALF_END_MASK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cc_data_oen_mask(&self) -> TX_CC_DATA_OEN_MASK_R {
        TX_CC_DATA_OEN_MASK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn kchar_error_mask(&self) -> KCHAR_ERROR_MASK_R {
        KCHAR_ERROR_MASK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn noise_on_pkt_mask(&self) -> NOISE_ON_PKT_MASK_R {
        NOISE_ON_PKT_MASK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_valid_data_noise_detected_mask(&self) -> CC_VALID_DATA_NOISE_DETECTED_MASK_R {
        CC_VALID_DATA_NOISE_DETECTED_MASK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_no_valid_data_noise_detected_mask(&self) -> CC_NO_VALID_DATA_NOISE_DETECTED_MASK_R {
        CC_NO_VALID_DATA_NOISE_DETECTED_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_good_packet_complete_mask(&mut self) -> RCV_GOOD_PACKET_COMPLETE_MASK_W {
        RCV_GOOD_PACKET_COMPLETE_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_bad_packet_complete_mask(&mut self) -> RCV_BAD_PACKET_COMPLETE_MASK_W {
        RCV_BAD_PACKET_COMPLETE_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_sop_mask(&mut self) -> RX_SOP_MASK_W {
        RX_SOP_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_goodcrc_msg_complete_mask(&mut self) -> RCV_GOODCRC_MSG_COMPLETE_MASK_W {
        RCV_GOODCRC_MSG_COMPLETE_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_expt_goodcrc_msg_complete_mask(&mut self) -> RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_W {
        RCV_EXPT_GOODCRC_MSG_COMPLETE_MASK_W { w: self }
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eop_error_mask(&mut self) -> EOP_ERROR_MASK_W {
        EOP_ERROR_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_over_run_mask(&mut self) -> RX_OVER_RUN_MASK_W {
        RX_OVER_RUN_MASK_W { w: self }
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_packet_done_mask(&mut self) -> TX_PACKET_DONE_MASK_W {
        TX_PACKET_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_hard_rst_done_mask(&mut self) -> TX_HARD_RST_DONE_MASK_W {
        TX_HARD_RST_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcv_rst_mask(&mut self) -> RCV_RST_MASK_W {
        RCV_RST_MASK_W { w: self }
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sar_done_mask(&mut self) -> SAR_DONE_MASK_W {
        SAR_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_goodcrc_msg_done_mask(&mut self) -> TX_GOODCRC_MSG_DONE_MASK_W {
        TX_GOODCRC_MSG_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 12 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_valid_data_detected_mask(&mut self) -> CC_VALID_DATA_DETECTED_MASK_W {
        CC_VALID_DATA_DETECTED_MASK_W { w: self }
    }
    #[doc = "Bit 13 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_no_valid_data_detected_mask(&mut self) -> CC_NO_VALID_DATA_DETECTED_MASK_W {
        CC_NO_VALID_DATA_DETECTED_MASK_W { w: self }
    }
    #[doc = "Bit 14 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn crc_rx_timer_exp_mask(&mut self) -> CRC_RX_TIMER_EXP_MASK_W {
        CRC_RX_TIMER_EXP_MASK_W { w: self }
    }
    #[doc = "Bit 15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type1_mask(&mut self) -> COLLISION_TYPE1_MASK_W {
        COLLISION_TYPE1_MASK_W { w: self }
    }
    #[doc = "Bit 16 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type2_mask(&mut self) -> COLLISION_TYPE2_MASK_W {
        COLLISION_TYPE2_MASK_W { w: self }
    }
    #[doc = "Bit 17 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type3_mask(&mut self) -> COLLISION_TYPE3_MASK_W {
        COLLISION_TYPE3_MASK_W { w: self }
    }
    #[doc = "Bit 18 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn collision_type4_mask(&mut self) -> COLLISION_TYPE4_MASK_W {
        COLLISION_TYPE4_MASK_W { w: self }
    }
    #[doc = "Bit 19 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cmp_out_det_mask(&mut self) -> CMP_OUT_DET_MASK_W {
        CMP_OUT_DET_MASK_W { w: self }
    }
    #[doc = "Bit 20 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_sram_half_end_mask(&mut self) -> TX_SRAM_HALF_END_MASK_W {
        TX_SRAM_HALF_END_MASK_W { w: self }
    }
    #[doc = "Bit 21 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_sram_half_end_mask(&mut self) -> RX_SRAM_HALF_END_MASK_W {
        RX_SRAM_HALF_END_MASK_W { w: self }
    }
    #[doc = "Bit 22 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cc_data_oen_mask(&mut self) -> TX_CC_DATA_OEN_MASK_W {
        TX_CC_DATA_OEN_MASK_W { w: self }
    }
    #[doc = "Bit 23 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn kchar_error_mask(&mut self) -> KCHAR_ERROR_MASK_W {
        KCHAR_ERROR_MASK_W { w: self }
    }
    #[doc = "Bit 24 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn noise_on_pkt_mask(&mut self) -> NOISE_ON_PKT_MASK_W {
        NOISE_ON_PKT_MASK_W { w: self }
    }
    #[doc = "Bit 25 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_valid_data_noise_detected_mask(&mut self) -> CC_VALID_DATA_NOISE_DETECTED_MASK_W {
        CC_VALID_DATA_NOISE_DETECTED_MASK_W { w: self }
    }
    #[doc = "Bit 26 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn cc_no_valid_data_noise_detected_mask(
        &mut self,
    ) -> CC_NO_VALID_DATA_NOISE_DETECTED_MASK_W {
        CC_NO_VALID_DATA_NOISE_DETECTED_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTR0 Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr0_mask](index.html) module"]
pub struct INTR0_MASK_SPEC;
impl crate::RegisterSpec for INTR0_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr0_mask::R](R) reader structure"]
impl crate::Readable for INTR0_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr0_mask::W](W) writer structure"]
impl crate::Writable for INTR0_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR0_MASK to value 0"]
impl crate::Resettable for INTR0_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
