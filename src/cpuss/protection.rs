#[doc = "Register `PROTECTION` reader"]
pub struct R(crate::R<PROTECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROTECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROTECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROTECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROTECTION` writer"]
pub struct W(crate::W<PROTECTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROTECTION_SPEC>;
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
impl From<crate::W<PROTECTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROTECTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTECTION_MODE` reader - "]
pub struct PROTECTION_MODE_R(crate::FieldReader<u8, u8>);
impl PROTECTION_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROTECTION_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROTECTION_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTECTION_MODE` writer - "]
pub struct PROTECTION_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTECTION_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `FLASH_LOCK` reader - "]
pub struct FLASH_LOCK_R(crate::FieldReader<bool, bool>);
impl FLASH_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_LOCK` writer - "]
pub struct FLASH_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_LOCK_W<'a> {
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
#[doc = "Field `PROTECTION_LOCK` reader - "]
pub struct PROTECTION_LOCK_R(crate::FieldReader<bool, bool>);
impl PROTECTION_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROTECTION_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROTECTION_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTECTION_LOCK` writer - "]
pub struct PROTECTION_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTECTION_LOCK_W<'a> {
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
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn protection_mode(&self) -> PROTECTION_MODE_R {
        PROTECTION_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn flash_lock(&self) -> FLASH_LOCK_R {
        FLASH_LOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn protection_lock(&self) -> PROTECTION_LOCK_R {
        PROTECTION_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn protection_mode(&mut self) -> PROTECTION_MODE_W {
        PROTECTION_MODE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn flash_lock(&mut self) -> FLASH_LOCK_W {
        FLASH_LOCK_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn protection_lock(&mut self) -> PROTECTION_LOCK_W {
        PROTECTION_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protection](index.html) module"]
pub struct PROTECTION_SPEC;
impl crate::RegisterSpec for PROTECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [protection::R](R) reader structure"]
impl crate::Readable for PROTECTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [protection::W](W) writer structure"]
impl crate::Writable for PROTECTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROTECTION to value 0x0f"]
impl crate::Resettable for PROTECTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
