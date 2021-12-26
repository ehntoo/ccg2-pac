#[doc = "Register `TST_TRIM_CNTR2` reader"]
pub struct R(crate::R<TST_TRIM_CNTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_TRIM_CNTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_TRIM_CNTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_TRIM_CNTR2_SPEC>) -> Self {
        R(reader)
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
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_trim_cntr2](index.html) module"]
pub struct TST_TRIM_CNTR2_SPEC;
impl crate::RegisterSpec for TST_TRIM_CNTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst_trim_cntr2::R](R) reader structure"]
impl crate::Readable for TST_TRIM_CNTR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TST_TRIM_CNTR2 to value 0"]
impl crate::Resettable for TST_TRIM_CNTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
