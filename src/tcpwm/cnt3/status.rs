#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOWN` reader - When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
pub struct DOWN_R(crate::FieldReader<bool, bool>);
impl DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENERIC` reader - Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
pub struct GENERIC_R(crate::FieldReader<u8, u8>);
impl GENERIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERIC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNNING` reader - When '0', the counter is NOT running. When '1', the counter is running."]
pub struct RUNNING_R(crate::FieldReader<bool, bool>);
impl RUNNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub fn generic(&self) -> GENERIC_R {
        GENERIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - When '0', the counter is NOT running. When '1', the counter is running."]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Counter status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
