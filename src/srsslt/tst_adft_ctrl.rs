#[doc = "Register `TST_ADFT_CTRL` reader"]
pub struct R(crate::R<TST_ADFT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_ADFT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_ADFT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_ADFT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_ADFT_CTRL` writer"]
pub struct W(crate::W<TST_ADFT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_ADFT_CTRL_SPEC>;
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
impl From<crate::W<TST_ADFT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_ADFT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF_AUTO_ZERO` reader - "]
pub struct BUF_AUTO_ZERO_R(crate::FieldReader<bool, bool>);
impl BUF_AUTO_ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_AUTO_ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_AUTO_ZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF_AUTO_ZERO` writer - "]
pub struct BUF_AUTO_ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_AUTO_ZERO_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `BUF_MODE` reader - "]
pub struct BUF_MODE_R(crate::FieldReader<u8, u8>);
impl BUF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF_MODE` writer - "]
pub struct BUF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `BUF_COMP_OUT` reader - "]
pub struct BUF_COMP_OUT_R(crate::FieldReader<bool, bool>);
impl BUF_COMP_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_COMP_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_COMP_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF_EN` reader - "]
pub struct BUF_EN_R(crate::FieldReader<bool, bool>);
impl BUF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF_EN` writer - "]
pub struct BUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_EN_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn buf_auto_zero(&self) -> BUF_AUTO_ZERO_R {
        BUF_AUTO_ZERO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn buf_mode(&self) -> BUF_MODE_R {
        BUF_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn buf_comp_out(&self) -> BUF_COMP_OUT_R {
        BUF_COMP_OUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn buf_en(&self) -> BUF_EN_R {
        BUF_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn buf_auto_zero(&mut self) -> BUF_AUTO_ZERO_W {
        BUF_AUTO_ZERO_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn buf_mode(&mut self) -> BUF_MODE_W {
        BUF_MODE_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn buf_en(&mut self) -> BUF_EN_W {
        BUF_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_adft_ctrl](index.html) module"]
pub struct TST_ADFT_CTRL_SPEC;
impl crate::RegisterSpec for TST_ADFT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst_adft_ctrl::R](R) reader structure"]
impl crate::Readable for TST_ADFT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_adft_ctrl::W](W) writer structure"]
impl crate::Writable for TST_ADFT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TST_ADFT_CTRL to value 0"]
impl crate::Resettable for TST_ADFT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
