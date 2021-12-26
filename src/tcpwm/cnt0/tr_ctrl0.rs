#[doc = "Register `TR_CTRL0` reader"]
pub struct R(crate::R<TR_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTRL0` writer"]
pub struct W(crate::W<TR_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTRL0_SPEC>;
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
impl From<crate::W<TR_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE_SEL` reader - "]
pub struct CAPTURE_SEL_R(crate::FieldReader<u8, u8>);
impl CAPTURE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPTURE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTURE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE_SEL` writer - "]
pub struct CAPTURE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `COUNT_SEL` reader - "]
pub struct COUNT_SEL_R(crate::FieldReader<u8, u8>);
impl COUNT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT_SEL` writer - "]
pub struct COUNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `RELOAD_SEL` reader - "]
pub struct RELOAD_SEL_R(crate::FieldReader<u8, u8>);
impl RELOAD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RELOAD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOAD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOAD_SEL` writer - "]
pub struct RELOAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `STOP_SEL` reader - "]
pub struct STOP_SEL_R(crate::FieldReader<u8, u8>);
impl STOP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STOP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_SEL` writer - "]
pub struct STOP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `START_SEL` reader - "]
pub struct START_SEL_R(crate::FieldReader<u8, u8>);
impl START_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        START_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_SEL` writer - "]
pub struct START_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn capture_sel(&self) -> CAPTURE_SEL_R {
        CAPTURE_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn count_sel(&self) -> COUNT_SEL_R {
        COUNT_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reload_sel(&self) -> RELOAD_SEL_R {
        RELOAD_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn stop_sel(&self) -> STOP_SEL_R {
        STOP_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn start_sel(&self) -> START_SEL_R {
        START_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn capture_sel(&mut self) -> CAPTURE_SEL_W {
        CAPTURE_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn count_sel(&mut self) -> COUNT_SEL_W {
        COUNT_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reload_sel(&mut self) -> RELOAD_SEL_W {
        RELOAD_SEL_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn stop_sel(&mut self) -> STOP_SEL_W {
        STOP_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn start_sel(&mut self) -> START_SEL_W {
        START_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctrl0](index.html) module"]
pub struct TR_CTRL0_SPEC;
impl crate::RegisterSpec for TR_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctrl0::R](R) reader structure"]
impl crate::Readable for TR_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctrl0::W](W) writer structure"]
impl crate::Writable for TR_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_CTRL0 to value 0x10"]
impl crate::Resettable for TR_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
