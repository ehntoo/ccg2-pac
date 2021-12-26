#[doc = "Register `DIV_16_5_CTL0` reader"]
pub struct R(crate::R<DIV_16_5_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_16_5_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_16_5_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_16_5_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_16_5_CTL0` writer"]
pub struct W(crate::W<DIV_16_5_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_16_5_CTL0_SPEC>;
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
impl From<crate::W<DIV_16_5_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_16_5_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - "]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAC5_DIV` reader - "]
pub struct FRAC5_DIV_R(crate::FieldReader<u8, u8>);
impl FRAC5_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAC5_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAC5_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAC5_DIV` writer - "]
pub struct FRAC5_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC5_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `INT16_DIV` reader - "]
pub struct INT16_DIV_R(crate::FieldReader<u16, u16>);
impl INT16_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INT16_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT16_DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT16_DIV` writer - "]
pub struct INT16_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INT16_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn frac5_div(&self) -> FRAC5_DIV_R {
        FRAC5_DIV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn int16_div(&self) -> INT16_DIV_R {
        INT16_DIV_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn frac5_div(&mut self) -> FRAC5_DIV_W {
        FRAC5_DIV_W { w: self }
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn int16_div(&mut self) -> INT16_DIV_W {
        INT16_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_16_5_ctl0](index.html) module"]
pub struct DIV_16_5_CTL0_SPEC;
impl crate::RegisterSpec for DIV_16_5_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_16_5_ctl0::R](R) reader structure"]
impl crate::Readable for DIV_16_5_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_16_5_ctl0::W](W) writer structure"]
impl crate::Writable for DIV_16_5_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_16_5_CTL0 to value 0"]
impl crate::Resettable for DIV_16_5_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
