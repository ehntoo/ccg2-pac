#[doc = "Register `PS` reader"]
pub struct R(crate::R<PS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - IO pad 0 state: 1: Logic high, if the pin voltage is above the input buffer threshold, logic high. 0: Logic low, if the pin voltage is below that threshold, logic low. If the drive mode for the pin is set to high Z Analog, the pin state will read 0 independent of the voltage on the pin."]
pub struct DATA0_R(crate::FieldReader<bool, bool>);
impl DATA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA1` reader - IO pad 1 state."]
pub struct DATA1_R(crate::FieldReader<bool, bool>);
impl DATA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_DATA` reader - Reads of this register return the logical state of the filtered pin."]
pub struct FLT_DATA_R(crate::FieldReader<bool, bool>);
impl FLT_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - IO pad 0 state: 1: Logic high, if the pin voltage is above the input buffer threshold, logic high. 0: Logic low, if the pin voltage is below that threshold, logic low. If the drive mode for the pin is set to high Z Analog, the pin state will read 0 independent of the voltage on the pin."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO pad 1 state."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reads of this register return the logical state of the filtered pin."]
    #[inline(always)]
    pub fn flt_data(&self) -> FLT_DATA_R {
        FLT_DATA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Port IO pad state register Used to read. Writes to this register have no effect. If the drive mode for the pin is set to high Z Analog, the state will read 0 independent of the voltage on the pin.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps](index.html) module"]
pub struct PS_SPEC;
impl crate::RegisterSpec for PS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps::R](R) reader structure"]
impl crate::Readable for PS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PS to value 0"]
impl crate::Resettable for PS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
