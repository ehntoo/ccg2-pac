#[doc = "Register `INTR_RX_SET` reader"]
pub struct R(crate::R<INTR_RX_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RX_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RX_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RX_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_RX_SET` writer"]
pub struct W(crate::W<INTR_RX_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_RX_SET_SPEC>;
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
impl From<crate::W<INTR_RX_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_RX_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub struct TRIGGER_R(crate::FieldReader<bool, bool>);
impl TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
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
#[doc = "Field `NOT_EMPTY` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct NOT_EMPTY_R(crate::FieldReader<bool, bool>);
impl NOT_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOT_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOT_EMPTY` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct NOT_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> NOT_EMPTY_W<'a> {
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
#[doc = "Field `FULL` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct FULL_R(crate::FieldReader<bool, bool>);
impl FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
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
#[doc = "Field `OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct OVERFLOW_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
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
#[doc = "Field `UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl UNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_W<'a> {
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
#[doc = "Field `BLOCKED` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct BLOCKED_R(crate::FieldReader<bool, bool>);
impl BLOCKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCKED` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct BLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKED_W<'a> {
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
#[doc = "Field `FRAME_ERROR` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct FRAME_ERROR_R(crate::FieldReader<bool, bool>);
impl FRAME_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_ERROR` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct FRAME_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ERROR_W<'a> {
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
#[doc = "Field `PARITY_ERROR` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct PARITY_ERROR_R(crate::FieldReader<bool, bool>);
impl PARITY_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ERROR` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct PARITY_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ERROR_W<'a> {
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
#[doc = "Field `BAUD_DETECT` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct BAUD_DETECT_R(crate::FieldReader<bool, bool>);
impl BAUD_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BAUD_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD_DETECT` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct BAUD_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_DETECT_W<'a> {
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
#[doc = "Field `BREAK_DETECT` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub struct BREAK_DETECT_R(crate::FieldReader<bool, bool>);
impl BREAK_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BREAK_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAK_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK_DETECT` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub struct BREAK_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_DETECT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn not_empty(&mut self) -> NOT_EMPTY_W {
        NOT_EMPTY_W { w: self }
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn blocked(&mut self) -> BLOCKED_W {
        BLOCKED_W { w: self }
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn frame_error(&mut self) -> FRAME_ERROR_W {
        FRAME_ERROR_W { w: self }
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn parity_error(&mut self) -> PARITY_ERROR_W {
        PARITY_ERROR_W { w: self }
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn baud_detect(&mut self) -> BAUD_DETECT_W {
        BAUD_DETECT_W { w: self }
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn break_detect(&mut self) -> BREAK_DETECT_W {
        BREAK_DETECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver interrupt set request register. When read, this register reflects the interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_rx_set](index.html) module"]
pub struct INTR_RX_SET_SPEC;
impl crate::RegisterSpec for INTR_RX_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_rx_set::R](R) reader structure"]
impl crate::Readable for INTR_RX_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_rx_set::W](W) writer structure"]
impl crate::Writable for INTR_RX_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_RX_SET to value 0"]
impl crate::Resettable for INTR_RX_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
