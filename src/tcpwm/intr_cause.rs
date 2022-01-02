#[doc = "Register `INTR_CAUSE` reader"]
pub struct R(crate::R<INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNTER_INT` reader - Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
pub struct COUNTER_INT_R(crate::FieldReader<u8, u8>);
impl COUNTER_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_INT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_int(&self) -> COUNTER_INT_R {
        COUNTER_INT_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "TCPWM Counter interrupt cause register. Enables software to determine the source of the combined interrupt output signal \"interrupt\". The register fields are not retained. This is to ensure that they come up as '0' after coming out of DeepSleep system power mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
pub struct INTR_CAUSE_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for INTR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
