#[doc = "Register `PWR_BG_TRIM1` reader"]
pub struct R(crate::R<PWR_BG_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BG_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BG_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BG_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_BG_TRIM1` writer"]
pub struct W(crate::W<PWR_BG_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_BG_TRIM1_SPEC>;
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
impl From<crate::W<PWR_BG_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_BG_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_VTRIM` reader - "]
pub struct REF_VTRIM_R(crate::FieldReader<u8, u8>);
impl REF_VTRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REF_VTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_VTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_VTRIM` writer - "]
pub struct REF_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ref_vtrim(&self) -> REF_VTRIM_R {
        REF_VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ref_vtrim(&mut self) -> REF_VTRIM_W {
        REF_VTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_bg_trim1](index.html) module"]
pub struct PWR_BG_TRIM1_SPEC;
impl crate::RegisterSpec for PWR_BG_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_bg_trim1::R](R) reader structure"]
impl crate::Readable for PWR_BG_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_bg_trim1::W](W) writer structure"]
impl crate::Writable for PWR_BG_TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_BG_TRIM1 to value 0x10"]
impl crate::Resettable for PWR_BG_TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
