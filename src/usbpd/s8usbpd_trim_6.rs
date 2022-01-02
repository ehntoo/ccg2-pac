#[doc = "Register `S8USBPD_TRIM_6` reader"]
pub struct R(crate::R<S8USBPD_TRIM_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S8USBPD_TRIM_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S8USBPD_TRIM_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S8USBPD_TRIM_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S8USBPD_TRIM_6` writer"]
pub struct W(crate::W<S8USBPD_TRIM_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S8USBPD_TRIM_6_SPEC>;
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
impl From<crate::W<S8USBPD_TRIM_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S8USBPD_TRIM_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `V1P575_TRIM` reader - Trim bits for 1.575V comparator reference"]
pub struct V1P575_TRIM_R(crate::FieldReader<u8, u8>);
impl V1P575_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        V1P575_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V1P575_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V1P575_TRIM` writer - Trim bits for 1.575V comparator reference"]
pub struct V1P575_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> V1P575_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `REF_TRIM` reader - Beta multiplier reference trim bits. Refer to GPM-541 for trim settings."]
pub struct REF_TRIM_R(crate::FieldReader<u8, u8>);
impl REF_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REF_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_TRIM` writer - Beta multiplier reference trim bits. Refer to GPM-541 for trim settings."]
pub struct REF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Trim bits for 1.575V comparator reference"]
    #[inline(always)]
    pub fn v1p575_trim(&self) -> V1P575_TRIM_R {
        V1P575_TRIM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:6 - Beta multiplier reference trim bits. Refer to GPM-541 for trim settings."]
    #[inline(always)]
    pub fn ref_trim(&self) -> REF_TRIM_R {
        REF_TRIM_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Trim bits for 1.575V comparator reference"]
    #[inline(always)]
    pub fn v1p575_trim(&mut self) -> V1P575_TRIM_W {
        V1P575_TRIM_W { w: self }
    }
    #[doc = "Bits 3:6 - Beta multiplier reference trim bits. Refer to GPM-541 for trim settings."]
    #[inline(always)]
    pub fn ref_trim(&mut self) -> REF_TRIM_W {
        REF_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD C-connector Trim Register6. Production trims stored in flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s8usbpd_trim_6](index.html) module"]
pub struct S8USBPD_TRIM_6_SPEC;
impl crate::RegisterSpec for S8USBPD_TRIM_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s8usbpd_trim_6::R](R) reader structure"]
impl crate::Readable for S8USBPD_TRIM_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s8usbpd_trim_6::W](W) writer structure"]
impl crate::Writable for S8USBPD_TRIM_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S8USBPD_TRIM_6 to value 0"]
impl crate::Resettable for S8USBPD_TRIM_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
