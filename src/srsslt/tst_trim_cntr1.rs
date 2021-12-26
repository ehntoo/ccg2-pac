#[doc = "Register `TST_TRIM_CNTR1` reader"]
pub struct R(crate::R<TST_TRIM_CNTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_TRIM_CNTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_TRIM_CNTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_TRIM_CNTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_TRIM_CNTR1` writer"]
pub struct W(crate::W<TST_TRIM_CNTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_TRIM_CNTR1_SPEC>;
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
impl From<crate::W<TST_TRIM_CNTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_TRIM_CNTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - "]
pub struct COUNTER_R(crate::FieldReader<u16, u16>);
impl COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER` writer - "]
pub struct COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `COUNTER_DONE` reader - "]
pub struct COUNTER_DONE_R(crate::FieldReader<bool, bool>);
impl COUNTER_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COUNTER_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn counter_done(&self) -> COUNTER_DONE_R {
        COUNTER_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W {
        COUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_trim_cntr1](index.html) module"]
pub struct TST_TRIM_CNTR1_SPEC;
impl crate::RegisterSpec for TST_TRIM_CNTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst_trim_cntr1::R](R) reader structure"]
impl crate::Readable for TST_TRIM_CNTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_trim_cntr1::W](W) writer structure"]
impl crate::Writable for TST_TRIM_CNTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TST_TRIM_CNTR1 to value 0x8000_0000"]
impl crate::Resettable for TST_TRIM_CNTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
