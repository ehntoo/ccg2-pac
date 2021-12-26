#[doc = "Register `CRC_COUNTER` reader"]
pub struct R(crate::R<CRC_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_COUNTER` writer"]
pub struct W(crate::W<CRC_COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_COUNTER_SPEC>;
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
impl From<crate::W<CRC_COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_COUNTER` reader - "]
pub struct CRC_COUNTER_R(crate::FieldReader<u16, u16>);
impl CRC_COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CRC_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_COUNTER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_COUNTER` writer - "]
pub struct CRC_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn crc_counter(&self) -> CRC_COUNTER_R {
        CRC_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn crc_counter(&mut self) -> CRC_COUNTER_W {
        CRC_COUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_counter](index.html) module"]
pub struct CRC_COUNTER_SPEC;
impl crate::RegisterSpec for CRC_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_counter::R](R) reader structure"]
impl crate::Readable for CRC_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_counter::W](W) writer structure"]
impl crate::Writable for CRC_COUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_COUNTER to value 0"]
impl crate::Resettable for CRC_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
