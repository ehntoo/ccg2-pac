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
#[doc = "\n\nValue on reset: 0"]
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
#[doc = "Field `EDGE0_SEL` reader - "]
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
#[doc = "Field `EDGE0_SEL` writer - "]
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
#[doc = "Field `EDGE1_SEL` reader - "]
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
#[doc = "Field `EDGE1_SEL` writer - "]
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
#[doc = "Field `EDGE2_SEL` reader - "]
pub struct EDGE2_SEL_R(crate::FieldReader<u8, u8>);
impl EDGE2_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE2_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE2_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE2_SEL` writer - "]
pub struct EDGE2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `EDGE3_SEL` reader - "]
pub struct EDGE3_SEL_R(crate::FieldReader<u8, u8>);
impl EDGE3_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE3_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE3_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE3_SEL` writer - "]
pub struct EDGE3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `EDGE4_SEL` reader - "]
pub struct EDGE4_SEL_R(crate::FieldReader<u8, u8>);
impl EDGE4_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE4_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE4_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE4_SEL` writer - "]
pub struct EDGE4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE4_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `EDGE5_SEL` reader - "]
pub struct EDGE5_SEL_R(crate::FieldReader<u8, u8>);
impl EDGE5_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE5_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE5_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE5_SEL` writer - "]
pub struct EDGE5_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE5_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `EDGE6_SEL` reader - "]
pub struct EDGE6_SEL_R(crate::FieldReader<u8, u8>);
impl EDGE6_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE6_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE6_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE6_SEL` writer - "]
pub struct EDGE6_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE6_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `EDGE7_SEL` reader - "]
pub struct EDGE7_SEL_R(crate::FieldReader<u8, u8>);
impl EDGE7_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGE7_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE7_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE7_SEL` writer - "]
pub struct EDGE7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE7_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
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
#[doc = "Field `FLT_EDGE_SEL` reader - "]
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
#[doc = "Field `FLT_EDGE_SEL` writer - "]
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
#[doc = "Field `FLT_SEL` reader - "]
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
#[doc = "Field `FLT_SEL` writer - "]
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
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn edge0_sel(&self) -> EDGE0_SEL_R {
        EDGE0_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn edge1_sel(&self) -> EDGE1_SEL_R {
        EDGE1_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn edge2_sel(&self) -> EDGE2_SEL_R {
        EDGE2_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn edge3_sel(&self) -> EDGE3_SEL_R {
        EDGE3_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn edge4_sel(&self) -> EDGE4_SEL_R {
        EDGE4_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn edge5_sel(&self) -> EDGE5_SEL_R {
        EDGE5_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn edge6_sel(&self) -> EDGE6_SEL_R {
        EDGE6_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn edge7_sel(&self) -> EDGE7_SEL_R {
        EDGE7_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn flt_edge_sel(&self) -> FLT_EDGE_SEL_R {
        FLT_EDGE_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn flt_sel(&self) -> FLT_SEL_R {
        FLT_SEL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn edge0_sel(&mut self) -> EDGE0_SEL_W {
        EDGE0_SEL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn edge1_sel(&mut self) -> EDGE1_SEL_W {
        EDGE1_SEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn edge2_sel(&mut self) -> EDGE2_SEL_W {
        EDGE2_SEL_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn edge3_sel(&mut self) -> EDGE3_SEL_W {
        EDGE3_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn edge4_sel(&mut self) -> EDGE4_SEL_W {
        EDGE4_SEL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn edge5_sel(&mut self) -> EDGE5_SEL_W {
        EDGE5_SEL_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn edge6_sel(&mut self) -> EDGE6_SEL_W {
        EDGE6_SEL_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn edge7_sel(&mut self) -> EDGE7_SEL_W {
        EDGE7_SEL_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn flt_edge_sel(&mut self) -> FLT_EDGE_SEL_W {
        FLT_EDGE_SEL_W { w: self }
    }
    #[doc = "Bits 18:20"]
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cfg](index.html) module"]
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
