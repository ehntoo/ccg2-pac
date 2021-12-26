#[doc = "Register `S8USBPD_TRIM_1` reader"]
pub struct R(crate::R<S8USBPD_TRIM_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S8USBPD_TRIM_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S8USBPD_TRIM_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S8USBPD_TRIM_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S8USBPD_TRIM_1` writer"]
pub struct W(crate::W<S8USBPD_TRIM_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S8USBPD_TRIM_1_SPEC>;
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
impl From<crate::W<S8USBPD_TRIM_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S8USBPD_TRIM_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RP_CC1_TRIM` reader - "]
pub struct RP_CC1_TRIM_R(crate::FieldReader<u8, u8>);
impl RP_CC1_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RP_CC1_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RP_CC1_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RP_CC1_TRIM` writer - "]
pub struct RP_CC1_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RP_CC1_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `TC_I_TRIM_9P` reader - "]
pub struct TC_I_TRIM_9P_R(crate::FieldReader<u8, u8>);
impl TC_I_TRIM_9P_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TC_I_TRIM_9P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC_I_TRIM_9P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC_I_TRIM_9P` writer - "]
pub struct TC_I_TRIM_9P_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_I_TRIM_9P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rp_cc1_trim(&self) -> RP_CC1_TRIM_R {
        RP_CC1_TRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tc_i_trim_9p(&self) -> TC_I_TRIM_9P_R {
        TC_I_TRIM_9P_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rp_cc1_trim(&mut self) -> RP_CC1_TRIM_W {
        RP_CC1_TRIM_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tc_i_trim_9p(&mut self) -> TC_I_TRIM_9P_W {
        TC_I_TRIM_9P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s8usbpd_trim_1](index.html) module"]
pub struct S8USBPD_TRIM_1_SPEC;
impl crate::RegisterSpec for S8USBPD_TRIM_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s8usbpd_trim_1::R](R) reader structure"]
impl crate::Readable for S8USBPD_TRIM_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s8usbpd_trim_1::W](W) writer structure"]
impl crate::Writable for S8USBPD_TRIM_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S8USBPD_TRIM_1 to value 0"]
impl crate::Resettable for S8USBPD_TRIM_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
