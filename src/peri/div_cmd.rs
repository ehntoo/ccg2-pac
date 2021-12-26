#[doc = "Register `DIV_CMD` reader"]
pub struct R(crate::R<DIV_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_CMD` writer"]
pub struct W(crate::W<DIV_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_CMD_SPEC>;
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
impl From<crate::W<DIV_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL_DIV` reader - "]
pub struct SEL_DIV_R(crate::FieldReader<u8, u8>);
impl SEL_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_DIV` writer - "]
pub struct SEL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `SEL_TYPE` reader - "]
pub struct SEL_TYPE_R(crate::FieldReader<u8, u8>);
impl SEL_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_TYPE` writer - "]
pub struct SEL_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PA_SEL_DIV` reader - "]
pub struct PA_SEL_DIV_R(crate::FieldReader<u8, u8>);
impl PA_SEL_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_SEL_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_SEL_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_SEL_DIV` writer - "]
pub struct PA_SEL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SEL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `PA_SEL_TYPE` reader - "]
pub struct PA_SEL_TYPE_R(crate::FieldReader<u8, u8>);
impl PA_SEL_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_SEL_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_SEL_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_SEL_TYPE` writer - "]
pub struct PA_SEL_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SEL_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `DISABLE` reader - "]
pub struct DISABLE_R(crate::FieldReader<bool, bool>);
impl DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE` writer - "]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - "]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - "]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn sel_div(&self) -> SEL_DIV_R {
        SEL_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sel_type(&self) -> SEL_TYPE_R {
        SEL_TYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pa_sel_div(&self) -> PA_SEL_DIV_R {
        PA_SEL_DIV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pa_sel_type(&self) -> PA_SEL_TYPE_R {
        PA_SEL_TYPE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn sel_div(&mut self) -> SEL_DIV_W {
        SEL_DIV_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sel_type(&mut self) -> SEL_TYPE_W {
        SEL_TYPE_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pa_sel_div(&mut self) -> PA_SEL_DIV_W {
        PA_SEL_DIV_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pa_sel_type(&mut self) -> PA_SEL_TYPE_W {
        PA_SEL_TYPE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_cmd](index.html) module"]
pub struct DIV_CMD_SPEC;
impl crate::RegisterSpec for DIV_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_cmd::R](R) reader structure"]
impl crate::Readable for DIV_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_cmd::W](W) writer structure"]
impl crate::Writable for DIV_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_CMD to value 0xffff"]
impl crate::Resettable for DIV_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
