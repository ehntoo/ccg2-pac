#[doc = "Register `INTR1_MASKED` reader"]
pub struct R(crate::R<INTR1_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR1_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR1_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR1_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCONN1_CHANGED_MASKED` reader - "]
pub struct VCONN1_CHANGED_MASKED_R(crate::FieldReader<bool, bool>);
impl VCONN1_CHANGED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN1_CHANGED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN1_CHANGED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN2_CHANGED_MASKED` reader - "]
pub struct VCONN2_CHANGED_MASKED_R(crate::FieldReader<bool, bool>);
impl VCONN2_CHANGED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN2_CHANGED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN2_CHANGED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1_CHANGED_MASKED` reader - "]
pub struct CC1_CHANGED_MASKED_R(crate::FieldReader<bool, bool>);
impl CC1_CHANGED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC1_CHANGED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1_CHANGED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2_CHANGED_MASKED` reader - "]
pub struct CC2_CHANGED_MASKED_R(crate::FieldReader<bool, bool>);
impl CC2_CHANGED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC2_CHANGED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2_CHANGED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_LA_CHANGED_MASKED` reader - "]
pub struct VCMP_LA_CHANGED_MASKED_R(crate::FieldReader<bool, bool>);
impl VCMP_LA_CHANGED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_LA_CHANGED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_LA_CHANGED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_UP_CHANGED_MASKED` reader - "]
pub struct VCMP_UP_CHANGED_MASKED_R(crate::FieldReader<bool, bool>);
impl VCMP_UP_CHANGED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_UP_CHANGED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_UP_CHANGED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_DN_CHANGED_MASKED` reader - "]
pub struct VCMP_DN_CHANGED_MASKED_R(crate::FieldReader<bool, bool>);
impl VCMP_DN_CHANGED_MASKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_DN_CHANGED_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_DN_CHANGED_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vconn1_changed_masked(&self) -> VCONN1_CHANGED_MASKED_R {
        VCONN1_CHANGED_MASKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vconn2_changed_masked(&self) -> VCONN2_CHANGED_MASKED_R {
        VCONN2_CHANGED_MASKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc1_changed_masked(&self) -> CC1_CHANGED_MASKED_R {
        CC1_CHANGED_MASKED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc2_changed_masked(&self) -> CC2_CHANGED_MASKED_R {
        CC2_CHANGED_MASKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vcmp_la_changed_masked(&self) -> VCMP_LA_CHANGED_MASKED_R {
        VCMP_LA_CHANGED_MASKED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vcmp_up_changed_masked(&self) -> VCMP_UP_CHANGED_MASKED_R {
        VCMP_UP_CHANGED_MASKED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vcmp_dn_changed_masked(&self) -> VCMP_DN_CHANGED_MASKED_R {
        VCMP_DN_CHANGED_MASKED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr1_masked](index.html) module"]
pub struct INTR1_MASKED_SPEC;
impl crate::RegisterSpec for INTR1_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr1_masked::R](R) reader structure"]
impl crate::Readable for INTR1_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR1_MASKED to value 0"]
impl crate::Resettable for INTR1_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
