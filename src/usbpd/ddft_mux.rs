#[doc = "Register `DDFT_MUX` reader"]
pub struct R(crate::R<DDFT_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDFT_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDFT_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDFT_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDFT_MUX` writer"]
pub struct W(crate::W<DDFT_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDFT_MUX_SPEC>;
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
impl From<crate::W<DDFT_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDFT_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ddft0_sel` reader - "]
pub struct DDFT0_SEL_R(crate::FieldReader<u8, u8>);
impl DDFT0_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DDFT0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDFT0_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddft0_sel` writer - "]
pub struct DDFT0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `ddft1_sel` reader - "]
pub struct DDFT1_SEL_R(crate::FieldReader<u8, u8>);
impl DDFT1_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DDFT1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDFT1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddft1_sel` writer - "]
pub struct DDFT1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ddft0_sel(&self) -> DDFT0_SEL_R {
        DDFT0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn ddft1_sel(&self) -> DDFT1_SEL_R {
        DDFT1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ddft0_sel(&mut self) -> DDFT0_SEL_W {
        DDFT0_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn ddft1_sel(&mut self) -> DDFT1_SEL_W {
        DDFT1_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddft_mux](index.html) module"]
pub struct DDFT_MUX_SPEC;
impl crate::RegisterSpec for DDFT_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddft_mux::R](R) reader structure"]
impl crate::Readable for DDFT_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddft_mux::W](W) writer structure"]
impl crate::Writable for DDFT_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDFT_MUX to value 0"]
impl crate::Resettable for DDFT_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
