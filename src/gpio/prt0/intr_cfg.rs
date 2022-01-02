#[doc = "Register `INTR_CFG` reader"]
pub struct R(crate::R<INTR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_CFG` writer"]
pub struct W(crate::W<INTR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_CFG_SPEC>;
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
impl From<crate::W<INTR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sets which edge will trigger an IRQ for IO pad 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGE0_SEL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    RISING = 1,
    #[doc = "2: `10`"]
    FALLING = 2,
    #[doc = "3: `11`"]
    BOTH = 3,
}
impl From<EDGE0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EDGE0_SEL` reader - Sets which edge will trigger an IRQ for IO pad 0."]
pub struct EDGE0_SEL_R(crate::FieldReader<u8, EDGE0_SEL_A>);
impl EDGE0_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE0_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE0_SEL_A {
        match self.bits {
            0 => EDGE0_SEL_A::DISABLE,
            1 => EDGE0_SEL_A::RISING,
            2 => EDGE0_SEL_A::FALLING,
            3 => EDGE0_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == EDGE0_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == EDGE0_SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == EDGE0_SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == EDGE0_SEL_A::BOTH
    }
}
impl core::ops::Deref for EDGE0_SEL_R {
    type Target = crate::FieldReader<u8, EDGE0_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE0_SEL` writer - Sets which edge will trigger an IRQ for IO pad 0."]
pub struct EDGE0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE0_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE0_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::RISING)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::FALLING)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `EDGE1_SEL` reader - Sets which edge will trigger an IRQ for IO pad 1."]
pub struct EDGE1_SEL_R(crate::FieldReader<u8, u8>);
impl EDGE1_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE1_SEL` writer - Sets which edge will trigger an IRQ for IO pad 1."]
pub struct EDGE1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Same for the glitch filtered pin (selected by FLT_SELECT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLT_EDGE_SEL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    RISING = 1,
    #[doc = "2: `10`"]
    FALLING = 2,
    #[doc = "3: `11`"]
    BOTH = 3,
}
impl From<FLT_EDGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT_EDGE_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLT_EDGE_SEL` reader - Same for the glitch filtered pin (selected by FLT_SELECT)."]
pub struct FLT_EDGE_SEL_R(crate::FieldReader<u8, FLT_EDGE_SEL_A>);
impl FLT_EDGE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLT_EDGE_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT_EDGE_SEL_A {
        match self.bits {
            0 => FLT_EDGE_SEL_A::DISABLE,
            1 => FLT_EDGE_SEL_A::RISING,
            2 => FLT_EDGE_SEL_A::FALLING,
            3 => FLT_EDGE_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FLT_EDGE_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == FLT_EDGE_SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == FLT_EDGE_SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == FLT_EDGE_SEL_A::BOTH
    }
}
impl core::ops::Deref for FLT_EDGE_SEL_R {
    type Target = crate::FieldReader<u8, FLT_EDGE_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_EDGE_SEL` writer - Same for the glitch filtered pin (selected by FLT_SELECT)."]
pub struct FLT_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_EDGE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_EDGE_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::RISING)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::FALLING)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FLT_SEL` reader - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub struct FLT_SEL_R(crate::FieldReader<u8, u8>);
impl FLT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_SEL` writer - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub struct FLT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pad 0."]
    #[inline(always)]
    pub fn edge0_sel(&self) -> EDGE0_SEL_R {
        EDGE0_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pad 1."]
    #[inline(always)]
    pub fn edge1_sel(&self) -> EDGE1_SEL_R {
        EDGE1_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Same for the glitch filtered pin (selected by FLT_SELECT)."]
    #[inline(always)]
    pub fn flt_edge_sel(&self) -> FLT_EDGE_SEL_R {
        FLT_EDGE_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&self) -> FLT_SEL_R {
        FLT_SEL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pad 0."]
    #[inline(always)]
    pub fn edge0_sel(&mut self) -> EDGE0_SEL_W {
        EDGE0_SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pad 1."]
    #[inline(always)]
    pub fn edge1_sel(&mut self) -> EDGE1_SEL_W {
        EDGE1_SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Same for the glitch filtered pin (selected by FLT_SELECT)."]
    #[inline(always)]
    pub fn flt_edge_sel(&mut self) -> FLT_EDGE_SEL_W {
        FLT_EDGE_SEL_W { w: self }
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&mut self) -> FLT_SEL_W {
        FLT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port interrupt configuration register This register configures the IRQ configuration for all pins in a port, with the IRQ type being individually pin-configurable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cfg](index.html) module"]
pub struct INTR_CFG_SPEC;
impl crate::RegisterSpec for INTR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cfg::R](R) reader structure"]
impl crate::Readable for INTR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_cfg::W](W) writer structure"]
impl crate::Writable for INTR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_CFG to value 0"]
impl crate::Resettable for INTR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
