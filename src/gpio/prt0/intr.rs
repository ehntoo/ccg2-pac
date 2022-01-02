#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
pub struct DATA0_R(crate::FieldReader<bool, bool>);
impl DATA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA0` writer - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
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
#[doc = "Field `DATA1` reader - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
pub struct DATA1_R(crate::FieldReader<bool, bool>);
impl DATA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA1` writer - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
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
#[doc = "Field `FLT_DATA` reader - Deglitched interrupt pending (selected by FLT_SELECT)."]
pub struct FLT_DATA_R(crate::FieldReader<bool, bool>);
impl FLT_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_DATA` writer - Deglitched interrupt pending (selected by FLT_SELECT)."]
pub struct FLT_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_DATA_W<'a> {
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
#[doc = "Field `PS_DATA0` reader - `"]
pub struct PS_DATA0_R(crate::FieldReader<bool, bool>);
impl PS_DATA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PS_DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_DATA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS_DATA1` reader - "]
pub struct PS_DATA1_R(crate::FieldReader<bool, bool>);
impl PS_DATA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PS_DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_DATA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS_FLT_DATA` reader - This is a duplicate of the contents of the PS register, provided here to allow reading of both pin state and interrupt state of the port in a single read operation."]
pub struct PS_FLT_DATA_R(crate::FieldReader<bool, bool>);
impl PS_FLT_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PS_FLT_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_FLT_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Deglitched interrupt pending (selected by FLT_SELECT)."]
    #[inline(always)]
    pub fn flt_data(&self) -> FLT_DATA_R {
        FLT_DATA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - `"]
    #[inline(always)]
    pub fn ps_data0(&self) -> PS_DATA0_R {
        PS_DATA0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ps_data1(&self) -> PS_DATA1_R {
        PS_DATA1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This is a duplicate of the contents of the PS register, provided here to allow reading of both pin state and interrupt state of the port in a single read operation."]
    #[inline(always)]
    pub fn ps_flt_data(&self) -> PS_FLT_DATA_R {
        PS_FLT_DATA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
    #[doc = "Bit 8 - Deglitched interrupt pending (selected by FLT_SELECT)."]
    #[inline(always)]
    pub fn flt_data(&mut self) -> FLT_DATA_W {
        FLT_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port interrupt status register An interrupt cause is cleared (set to '0') by writing a '1' to the corresponding bit field. It is not recommended to write 0xff to clear all interrupt causes, as a new interrupt cause may have occurred between reading the register and clearing. Note that the interrupt cause fields and the associated interrupt provide Hibernate functionality (interrupt causes can be set to '1' and the interrupt can be activated in Hibernate power mode). The PS_DATA fields reflect the logical IO pad states of the port (also found in the PS register).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
