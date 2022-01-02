#[doc = "Register `FLASH_CTL` reader"]
pub struct R(crate::R<FLASH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_CTL` writer"]
pub struct W(crate::W<FLASH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CTL_SPEC>;
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
impl From<crate::W<FLASH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_WS` reader - Amount of ROM wait states: \"0\": 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) \"1\": 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) \"2\": 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) \"3\": undefined"]
pub struct FLASH_WS_R(crate::FieldReader<u8, u8>);
impl FLASH_WS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_WS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_WS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_WS` writer - Amount of ROM wait states: \"0\": 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) \"1\": 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) \"2\": 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) \"3\": undefined"]
pub struct FLASH_WS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_WS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PREF_EN` reader - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is \"0\" or when predictable execution behavior is required. '1': enabled."]
pub struct PREF_EN_R(crate::FieldReader<bool, bool>);
impl PREF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREF_EN` writer - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is \"0\" or when predictable execution behavior is required. '1': enabled."]
pub struct PREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FLASH_INVALIDATE` reader - 1': Invalidates the content of the flash controller's buffers."]
pub struct FLASH_INVALIDATE_R(crate::FieldReader<bool, bool>);
impl FLASH_INVALIDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_INVALIDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_INVALIDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_INVALIDATE` writer - 1': Invalidates the content of the flash controller's buffers."]
pub struct FLASH_INVALIDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_INVALIDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Amount of ROM wait states: \"0\": 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) \"1\": 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) \"2\": 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) \"3\": undefined"]
    #[inline(always)]
    pub fn flash_ws(&self) -> FLASH_WS_R {
        FLASH_WS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is \"0\" or when predictable execution behavior is required. '1': enabled."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1': Invalidates the content of the flash controller's buffers."]
    #[inline(always)]
    pub fn flash_invalidate(&self) -> FLASH_INVALIDATE_R {
        FLASH_INVALIDATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Amount of ROM wait states: \"0\": 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) \"1\": 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) \"2\": 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) \"3\": undefined"]
    #[inline(always)]
    pub fn flash_ws(&mut self) -> FLASH_WS_W {
        FLASH_WS_W { w: self }
    }
    #[doc = "Bit 4 - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is \"0\" or when predictable execution behavior is required. '1': enabled."]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W { w: self }
    }
    #[doc = "Bit 8 - 1': Invalidates the content of the flash controller's buffers."]
    #[inline(always)]
    pub fn flash_invalidate(&mut self) -> FLASH_INVALIDATE_W {
        FLASH_INVALIDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctl](index.html) module"]
pub struct FLASH_CTL_SPEC;
impl crate::RegisterSpec for FLASH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ctl::R](R) reader structure"]
impl crate::Readable for FLASH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ctl::W](W) writer structure"]
impl crate::Writable for FLASH_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_CTL to value 0"]
impl crate::Resettable for FLASH_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
