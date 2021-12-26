#[doc = "Register `RX_EXPECT_GOODCRC_MSG` reader"]
pub struct R(crate::R<RX_EXPECT_GOODCRC_MSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_EXPECT_GOODCRC_MSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_EXPECT_GOODCRC_MSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_EXPECT_GOODCRC_MSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_EXPECT_GOODCRC_MSG` writer"]
pub struct W(crate::W<RX_EXPECT_GOODCRC_MSG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_EXPECT_GOODCRC_MSG_SPEC>;
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
impl From<crate::W<RX_EXPECT_GOODCRC_MSG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_EXPECT_GOODCRC_MSG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXPECTED_HEADER` reader - "]
pub struct EXPECTED_HEADER_R(crate::FieldReader<u16, u16>);
impl EXPECTED_HEADER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EXPECTED_HEADER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXPECTED_HEADER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXPECTED_HEADER` writer - "]
pub struct EXPECTED_HEADER_W<'a> {
    w: &'a mut W,
}
impl<'a> EXPECTED_HEADER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXPECTED_SOP_A {
    #[doc = "0: `0`"]
    EXP_RESERVED = 0,
    #[doc = "1: `1`"]
    EXP_DEFAULT_SOP_RCVD = 1,
    #[doc = "2: `10`"]
    EXP_PRIME_SOP_RCVD = 2,
    #[doc = "3: `11`"]
    EXP_DBL_PRIME_SOP_RCVD = 3,
    #[doc = "4: `100`"]
    EXP_DBG_PRIME_SOP_RCVD = 4,
    #[doc = "5: `101`"]
    EXP_DBG_DBL_PRIME_SOP_RCVD = 5,
    #[doc = "6: `110`"]
    EXP_RSVD1_SOP_RCVD = 6,
    #[doc = "7: `111`"]
    EXP_RSVD2_SOP_RCVD = 7,
}
impl From<EXPECTED_SOP_A> for u8 {
    #[inline(always)]
    fn from(variant: EXPECTED_SOP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXPECTED_SOP` reader - "]
pub struct EXPECTED_SOP_R(crate::FieldReader<u8, EXPECTED_SOP_A>);
impl EXPECTED_SOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXPECTED_SOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXPECTED_SOP_A {
        match self.bits {
            0 => EXPECTED_SOP_A::EXP_RESERVED,
            1 => EXPECTED_SOP_A::EXP_DEFAULT_SOP_RCVD,
            2 => EXPECTED_SOP_A::EXP_PRIME_SOP_RCVD,
            3 => EXPECTED_SOP_A::EXP_DBL_PRIME_SOP_RCVD,
            4 => EXPECTED_SOP_A::EXP_DBG_PRIME_SOP_RCVD,
            5 => EXPECTED_SOP_A::EXP_DBG_DBL_PRIME_SOP_RCVD,
            6 => EXPECTED_SOP_A::EXP_RSVD1_SOP_RCVD,
            7 => EXPECTED_SOP_A::EXP_RSVD2_SOP_RCVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXP_RESERVED`"]
    #[inline(always)]
    pub fn is_exp_reserved(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_RESERVED
    }
    #[doc = "Checks if the value of the field is `EXP_DEFAULT_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_exp_default_sop_rcvd(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_DEFAULT_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `EXP_PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_exp_prime_sop_rcvd(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `EXP_DBL_PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_exp_dbl_prime_sop_rcvd(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_DBL_PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `EXP_DBG_PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_exp_dbg_prime_sop_rcvd(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_DBG_PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `EXP_DBG_DBL_PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_exp_dbg_dbl_prime_sop_rcvd(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_DBG_DBL_PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `EXP_RSVD1_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_exp_rsvd1_sop_rcvd(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_RSVD1_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `EXP_RSVD2_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_exp_rsvd2_sop_rcvd(&self) -> bool {
        **self == EXPECTED_SOP_A::EXP_RSVD2_SOP_RCVD
    }
}
impl core::ops::Deref for EXPECTED_SOP_R {
    type Target = crate::FieldReader<u8, EXPECTED_SOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXPECTED_SOP` writer - "]
pub struct EXPECTED_SOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EXPECTED_SOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXPECTED_SOP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn exp_reserved(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_RESERVED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn exp_default_sop_rcvd(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_DEFAULT_SOP_RCVD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn exp_prime_sop_rcvd(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_PRIME_SOP_RCVD)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn exp_dbl_prime_sop_rcvd(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_DBL_PRIME_SOP_RCVD)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn exp_dbg_prime_sop_rcvd(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_DBG_PRIME_SOP_RCVD)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn exp_dbg_dbl_prime_sop_rcvd(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_DBG_DBL_PRIME_SOP_RCVD)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn exp_rsvd1_sop_rcvd(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_RSVD1_SOP_RCVD)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn exp_rsvd2_sop_rcvd(self) -> &'a mut W {
        self.variant(EXPECTED_SOP_A::EXP_RSVD2_SOP_RCVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `DISABLE_RX_CRC_TIMER` reader - "]
pub struct DISABLE_RX_CRC_TIMER_R(crate::FieldReader<bool, bool>);
impl DISABLE_RX_CRC_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_RX_CRC_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_RX_CRC_TIMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_RX_CRC_TIMER` writer - "]
pub struct DISABLE_RX_CRC_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_RX_CRC_TIMER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn expected_header(&self) -> EXPECTED_HEADER_R {
        EXPECTED_HEADER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn expected_sop(&self) -> EXPECTED_SOP_R {
        EXPECTED_SOP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn disable_rx_crc_timer(&self) -> DISABLE_RX_CRC_TIMER_R {
        DISABLE_RX_CRC_TIMER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn expected_header(&mut self) -> EXPECTED_HEADER_W {
        EXPECTED_HEADER_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn expected_sop(&mut self) -> EXPECTED_SOP_W {
        EXPECTED_SOP_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn disable_rx_crc_timer(&mut self) -> DISABLE_RX_CRC_TIMER_W {
        DISABLE_RX_CRC_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_expect_goodcrc_msg](index.html) module"]
pub struct RX_EXPECT_GOODCRC_MSG_SPEC;
impl crate::RegisterSpec for RX_EXPECT_GOODCRC_MSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_expect_goodcrc_msg::R](R) reader structure"]
impl crate::Readable for RX_EXPECT_GOODCRC_MSG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_expect_goodcrc_msg::W](W) writer structure"]
impl crate::Writable for RX_EXPECT_GOODCRC_MSG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_EXPECT_GOODCRC_MSG to value 0x01"]
impl crate::Resettable for RX_EXPECT_GOODCRC_MSG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
