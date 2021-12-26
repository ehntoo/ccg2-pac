#[doc = "Register `UART_RX_CTRL` reader"]
pub struct R(crate::R<UART_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_RX_CTRL` writer"]
pub struct W(crate::W<UART_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RX_CTRL_SPEC>;
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
impl From<crate::W<UART_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_BITS` reader - "]
pub struct STOP_BITS_R(crate::FieldReader<u8, u8>);
impl STOP_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STOP_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_BITS` writer - "]
pub struct STOP_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `PARITY` reader - "]
pub struct PARITY_R(crate::FieldReader<bool, bool>);
impl PARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY` writer - "]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
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
#[doc = "Field `PARITY_ENABLED` reader - "]
pub struct PARITY_ENABLED_R(crate::FieldReader<bool, bool>);
impl PARITY_ENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ENABLED` writer - "]
pub struct PARITY_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ENABLED_W<'a> {
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
#[doc = "Field `POLARITY` reader - "]
pub struct POLARITY_R(crate::FieldReader<bool, bool>);
impl POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLARITY` writer - "]
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
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
#[doc = "Field `DROP_ON_PARITY_ERROR` reader - "]
pub struct DROP_ON_PARITY_ERROR_R(crate::FieldReader<bool, bool>);
impl DROP_ON_PARITY_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DROP_ON_PARITY_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DROP_ON_PARITY_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DROP_ON_PARITY_ERROR` writer - "]
pub struct DROP_ON_PARITY_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DROP_ON_PARITY_ERROR_W<'a> {
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
#[doc = "Field `DROP_ON_FRAME_ERROR` reader - "]
pub struct DROP_ON_FRAME_ERROR_R(crate::FieldReader<bool, bool>);
impl DROP_ON_FRAME_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DROP_ON_FRAME_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DROP_ON_FRAME_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DROP_ON_FRAME_ERROR` writer - "]
pub struct DROP_ON_FRAME_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DROP_ON_FRAME_ERROR_W<'a> {
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
#[doc = "Field `MP_MODE` reader - "]
pub struct MP_MODE_R(crate::FieldReader<bool, bool>);
impl MP_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MP_MODE` writer - "]
pub struct MP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MP_MODE_W<'a> {
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
#[doc = "Field `LIN_MODE` reader - "]
pub struct LIN_MODE_R(crate::FieldReader<bool, bool>);
impl LIN_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIN_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIN_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIN_MODE` writer - "]
pub struct LIN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LIN_MODE_W<'a> {
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
#[doc = "Field `SKIP_START` reader - "]
pub struct SKIP_START_R(crate::FieldReader<bool, bool>);
impl SKIP_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SKIP_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKIP_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKIP_START` writer - "]
pub struct SKIP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_START_W<'a> {
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
#[doc = "Field `BREAK_WIDTH` reader - "]
pub struct BREAK_WIDTH_R(crate::FieldReader<u8, u8>);
impl BREAK_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BREAK_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAK_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK_WIDTH` writer - "]
pub struct BREAK_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn drop_on_parity_error(&self) -> DROP_ON_PARITY_ERROR_R {
        DROP_ON_PARITY_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn drop_on_frame_error(&self) -> DROP_ON_FRAME_ERROR_R {
        DROP_ON_FRAME_ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mp_mode(&self) -> MP_MODE_R {
        MP_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lin_mode(&self) -> LIN_MODE_R {
        LIN_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn skip_start(&self) -> SKIP_START_R {
        SKIP_START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn break_width(&self) -> BREAK_WIDTH_R {
        BREAK_WIDTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> STOP_BITS_W {
        STOP_BITS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W {
        PARITY_ENABLED_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn drop_on_parity_error(&mut self) -> DROP_ON_PARITY_ERROR_W {
        DROP_ON_PARITY_ERROR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn drop_on_frame_error(&mut self) -> DROP_ON_FRAME_ERROR_W {
        DROP_ON_FRAME_ERROR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mp_mode(&mut self) -> MP_MODE_W {
        MP_MODE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lin_mode(&mut self) -> LIN_MODE_W {
        LIN_MODE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn skip_start(&mut self) -> SKIP_START_W {
        SKIP_START_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn break_width(&mut self) -> BREAK_WIDTH_W {
        BREAK_WIDTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rx_ctrl](index.html) module"]
pub struct UART_RX_CTRL_SPEC;
impl crate::RegisterSpec for UART_RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rx_ctrl::R](R) reader structure"]
impl crate::Readable for UART_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_rx_ctrl::W](W) writer structure"]
impl crate::Writable for UART_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_RX_CTRL to value 0x000a_0002"]
impl crate::Resettable for UART_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_0002
    }
}
