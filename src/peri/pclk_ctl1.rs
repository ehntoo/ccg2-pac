#[doc = "Register `PCLK_CTL1` reader"]
pub struct R(crate::R<PCLK_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLK_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLK_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLK_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLK_CTL1` writer"]
pub struct W(crate::W<PCLK_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLK_CTL1_SPEC>;
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
impl From<crate::W<PCLK_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLK_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL_DIV` reader - "]
pub struct SEL_DIV_R(crate::FieldReader<u8, u8>);
impl SEL_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_DIV` writer - "]
pub struct SEL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SEL_TYPE` reader - "]
pub struct SEL_TYPE_R(crate::FieldReader<u8, u8>);
impl SEL_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_TYPE` writer - "]
pub struct SEL_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sel_div(&self) -> SEL_DIV_R {
        SEL_DIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sel_type(&self) -> SEL_TYPE_R {
        SEL_TYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sel_div(&mut self) -> SEL_DIV_W {
        SEL_DIV_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sel_type(&mut self) -> SEL_TYPE_W {
        SEL_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclk_ctl1](index.html) module"]
pub struct PCLK_CTL1_SPEC;
impl crate::RegisterSpec for PCLK_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclk_ctl1::R](R) reader structure"]
impl crate::Readable for PCLK_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclk_ctl1::W](W) writer structure"]
impl crate::Writable for PCLK_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCLK_CTL1 to value 0xc3"]
impl crate::Resettable for PCLK_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc3
    }
}
