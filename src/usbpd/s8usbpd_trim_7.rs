#[doc = "Register `S8USBPD_TRIM_7` reader"]
pub struct R(crate::R<S8USBPD_TRIM_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S8USBPD_TRIM_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S8USBPD_TRIM_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S8USBPD_TRIM_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S8USBPD_TRIM_7` writer"]
pub struct W(crate::W<S8USBPD_TRIM_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S8USBPD_TRIM_7_SPEC>;
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
impl From<crate::W<S8USBPD_TRIM_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S8USBPD_TRIM_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I_TRIM` reader - DeepSleep 2.4uA current reference trim bit. Refer to s8usbpd BROS for bit settings."]
pub struct I_TRIM_R(crate::FieldReader<u8, u8>);
impl I_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I_TRIM` writer - DeepSleep 2.4uA current reference trim bit. Refer to s8usbpd BROS for bit settings."]
pub struct I_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> I_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DeepSleep 2.4uA current reference trim bit. Refer to s8usbpd BROS for bit settings."]
    #[inline(always)]
    pub fn i_trim(&self) -> I_TRIM_R {
        I_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DeepSleep 2.4uA current reference trim bit. Refer to s8usbpd BROS for bit settings."]
    #[inline(always)]
    pub fn i_trim(&mut self) -> I_TRIM_W {
        I_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD C-connector Trim Register7. Production trims stored in flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s8usbpd_trim_7](index.html) module"]
pub struct S8USBPD_TRIM_7_SPEC;
impl crate::RegisterSpec for S8USBPD_TRIM_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s8usbpd_trim_7::R](R) reader structure"]
impl crate::Readable for S8USBPD_TRIM_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s8usbpd_trim_7::W](W) writer structure"]
impl crate::Writable for S8USBPD_TRIM_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S8USBPD_TRIM_7 to value 0xf1"]
impl crate::Resettable for S8USBPD_TRIM_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf1
    }
}
