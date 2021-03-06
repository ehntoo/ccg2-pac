#[doc = "Register `RES_DFT` reader"]
pub struct R(crate::R<RES_DFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_DFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_DFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_DFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES_DFT` writer"]
pub struct W(crate::W<RES_DFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_DFT_SPEC>;
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
impl From<crate::W<RES_DFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_DFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY_BLOCK` reader - "]
pub struct DELAY_BLOCK_R(crate::FieldReader<bool, bool>);
impl DELAY_BLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DELAY_BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_BLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELAY_BLOCK` writer - "]
pub struct DELAY_BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_BLOCK_W<'a> {
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
#[doc = "Field `DELAY_IN` reader - "]
pub struct DELAY_IN_R(crate::FieldReader<bool, bool>);
impl DELAY_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DELAY_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELAY_IN` writer - "]
pub struct DELAY_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_IN_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn delay_block(&self) -> DELAY_BLOCK_R {
        DELAY_BLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn delay_in(&self) -> DELAY_IN_R {
        DELAY_IN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn delay_block(&mut self) -> DELAY_BLOCK_W {
        DELAY_BLOCK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn delay_in(&mut self) -> DELAY_IN_W {
        DELAY_IN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_dft](index.html) module"]
pub struct RES_DFT_SPEC;
impl crate::RegisterSpec for RES_DFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res_dft::R](R) reader structure"]
impl crate::Readable for RES_DFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res_dft::W](W) writer structure"]
impl crate::Writable for RES_DFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES_DFT to value 0"]
impl crate::Resettable for RES_DFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
