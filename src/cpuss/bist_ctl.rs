#[doc = "Register `BIST_CTL` reader"]
pub struct R(crate::R<BIST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_CTL` writer"]
pub struct W(crate::W<BIST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_CTL_SPEC>;
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
impl From<crate::W<BIST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Hot-one mask for the SRAMs for which the BIST is performed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMS_ENABLED_A {
    #[doc = "1: `1`"]
    SRAM_CPUSS_MASK = 1,
    #[doc = "2: `10`"]
    SRAM_DMAC_MASK = 2,
    #[doc = "4: `100`"]
    EXT_SRAM_0_MASK = 4,
    #[doc = "8: `1000`"]
    EXT_SRAM_1_MASK = 8,
    #[doc = "16: `10000`"]
    EXT_SRAM_2_MASK = 16,
    #[doc = "32: `100000`"]
    EXT_SRAM_3_MASK = 32,
}
impl From<SRAMS_ENABLED_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMS_ENABLED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAMS_ENABLED` reader - Hot-one mask for the SRAMs for which the BIST is performed."]
pub struct SRAMS_ENABLED_R(crate::FieldReader<u8, SRAMS_ENABLED_A>);
impl SRAMS_ENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAMS_ENABLED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAMS_ENABLED_A> {
        match self.bits {
            1 => Some(SRAMS_ENABLED_A::SRAM_CPUSS_MASK),
            2 => Some(SRAMS_ENABLED_A::SRAM_DMAC_MASK),
            4 => Some(SRAMS_ENABLED_A::EXT_SRAM_0_MASK),
            8 => Some(SRAMS_ENABLED_A::EXT_SRAM_1_MASK),
            16 => Some(SRAMS_ENABLED_A::EXT_SRAM_2_MASK),
            32 => Some(SRAMS_ENABLED_A::EXT_SRAM_3_MASK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_CPUSS_MASK`"]
    #[inline(always)]
    pub fn is_sram_cpuss_mask(&self) -> bool {
        **self == SRAMS_ENABLED_A::SRAM_CPUSS_MASK
    }
    #[doc = "Checks if the value of the field is `SRAM_DMAC_MASK`"]
    #[inline(always)]
    pub fn is_sram_dmac_mask(&self) -> bool {
        **self == SRAMS_ENABLED_A::SRAM_DMAC_MASK
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_0_MASK`"]
    #[inline(always)]
    pub fn is_ext_sram_0_mask(&self) -> bool {
        **self == SRAMS_ENABLED_A::EXT_SRAM_0_MASK
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_1_MASK`"]
    #[inline(always)]
    pub fn is_ext_sram_1_mask(&self) -> bool {
        **self == SRAMS_ENABLED_A::EXT_SRAM_1_MASK
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_2_MASK`"]
    #[inline(always)]
    pub fn is_ext_sram_2_mask(&self) -> bool {
        **self == SRAMS_ENABLED_A::EXT_SRAM_2_MASK
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_3_MASK`"]
    #[inline(always)]
    pub fn is_ext_sram_3_mask(&self) -> bool {
        **self == SRAMS_ENABLED_A::EXT_SRAM_3_MASK
    }
}
impl core::ops::Deref for SRAMS_ENABLED_R {
    type Target = crate::FieldReader<u8, SRAMS_ENABLED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMS_ENABLED` writer - Hot-one mask for the SRAMs for which the BIST is performed."]
pub struct SRAMS_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMS_ENABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMS_ENABLED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sram_cpuss_mask(self) -> &'a mut W {
        self.variant(SRAMS_ENABLED_A::SRAM_CPUSS_MASK)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sram_dmac_mask(self) -> &'a mut W {
        self.variant(SRAMS_ENABLED_A::SRAM_DMAC_MASK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ext_sram_0_mask(self) -> &'a mut W {
        self.variant(SRAMS_ENABLED_A::EXT_SRAM_0_MASK)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn ext_sram_1_mask(self) -> &'a mut W {
        self.variant(SRAMS_ENABLED_A::EXT_SRAM_1_MASK)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn ext_sram_2_mask(self) -> &'a mut W {
        self.variant(SRAMS_ENABLED_A::EXT_SRAM_2_MASK)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn ext_sram_3_mask(self) -> &'a mut W {
        self.variant(SRAMS_ENABLED_A::EXT_SRAM_3_MASK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `ROW_FIRST` reader - Specifies how the SRAM BIST addresses are generated (should be set to '0' for SROM BIST): '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the column address incremented/decremented."]
pub struct ROW_FIRST_R(crate::FieldReader<bool, bool>);
impl ROW_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROW_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROW_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROW_FIRST` writer - Specifies how the SRAM BIST addresses are generated (should be set to '0' for SROM BIST): '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the column address incremented/decremented."]
pub struct ROW_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ROW_FIRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Hot-one mask for the SRAMs for which the BIST is performed."]
    #[inline(always)]
    pub fn srams_enabled(&self) -> SRAMS_ENABLED_R {
        SRAMS_ENABLED_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 20 - Specifies how the SRAM BIST addresses are generated (should be set to '0' for SROM BIST): '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&self) -> ROW_FIRST_R {
        ROW_FIRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Hot-one mask for the SRAMs for which the BIST is performed."]
    #[inline(always)]
    pub fn srams_enabled(&mut self) -> SRAMS_ENABLED_W {
        SRAMS_ENABLED_W { w: self }
    }
    #[doc = "Bit 20 - Specifies how the SRAM BIST addresses are generated (should be set to '0' for SROM BIST): '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&mut self) -> ROW_FIRST_W {
        ROW_FIRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_ctl](index.html) module"]
pub struct BIST_CTL_SPEC;
impl crate::RegisterSpec for BIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_ctl::R](R) reader structure"]
impl crate::Readable for BIST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_ctl::W](W) writer structure"]
impl crate::Writable for BIST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_CTL to value 0"]
impl crate::Resettable for BIST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
