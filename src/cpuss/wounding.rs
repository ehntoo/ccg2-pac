#[doc = "Register `WOUNDING` reader"]
pub struct R(crate::R<WOUNDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WOUNDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WOUNDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WOUNDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WOUNDING` writer"]
pub struct W(crate::W<WOUNDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WOUNDING_SPEC>;
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
impl From<crate::W<WOUNDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WOUNDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_WOUND` reader - "]
pub struct RAM_WOUND_R(crate::FieldReader<u8, u8>);
impl RAM_WOUND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_WOUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_WOUND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_WOUND` writer - "]
pub struct RAM_WOUND_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_WOUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `FLASH_WOUND` reader - "]
pub struct FLASH_WOUND_R(crate::FieldReader<u8, u8>);
impl FLASH_WOUND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_WOUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_WOUND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_WOUND` writer - "]
pub struct FLASH_WOUND_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_WOUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ram_wound(&self) -> RAM_WOUND_R {
        RAM_WOUND_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn flash_wound(&self) -> FLASH_WOUND_R {
        FLASH_WOUND_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ram_wound(&mut self) -> RAM_WOUND_W {
        RAM_WOUND_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn flash_wound(&mut self) -> FLASH_WOUND_W {
        FLASH_WOUND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wounding](index.html) module"]
pub struct WOUNDING_SPEC;
impl crate::RegisterSpec for WOUNDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wounding::R](R) reader structure"]
impl crate::Readable for WOUNDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wounding::W](W) writer structure"]
impl crate::Writable for WOUNDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WOUNDING to value 0"]
impl crate::Resettable for WOUNDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
