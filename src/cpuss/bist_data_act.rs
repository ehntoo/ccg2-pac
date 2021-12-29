#[doc = "Register `BIST_DATA_ACT` reader"]
pub struct R(crate::R<BIST_DATA_ACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_DATA_ACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_DATA_ACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_DATA_ACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - "]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_data_act](index.html) module"]
pub struct BIST_DATA_ACT_SPEC;
impl crate::RegisterSpec for BIST_DATA_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_data_act::R](R) reader structure"]
impl crate::Readable for BIST_DATA_ACT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BIST_DATA_ACT to value 0"]
impl crate::Resettable for BIST_DATA_ACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
