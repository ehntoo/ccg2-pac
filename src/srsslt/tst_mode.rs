#[doc = "Register `TST_MODE` reader"]
pub struct R(crate::R<TST_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_MODE` writer"]
pub struct W(crate::W<TST_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_MODE_SPEC>;
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
impl From<crate::W<TST_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWD_CONNECTED` reader - "]
pub struct SWD_CONNECTED_R(crate::FieldReader<bool, bool>);
impl SWD_CONNECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWD_CONNECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_CONNECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_ALT_XRES` reader - "]
pub struct BLOCK_ALT_XRES_R(crate::FieldReader<bool, bool>);
impl BLOCK_ALT_XRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLOCK_ALT_XRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_ALT_XRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_ALT_XRES` writer - "]
pub struct BLOCK_ALT_XRES_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_ALT_XRES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `TEST_KEY_DFT_EN` reader - "]
pub struct TEST_KEY_DFT_EN_R(crate::FieldReader<bool, bool>);
impl TEST_KEY_DFT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_KEY_DFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_KEY_DFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_MODE` reader - "]
pub struct TEST_MODE_R(crate::FieldReader<bool, bool>);
impl TEST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_MODE` writer - "]
pub struct TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_MODE_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swd_connected(&self) -> SWD_CONNECTED_R {
        SWD_CONNECTED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn block_alt_xres(&self) -> BLOCK_ALT_XRES_R {
        BLOCK_ALT_XRES_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn test_key_dft_en(&self) -> TEST_KEY_DFT_EN_R {
        TEST_KEY_DFT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn block_alt_xres(&mut self) -> BLOCK_ALT_XRES_W {
        BLOCK_ALT_XRES_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W {
        TEST_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_mode](index.html) module"]
pub struct TST_MODE_SPEC;
impl crate::RegisterSpec for TST_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst_mode::R](R) reader structure"]
impl crate::Readable for TST_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_mode::W](W) writer structure"]
impl crate::Writable for TST_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TST_MODE to value 0"]
impl crate::Resettable for TST_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
