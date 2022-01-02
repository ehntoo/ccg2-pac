#[doc = "Register `TR_CTRL2` reader"]
pub struct R(crate::R<TR_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTRL2` writer"]
pub struct W(crate::W<TR_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTRL2_SPEC>;
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
impl From<crate::W<TR_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Determines the effect of a compare match event (COUNTER equals CC register) on the \"line_out\" output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0%, the counter CC register should be set to \"0\". For a 100% duty cycle, the counter CC register should be set to larger than the counter PERIOD register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC_MATCH_MODE_A {
    #[doc = "0: `0`"]
    SET = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
    #[doc = "2: `10`"]
    INVERT = 2,
    #[doc = "3: `11`"]
    NO_CHANGE = 3,
}
impl From<CC_MATCH_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CC_MATCH_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CC_MATCH_MODE` reader - Determines the effect of a compare match event (COUNTER equals CC register) on the \"line_out\" output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0%, the counter CC register should be set to \"0\". For a 100% duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
pub struct CC_MATCH_MODE_R(crate::FieldReader<u8, CC_MATCH_MODE_A>);
impl CC_MATCH_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC_MATCH_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_MATCH_MODE_A {
        match self.bits {
            0 => CC_MATCH_MODE_A::SET,
            1 => CC_MATCH_MODE_A::CLEAR,
            2 => CC_MATCH_MODE_A::INVERT,
            3 => CC_MATCH_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == CC_MATCH_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CC_MATCH_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == CC_MATCH_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == CC_MATCH_MODE_A::NO_CHANGE
    }
}
impl core::ops::Deref for CC_MATCH_MODE_R {
    type Target = crate::FieldReader<u8, CC_MATCH_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_MATCH_MODE` writer - Determines the effect of a compare match event (COUNTER equals CC register) on the \"line_out\" output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0%, the counter CC register should be set to \"0\". For a 100% duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
pub struct CC_MATCH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_MATCH_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC_MATCH_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CC_MATCH_MODE_A::SET)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC_MATCH_MODE_A::CLEAR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(CC_MATCH_MODE_A::INVERT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CC_MATCH_MODE_A::NO_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the \"line_out\" output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVERFLOW_MODE_A {
    #[doc = "0: `0`"]
    SET = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
    #[doc = "2: `10`"]
    INVERT = 2,
    #[doc = "3: `11`"]
    NO_CHANGE = 3,
}
impl From<OVERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERFLOW_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVERFLOW_MODE` reader - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the \"line_out\" output signals."]
pub struct OVERFLOW_MODE_R(crate::FieldReader<u8, OVERFLOW_MODE_A>);
impl OVERFLOW_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVERFLOW_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_MODE_A {
        match self.bits {
            0 => OVERFLOW_MODE_A::SET,
            1 => OVERFLOW_MODE_A::CLEAR,
            2 => OVERFLOW_MODE_A::INVERT,
            3 => OVERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == OVERFLOW_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == OVERFLOW_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == OVERFLOW_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == OVERFLOW_MODE_A::NO_CHANGE
    }
}
impl core::ops::Deref for OVERFLOW_MODE_R {
    type Target = crate::FieldReader<u8, OVERFLOW_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW_MODE` writer - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the \"line_out\" output signals."]
pub struct OVERFLOW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::SET)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::CLEAR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::INVERT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::NO_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Determines the effect of a counter underflow event (COUNTER reaches \"0\") on the \"line_out\" output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UNDERFLOW_MODE_A {
    #[doc = "0: `0`"]
    SET = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
    #[doc = "2: `10`"]
    INVERT = 2,
    #[doc = "3: `11`"]
    NO_CHANGE = 3,
}
impl From<UNDERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UNDERFLOW_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UNDERFLOW_MODE` reader - Determines the effect of a counter underflow event (COUNTER reaches \"0\") on the \"line_out\" output signals."]
pub struct UNDERFLOW_MODE_R(crate::FieldReader<u8, UNDERFLOW_MODE_A>);
impl UNDERFLOW_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UNDERFLOW_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_MODE_A {
        match self.bits {
            0 => UNDERFLOW_MODE_A::SET,
            1 => UNDERFLOW_MODE_A::CLEAR,
            2 => UNDERFLOW_MODE_A::INVERT,
            3 => UNDERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == UNDERFLOW_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == UNDERFLOW_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == UNDERFLOW_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == UNDERFLOW_MODE_A::NO_CHANGE
    }
}
impl core::ops::Deref for UNDERFLOW_MODE_R {
    type Target = crate::FieldReader<u8, UNDERFLOW_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERFLOW_MODE` writer - Determines the effect of a counter underflow event (COUNTER reaches \"0\") on the \"line_out\" output signals."]
pub struct UNDERFLOW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDERFLOW_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::SET)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::CLEAR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::INVERT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::NO_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Determines the effect of a compare match event (COUNTER equals CC register) on the \"line_out\" output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0%, the counter CC register should be set to \"0\". For a 100% duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn cc_match_mode(&self) -> CC_MATCH_MODE_R {
        CC_MATCH_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the \"line_out\" output signals."]
    #[inline(always)]
    pub fn overflow_mode(&self) -> OVERFLOW_MODE_R {
        OVERFLOW_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches \"0\") on the \"line_out\" output signals."]
    #[inline(always)]
    pub fn underflow_mode(&self) -> UNDERFLOW_MODE_R {
        UNDERFLOW_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the effect of a compare match event (COUNTER equals CC register) on the \"line_out\" output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0%, the counter CC register should be set to \"0\". For a 100% duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn cc_match_mode(&mut self) -> CC_MATCH_MODE_W {
        CC_MATCH_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the \"line_out\" output signals."]
    #[inline(always)]
    pub fn overflow_mode(&mut self) -> OVERFLOW_MODE_W {
        OVERFLOW_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches \"0\") on the \"line_out\" output signals."]
    #[inline(always)]
    pub fn underflow_mode(&mut self) -> UNDERFLOW_MODE_W {
        UNDERFLOW_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter trigger control register 2 Used to control counter \"line_out\", \"dt_line_out\" and \"dt_line_compl_out\" output signals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctrl2](index.html) module"]
pub struct TR_CTRL2_SPEC;
impl crate::RegisterSpec for TR_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctrl2::R](R) reader structure"]
impl crate::Readable for TR_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctrl2::W](W) writer structure"]
impl crate::Writable for TR_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_CTRL2 to value 0x3f"]
impl crate::Resettable for TR_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
