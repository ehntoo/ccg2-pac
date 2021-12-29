#[doc = "Register `DFT` reader"]
pub struct R(crate::R<DFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFT` writer"]
pub struct W(crate::W<DFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFT_SPEC>;
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
impl From<crate::W<DFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PD` reader - "]
pub struct FLASH_PD_R(crate::FieldReader<bool, bool>);
impl FLASH_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PD` writer - "]
pub struct FLASH_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PD_W<'a> {
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
#[doc = "Field `ROM_PD` reader - "]
pub struct ROM_PD_R(crate::FieldReader<bool, bool>);
impl ROM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_PD` writer - "]
pub struct ROM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RAM_PD` reader - "]
pub struct RAM_PD_R(crate::FieldReader<bool, bool>);
impl RAM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_PD` writer - "]
pub struct RAM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DMAC_RAM_PD` reader - "]
pub struct DMAC_RAM_PD_R(crate::FieldReader<bool, bool>);
impl DMAC_RAM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC_RAM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC_RAM_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC_RAM_PD` writer - "]
pub struct DMAC_RAM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_RAM_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_pd(&self) -> FLASH_PD_R {
        FLASH_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rom_pd(&self) -> ROM_PD_R {
        ROM_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ram_pd(&self) -> RAM_PD_R {
        RAM_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmac_ram_pd(&self) -> DMAC_RAM_PD_R {
        DMAC_RAM_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_pd(&mut self) -> FLASH_PD_W {
        FLASH_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rom_pd(&mut self) -> ROM_PD_W {
        ROM_PD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ram_pd(&mut self) -> RAM_PD_W {
        RAM_PD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmac_ram_pd(&mut self) -> DMAC_RAM_PD_W {
        DMAC_RAM_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dft](index.html) module"]
pub struct DFT_SPEC;
impl crate::RegisterSpec for DFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dft::R](R) reader structure"]
impl crate::Readable for DFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dft::W](W) writer structure"]
impl crate::Writable for DFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFT to value 0"]
impl crate::Resettable for DFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
