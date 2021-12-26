#[doc = "Register `WDT_MATCH` reader"]
pub struct R(crate::R<WDT_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_MATCH` writer"]
pub struct W(crate::W<WDT_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_MATCH_SPEC>;
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
impl From<crate::W<WDT_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - "]
pub struct MATCH_R(crate::FieldReader<u16, u16>);
impl MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCH` writer - "]
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `IGNORE_BITS` reader - "]
pub struct IGNORE_BITS_R(crate::FieldReader<u8, u8>);
impl IGNORE_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IGNORE_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGNORE_BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNORE_BITS` writer - "]
pub struct IGNORE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn ignore_bits(&self) -> IGNORE_BITS_R {
        IGNORE_BITS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn ignore_bits(&mut self) -> IGNORE_BITS_W {
        IGNORE_BITS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_match](index.html) module"]
pub struct WDT_MATCH_SPEC;
impl crate::RegisterSpec for WDT_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_match::R](R) reader structure"]
impl crate::Readable for WDT_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_match::W](W) writer structure"]
impl crate::Writable for WDT_MATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_MATCH to value 0x1000"]
impl crate::Resettable for WDT_MATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
