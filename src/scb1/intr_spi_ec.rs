#[doc = "Register `INTR_SPI_EC` reader"]
pub struct R(crate::R<INTR_SPI_EC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPI_EC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPI_EC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPI_EC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SPI_EC` writer"]
pub struct W(crate::W<INTR_SPI_EC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPI_EC_SPEC>;
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
impl From<crate::W<INTR_SPI_EC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPI_EC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKE_UP` reader - "]
pub struct WAKE_UP_R(crate::FieldReader<bool, bool>);
impl WAKE_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKE_UP` writer - "]
pub struct WAKE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_UP_W<'a> {
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
#[doc = "Field `EZ_STOP` reader - "]
pub struct EZ_STOP_R(crate::FieldReader<bool, bool>);
impl EZ_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EZ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EZ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EZ_STOP` writer - "]
pub struct EZ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_STOP_W<'a> {
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
#[doc = "Field `EZ_WRITE_STOP` reader - "]
pub struct EZ_WRITE_STOP_R(crate::FieldReader<bool, bool>);
impl EZ_WRITE_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EZ_WRITE_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EZ_WRITE_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EZ_WRITE_STOP` writer - "]
pub struct EZ_WRITE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_WRITE_STOP_W<'a> {
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
#[doc = "Field `EZ_READ_STOP` reader - "]
pub struct EZ_READ_STOP_R(crate::FieldReader<bool, bool>);
impl EZ_READ_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EZ_READ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EZ_READ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EZ_READ_STOP` writer - "]
pub struct EZ_READ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_READ_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W {
        WAKE_UP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ez_stop(&mut self) -> EZ_STOP_W {
        EZ_STOP_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ez_write_stop(&mut self) -> EZ_WRITE_STOP_W {
        EZ_WRITE_STOP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ez_read_stop(&mut self) -> EZ_READ_STOP_W {
        EZ_READ_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_spi_ec](index.html) module"]
pub struct INTR_SPI_EC_SPEC;
impl crate::RegisterSpec for INTR_SPI_EC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_spi_ec::R](R) reader structure"]
impl crate::Readable for INTR_SPI_EC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_spi_ec::W](W) writer structure"]
impl crate::Writable for INTR_SPI_EC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SPI_EC to value 0"]
impl crate::Resettable for INTR_SPI_EC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
