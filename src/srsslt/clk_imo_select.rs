#[doc = "Register `CLK_IMO_SELECT` reader"]
pub struct R(crate::R<CLK_IMO_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_IMO_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_IMO_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_IMO_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_IMO_SELECT` writer"]
pub struct W(crate::W<CLK_IMO_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_IMO_SELECT_SPEC>;
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
impl From<crate::W<CLK_IMO_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_IMO_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select operating frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQ_A {
    #[doc = "0: `0`"]
    _24_MHZ = 0,
    #[doc = "1: `1`"]
    _28_MHZ = 1,
    #[doc = "2: `10`"]
    _32_MHZ = 2,
    #[doc = "3: `11`"]
    _36_MHZ = 3,
    #[doc = "4: `100`"]
    _40_MHZ = 4,
    #[doc = "5: `101`"]
    _44_MHZ = 5,
    #[doc = "6: `110`"]
    _48_MHZ = 6,
}
impl From<FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FREQ` reader - Select operating frequency"]
pub struct FREQ_R(crate::FieldReader<u8, FREQ_A>);
impl FREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQ_A> {
        match self.bits {
            0 => Some(FREQ_A::_24_MHZ),
            1 => Some(FREQ_A::_28_MHZ),
            2 => Some(FREQ_A::_32_MHZ),
            3 => Some(FREQ_A::_36_MHZ),
            4 => Some(FREQ_A::_40_MHZ),
            5 => Some(FREQ_A::_44_MHZ),
            6 => Some(FREQ_A::_48_MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_24_MHZ`"]
    #[inline(always)]
    pub fn is_24_mhz(&self) -> bool {
        **self == FREQ_A::_24_MHZ
    }
    #[doc = "Checks if the value of the field is `_28_MHZ`"]
    #[inline(always)]
    pub fn is_28_mhz(&self) -> bool {
        **self == FREQ_A::_28_MHZ
    }
    #[doc = "Checks if the value of the field is `_32_MHZ`"]
    #[inline(always)]
    pub fn is_32_mhz(&self) -> bool {
        **self == FREQ_A::_32_MHZ
    }
    #[doc = "Checks if the value of the field is `_36_MHZ`"]
    #[inline(always)]
    pub fn is_36_mhz(&self) -> bool {
        **self == FREQ_A::_36_MHZ
    }
    #[doc = "Checks if the value of the field is `_40_MHZ`"]
    #[inline(always)]
    pub fn is_40_mhz(&self) -> bool {
        **self == FREQ_A::_40_MHZ
    }
    #[doc = "Checks if the value of the field is `_44_MHZ`"]
    #[inline(always)]
    pub fn is_44_mhz(&self) -> bool {
        **self == FREQ_A::_44_MHZ
    }
    #[doc = "Checks if the value of the field is `_48_MHZ`"]
    #[inline(always)]
    pub fn is_48_mhz(&self) -> bool {
        **self == FREQ_A::_48_MHZ
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<u8, FREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ` writer - Select operating frequency"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _24_mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_24_MHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _28_mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_28_MHZ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32_mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_32_MHZ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _36_mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_36_MHZ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _40_mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_40_MHZ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _44_mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_44_MHZ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _48_mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_48_MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Select operating frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select operating frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMO Frequency Select Register Selects the operating frequency of the IMO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_imo_select](index.html) module"]
pub struct CLK_IMO_SELECT_SPEC;
impl crate::RegisterSpec for CLK_IMO_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_imo_select::R](R) reader structure"]
impl crate::Readable for CLK_IMO_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_imo_select::W](W) writer structure"]
impl crate::Writable for CLK_IMO_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_IMO_SELECT to value 0"]
impl crate::Resettable for CLK_IMO_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
