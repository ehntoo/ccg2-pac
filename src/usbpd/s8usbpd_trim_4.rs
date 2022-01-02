#[doc = "Register `S8USBPD_TRIM_4` reader"]
pub struct R(crate::R<S8USBPD_TRIM_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S8USBPD_TRIM_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S8USBPD_TRIM_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S8USBPD_TRIM_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S8USBPD_TRIM_4` writer"]
pub struct W(crate::W<S8USBPD_TRIM_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S8USBPD_TRIM_4_SPEC>;
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
impl From<crate::W<S8USBPD_TRIM_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S8USBPD_TRIM_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `V0P74_TRIM` reader - Trim bits for 0.74V comparator reference"]
pub struct V0P74_TRIM_R(crate::FieldReader<u8, u8>);
impl V0P74_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        V0P74_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V0P74_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V0P74_TRIM` writer - Trim bits for 0.74V comparator reference"]
pub struct V0P74_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> V0P74_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `V0P8_TRIM` reader - Trim bits for 0.8V comparator reference"]
pub struct V0P8_TRIM_R(crate::FieldReader<u8, u8>);
impl V0P8_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        V0P8_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V0P8_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V0P8_TRIM` writer - Trim bits for 0.8V comparator reference"]
pub struct V0P8_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> V0P8_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Trim bits for 0.74V comparator reference"]
    #[inline(always)]
    pub fn v0p74_trim(&self) -> V0P74_TRIM_R {
        V0P74_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Trim bits for 0.8V comparator reference"]
    #[inline(always)]
    pub fn v0p8_trim(&self) -> V0P8_TRIM_R {
        V0P8_TRIM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trim bits for 0.74V comparator reference"]
    #[inline(always)]
    pub fn v0p74_trim(&mut self) -> V0P74_TRIM_W {
        V0P74_TRIM_W { w: self }
    }
    #[doc = "Bits 4:6 - Trim bits for 0.8V comparator reference"]
    #[inline(always)]
    pub fn v0p8_trim(&mut self) -> V0P8_TRIM_W {
        V0P8_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD C-connector Trim Register4. Production trims stored in flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s8usbpd_trim_4](index.html) module"]
pub struct S8USBPD_TRIM_4_SPEC;
impl crate::RegisterSpec for S8USBPD_TRIM_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s8usbpd_trim_4::R](R) reader structure"]
impl crate::Readable for S8USBPD_TRIM_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s8usbpd_trim_4::W](W) writer structure"]
impl crate::Writable for S8USBPD_TRIM_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S8USBPD_TRIM_4 to value 0"]
impl crate::Resettable for S8USBPD_TRIM_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
