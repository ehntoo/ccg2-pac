#[doc = "Register `PWR_PWRSYS_TRIM1` reader"]
pub struct R(crate::R<PWR_PWRSYS_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_PWRSYS_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_PWRSYS_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_PWRSYS_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_PWRSYS_TRIM1` writer"]
pub struct W(crate::W<PWR_PWRSYS_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_PWRSYS_TRIM1_SPEC>;
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
impl From<crate::W<PWR_PWRSYS_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_PWRSYS_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPSLP_REF_TRIM` reader - Trims the DeepSleep reference that is used by the DeepSleep regulator and DeepSleep power comparator."]
pub struct DPSLP_REF_TRIM_R(crate::FieldReader<u8, u8>);
impl DPSLP_REF_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DPSLP_REF_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_REF_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_REF_TRIM` writer - Trims the DeepSleep reference that is used by the DeepSleep regulator and DeepSleep power comparator."]
pub struct DPSLP_REF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `SPARE_TRIM` reader - Active-Reference temperature compensation trim (repurposed from spare bits). Bits \\[7:6\\]
- trim the Active-Reference IREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = +80ppm/C 10: TC = -80ppm/C 11: TC = -150ppm/C Bits \\[5:4\\]
- trim the Active-Reference VREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = -50ppm/C 10: TC = -80ppm/C 11: TC = +150ppm/C"]
pub struct SPARE_TRIM_R(crate::FieldReader<u8, u8>);
impl SPARE_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPARE_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE_TRIM` writer - Active-Reference temperature compensation trim (repurposed from spare bits). Bits \\[7:6\\]
- trim the Active-Reference IREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = +80ppm/C 10: TC = -80ppm/C 11: TC = -150ppm/C Bits \\[5:4\\]
- trim the Active-Reference VREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = -50ppm/C 10: TC = -80ppm/C 11: TC = +150ppm/C"]
pub struct SPARE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Trims the DeepSleep reference that is used by the DeepSleep regulator and DeepSleep power comparator."]
    #[inline(always)]
    pub fn dpslp_ref_trim(&self) -> DPSLP_REF_TRIM_R {
        DPSLP_REF_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Active-Reference temperature compensation trim (repurposed from spare bits). Bits \\[7:6\\]
- trim the Active-Reference IREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = +80ppm/C 10: TC = -80ppm/C 11: TC = -150ppm/C Bits \\[5:4\\]
- trim the Active-Reference VREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = -50ppm/C 10: TC = -80ppm/C 11: TC = +150ppm/C"]
    #[inline(always)]
    pub fn spare_trim(&self) -> SPARE_TRIM_R {
        SPARE_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trims the DeepSleep reference that is used by the DeepSleep regulator and DeepSleep power comparator."]
    #[inline(always)]
    pub fn dpslp_ref_trim(&mut self) -> DPSLP_REF_TRIM_W {
        DPSLP_REF_TRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - Active-Reference temperature compensation trim (repurposed from spare bits). Bits \\[7:6\\]
- trim the Active-Reference IREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = +80ppm/C 10: TC = -80ppm/C 11: TC = -150ppm/C Bits \\[5:4\\]
- trim the Active-Reference VREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = -50ppm/C 10: TC = -80ppm/C 11: TC = +150ppm/C"]
    #[inline(always)]
    pub fn spare_trim(&mut self) -> SPARE_TRIM_W {
        SPARE_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power System Trim Register Power System Trim Bits. Entire register is engineering only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_pwrsys_trim1](index.html) module"]
pub struct PWR_PWRSYS_TRIM1_SPEC;
impl crate::RegisterSpec for PWR_PWRSYS_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_pwrsys_trim1::R](R) reader structure"]
impl crate::Readable for PWR_PWRSYS_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_pwrsys_trim1::W](W) writer structure"]
impl crate::Writable for PWR_PWRSYS_TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_PWRSYS_TRIM1 to value 0"]
impl crate::Resettable for PWR_PWRSYS_TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
