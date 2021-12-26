#[doc = "Register `S8USBPD_TRIM_0` reader"]
pub struct R(crate::R<S8USBPD_TRIM_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S8USBPD_TRIM_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S8USBPD_TRIM_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S8USBPD_TRIM_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S8USBPD_TRIM_0` writer"]
pub struct W(crate::W<S8USBPD_TRIM_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S8USBPD_TRIM_0_SPEC>;
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
impl From<crate::W<S8USBPD_TRIM_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S8USBPD_TRIM_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZDRV_TRIM` reader - "]
pub struct ZDRV_TRIM_R(crate::FieldReader<u8, u8>);
impl ZDRV_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ZDRV_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZDRV_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZDRV_TRIM` writer - "]
pub struct ZDRV_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ZDRV_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TX_TRIM` reader - "]
pub struct TX_TRIM_R(crate::FieldReader<u8, u8>);
impl TX_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TRIM` writer - "]
pub struct TX_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `RD_TRIM` reader - "]
pub struct RD_TRIM_R(crate::FieldReader<u8, u8>);
impl RD_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_TRIM` writer - "]
pub struct RD_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn zdrv_trim(&self) -> ZDRV_TRIM_R {
        ZDRV_TRIM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_trim(&self) -> TX_TRIM_R {
        TX_TRIM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rd_trim(&self) -> RD_TRIM_R {
        RD_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn zdrv_trim(&mut self) -> ZDRV_TRIM_W {
        ZDRV_TRIM_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_trim(&mut self) -> TX_TRIM_W {
        TX_TRIM_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rd_trim(&mut self) -> RD_TRIM_W {
        RD_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s8usbpd_trim_0](index.html) module"]
pub struct S8USBPD_TRIM_0_SPEC;
impl crate::RegisterSpec for S8USBPD_TRIM_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s8usbpd_trim_0::R](R) reader structure"]
impl crate::Readable for S8USBPD_TRIM_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s8usbpd_trim_0::W](W) writer structure"]
impl crate::Writable for S8USBPD_TRIM_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S8USBPD_TRIM_0 to value 0x40"]
impl crate::Resettable for S8USBPD_TRIM_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
