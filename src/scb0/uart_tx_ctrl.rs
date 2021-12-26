#[doc = "Register `UART_TX_CTRL` reader"]
pub struct R(crate::R<UART_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TX_CTRL` writer"]
pub struct W(crate::W<UART_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TX_CTRL_SPEC>;
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
impl From<crate::W<UART_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TX_CTRL_SPEC>) -> Self {
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
#[doc = "Field `RETRY_ON_NACK` reader - "]
pub struct RETRY_ON_NACK_R(crate::FieldReader<bool, bool>);
impl RETRY_ON_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RETRY_ON_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRY_ON_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRY_ON_NACK` writer - "]
pub struct RETRY_ON_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRY_ON_NACK_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn retry_on_nack(&self) -> RETRY_ON_NACK_R {
        RETRY_ON_NACK_R::new(((self.bits >> 8) & 0x01) != 0)
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn retry_on_nack(&mut self) -> RETRY_ON_NACK_W {
        RETRY_ON_NACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tx_ctrl](index.html) module"]
pub struct UART_TX_CTRL_SPEC;
impl crate::RegisterSpec for UART_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tx_ctrl::R](R) reader structure"]
impl crate::Readable for UART_TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tx_ctrl::W](W) writer structure"]
impl crate::Writable for UART_TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TX_CTRL to value 0x02"]
impl crate::Resettable for UART_TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
