#[doc = "Register `BIST_STEP0_CTL` reader"]
pub struct R(crate::R<BIST_STEP0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_STEP0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_STEP0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_STEP0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_STEP0_CTL` writer"]
pub struct W(crate::W<BIST_STEP0_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_STEP0_CTL_SPEC>;
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
impl From<crate::W<BIST_STEP0_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_STEP0_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPCODE` reader - "]
pub struct OPCODE_R(crate::FieldReader<u8, u8>);
impl OPCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPCODE` writer - "]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `UP` reader - "]
pub struct UP_R(crate::FieldReader<bool, bool>);
impl UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UP` writer - "]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
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
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_step0_ctl](index.html) module"]
pub struct BIST_STEP0_CTL_SPEC;
impl crate::RegisterSpec for BIST_STEP0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_step0_ctl::R](R) reader structure"]
impl crate::Readable for BIST_STEP0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_step0_ctl::W](W) writer structure"]
impl crate::Writable for BIST_STEP0_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_STEP0_CTL to value 0"]
impl crate::Resettable for BIST_STEP0_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
