#[doc = "Register `PWR_ADFT_SELECT` reader"]
pub struct R(crate::R<PWR_ADFT_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_ADFT_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_ADFT_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_ADFT_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_ADFT_SELECT` writer"]
pub struct W(crate::W<PWR_ADFT_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_ADFT_SELECT_SPEC>;
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
impl From<crate::W<PWR_ADFT_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_ADFT_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRSS_SEL_A {
    #[doc = "0: `0`"]
    IREF0 = 0,
    #[doc = "1: `1`"]
    IREF1 = 1,
    #[doc = "2: `10`"]
    IREF2 = 2,
    #[doc = "3: `11`"]
    IREF3 = 3,
    #[doc = "4: `100`"]
    IREF4 = 4,
    #[doc = "5: `101`"]
    IREF5 = 5,
    #[doc = "6: `110`"]
    IREF6 = 6,
    #[doc = "7: `111`"]
    IREF7 = 7,
    #[doc = "8: `1000`"]
    IREF8 = 8,
    #[doc = "9: `1001`"]
    IREF9 = 9,
    #[doc = "10: `1010`"]
    IREF10 = 10,
    #[doc = "11: `1011`"]
    IREF11 = 11,
    #[doc = "12: `1100`"]
    IREF12 = 12,
    #[doc = "13: `1101`"]
    IREF13 = 13,
    #[doc = "14: `1110`"]
    IREF14 = 14,
    #[doc = "15: `1111`"]
    IREF15 = 15,
    #[doc = "16: `10000`"]
    IREF16 = 16,
    #[doc = "17: `10001`"]
    IREF17 = 17,
    #[doc = "18: `10010`"]
    IREF18 = 18,
    #[doc = "19: `10011`"]
    IREF19 = 19,
    #[doc = "20: `10100`"]
    IREF_MAIN = 20,
    #[doc = "21: `10101`"]
    IREF_LOCAL = 21,
    #[doc = "22: `10110`"]
    IREF_SPARE1 = 22,
    #[doc = "23: `10111`"]
    IREF_IMO_REG = 23,
    #[doc = "24: `11000`"]
    VCCD = 24,
    #[doc = "25: `11001`"]
    VBG_1P25 = 25,
    #[doc = "26: `11010`"]
    VREF_1P2 = 26,
    #[doc = "27: `11011`"]
    DSREF_0P74 = 27,
    #[doc = "28: `11100`"]
    VSSD = 28,
    #[doc = "29: `11101`"]
    VREG_1P55 = 29,
    #[doc = "30: `11110`"]
    VSSQ = 30,
    #[doc = "31: `11111`"]
    IREF_ALL = 31,
}
impl From<SRSS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SRSS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRSS_SEL` reader - "]
pub struct SRSS_SEL_R(crate::FieldReader<u8, SRSS_SEL_A>);
impl SRSS_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRSS_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRSS_SEL_A {
        match self.bits {
            0 => SRSS_SEL_A::IREF0,
            1 => SRSS_SEL_A::IREF1,
            2 => SRSS_SEL_A::IREF2,
            3 => SRSS_SEL_A::IREF3,
            4 => SRSS_SEL_A::IREF4,
            5 => SRSS_SEL_A::IREF5,
            6 => SRSS_SEL_A::IREF6,
            7 => SRSS_SEL_A::IREF7,
            8 => SRSS_SEL_A::IREF8,
            9 => SRSS_SEL_A::IREF9,
            10 => SRSS_SEL_A::IREF10,
            11 => SRSS_SEL_A::IREF11,
            12 => SRSS_SEL_A::IREF12,
            13 => SRSS_SEL_A::IREF13,
            14 => SRSS_SEL_A::IREF14,
            15 => SRSS_SEL_A::IREF15,
            16 => SRSS_SEL_A::IREF16,
            17 => SRSS_SEL_A::IREF17,
            18 => SRSS_SEL_A::IREF18,
            19 => SRSS_SEL_A::IREF19,
            20 => SRSS_SEL_A::IREF_MAIN,
            21 => SRSS_SEL_A::IREF_LOCAL,
            22 => SRSS_SEL_A::IREF_SPARE1,
            23 => SRSS_SEL_A::IREF_IMO_REG,
            24 => SRSS_SEL_A::VCCD,
            25 => SRSS_SEL_A::VBG_1P25,
            26 => SRSS_SEL_A::VREF_1P2,
            27 => SRSS_SEL_A::DSREF_0P74,
            28 => SRSS_SEL_A::VSSD,
            29 => SRSS_SEL_A::VREG_1P55,
            30 => SRSS_SEL_A::VSSQ,
            31 => SRSS_SEL_A::IREF_ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IREF0`"]
    #[inline(always)]
    pub fn is_iref0(&self) -> bool {
        **self == SRSS_SEL_A::IREF0
    }
    #[doc = "Checks if the value of the field is `IREF1`"]
    #[inline(always)]
    pub fn is_iref1(&self) -> bool {
        **self == SRSS_SEL_A::IREF1
    }
    #[doc = "Checks if the value of the field is `IREF2`"]
    #[inline(always)]
    pub fn is_iref2(&self) -> bool {
        **self == SRSS_SEL_A::IREF2
    }
    #[doc = "Checks if the value of the field is `IREF3`"]
    #[inline(always)]
    pub fn is_iref3(&self) -> bool {
        **self == SRSS_SEL_A::IREF3
    }
    #[doc = "Checks if the value of the field is `IREF4`"]
    #[inline(always)]
    pub fn is_iref4(&self) -> bool {
        **self == SRSS_SEL_A::IREF4
    }
    #[doc = "Checks if the value of the field is `IREF5`"]
    #[inline(always)]
    pub fn is_iref5(&self) -> bool {
        **self == SRSS_SEL_A::IREF5
    }
    #[doc = "Checks if the value of the field is `IREF6`"]
    #[inline(always)]
    pub fn is_iref6(&self) -> bool {
        **self == SRSS_SEL_A::IREF6
    }
    #[doc = "Checks if the value of the field is `IREF7`"]
    #[inline(always)]
    pub fn is_iref7(&self) -> bool {
        **self == SRSS_SEL_A::IREF7
    }
    #[doc = "Checks if the value of the field is `IREF8`"]
    #[inline(always)]
    pub fn is_iref8(&self) -> bool {
        **self == SRSS_SEL_A::IREF8
    }
    #[doc = "Checks if the value of the field is `IREF9`"]
    #[inline(always)]
    pub fn is_iref9(&self) -> bool {
        **self == SRSS_SEL_A::IREF9
    }
    #[doc = "Checks if the value of the field is `IREF10`"]
    #[inline(always)]
    pub fn is_iref10(&self) -> bool {
        **self == SRSS_SEL_A::IREF10
    }
    #[doc = "Checks if the value of the field is `IREF11`"]
    #[inline(always)]
    pub fn is_iref11(&self) -> bool {
        **self == SRSS_SEL_A::IREF11
    }
    #[doc = "Checks if the value of the field is `IREF12`"]
    #[inline(always)]
    pub fn is_iref12(&self) -> bool {
        **self == SRSS_SEL_A::IREF12
    }
    #[doc = "Checks if the value of the field is `IREF13`"]
    #[inline(always)]
    pub fn is_iref13(&self) -> bool {
        **self == SRSS_SEL_A::IREF13
    }
    #[doc = "Checks if the value of the field is `IREF14`"]
    #[inline(always)]
    pub fn is_iref14(&self) -> bool {
        **self == SRSS_SEL_A::IREF14
    }
    #[doc = "Checks if the value of the field is `IREF15`"]
    #[inline(always)]
    pub fn is_iref15(&self) -> bool {
        **self == SRSS_SEL_A::IREF15
    }
    #[doc = "Checks if the value of the field is `IREF16`"]
    #[inline(always)]
    pub fn is_iref16(&self) -> bool {
        **self == SRSS_SEL_A::IREF16
    }
    #[doc = "Checks if the value of the field is `IREF17`"]
    #[inline(always)]
    pub fn is_iref17(&self) -> bool {
        **self == SRSS_SEL_A::IREF17
    }
    #[doc = "Checks if the value of the field is `IREF18`"]
    #[inline(always)]
    pub fn is_iref18(&self) -> bool {
        **self == SRSS_SEL_A::IREF18
    }
    #[doc = "Checks if the value of the field is `IREF19`"]
    #[inline(always)]
    pub fn is_iref19(&self) -> bool {
        **self == SRSS_SEL_A::IREF19
    }
    #[doc = "Checks if the value of the field is `IREF_MAIN`"]
    #[inline(always)]
    pub fn is_iref_main(&self) -> bool {
        **self == SRSS_SEL_A::IREF_MAIN
    }
    #[doc = "Checks if the value of the field is `IREF_LOCAL`"]
    #[inline(always)]
    pub fn is_iref_local(&self) -> bool {
        **self == SRSS_SEL_A::IREF_LOCAL
    }
    #[doc = "Checks if the value of the field is `IREF_SPARE1`"]
    #[inline(always)]
    pub fn is_iref_spare1(&self) -> bool {
        **self == SRSS_SEL_A::IREF_SPARE1
    }
    #[doc = "Checks if the value of the field is `IREF_IMO_REG`"]
    #[inline(always)]
    pub fn is_iref_imo_reg(&self) -> bool {
        **self == SRSS_SEL_A::IREF_IMO_REG
    }
    #[doc = "Checks if the value of the field is `VCCD`"]
    #[inline(always)]
    pub fn is_vccd(&self) -> bool {
        **self == SRSS_SEL_A::VCCD
    }
    #[doc = "Checks if the value of the field is `VBG_1P25`"]
    #[inline(always)]
    pub fn is_vbg_1p25(&self) -> bool {
        **self == SRSS_SEL_A::VBG_1P25
    }
    #[doc = "Checks if the value of the field is `VREF_1P2`"]
    #[inline(always)]
    pub fn is_vref_1p2(&self) -> bool {
        **self == SRSS_SEL_A::VREF_1P2
    }
    #[doc = "Checks if the value of the field is `DSREF_0P74`"]
    #[inline(always)]
    pub fn is_dsref_0p74(&self) -> bool {
        **self == SRSS_SEL_A::DSREF_0P74
    }
    #[doc = "Checks if the value of the field is `VSSD`"]
    #[inline(always)]
    pub fn is_vssd(&self) -> bool {
        **self == SRSS_SEL_A::VSSD
    }
    #[doc = "Checks if the value of the field is `VREG_1P55`"]
    #[inline(always)]
    pub fn is_vreg_1p55(&self) -> bool {
        **self == SRSS_SEL_A::VREG_1P55
    }
    #[doc = "Checks if the value of the field is `VSSQ`"]
    #[inline(always)]
    pub fn is_vssq(&self) -> bool {
        **self == SRSS_SEL_A::VSSQ
    }
    #[doc = "Checks if the value of the field is `IREF_ALL`"]
    #[inline(always)]
    pub fn is_iref_all(&self) -> bool {
        **self == SRSS_SEL_A::IREF_ALL
    }
}
impl core::ops::Deref for SRSS_SEL_R {
    type Target = crate::FieldReader<u8, SRSS_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRSS_SEL` writer - "]
pub struct SRSS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRSS_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iref0(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iref1(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn iref2(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn iref3(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn iref4(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn iref5(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn iref6(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn iref7(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn iref8(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF8)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn iref9(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF9)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn iref10(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF10)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn iref11(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF11)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn iref12(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF12)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn iref13(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF13)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn iref14(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF14)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn iref15(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF15)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn iref16(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF16)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn iref17(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF17)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn iref18(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF18)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn iref19(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF19)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn iref_main(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF_MAIN)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn iref_local(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF_LOCAL)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn iref_spare1(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF_SPARE1)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn iref_imo_reg(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF_IMO_REG)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn vccd(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::VCCD)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn vbg_1p25(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::VBG_1P25)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn vref_1p2(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::VREF_1P2)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn dsref_0p74(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::DSREF_0P74)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn vssd(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::VSSD)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn vreg_1p55(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::VREG_1P55)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn vssq(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::VSSQ)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn iref_all(self) -> &'a mut W {
        self.variant(SRSS_SEL_A::IREF_ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `SRSS_AMUXA` reader - "]
pub struct SRSS_AMUXA_R(crate::FieldReader<bool, bool>);
impl SRSS_AMUXA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRSS_AMUXA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRSS_AMUXA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRSS_AMUXA` writer - "]
pub struct SRSS_AMUXA_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSS_AMUXA_W<'a> {
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
#[doc = "Field `SRSS_AMUXB` reader - "]
pub struct SRSS_AMUXB_R(crate::FieldReader<bool, bool>);
impl SRSS_AMUXB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRSS_AMUXB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRSS_AMUXB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRSS_AMUXB` writer - "]
pub struct SRSS_AMUXB_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSS_AMUXB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `BLEED_EN` reader - "]
pub struct BLEED_EN_R(crate::FieldReader<bool, bool>);
impl BLEED_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLEED_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEED_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEED_EN` writer - "]
pub struct BLEED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEED_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `POR_TRIGGER` reader - "]
pub struct POR_TRIGGER_R(crate::FieldReader<bool, bool>);
impl POR_TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_TRIGGER` writer - "]
pub struct POR_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_TRIGGER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SRSS_EN` reader - "]
pub struct SRSS_EN_R(crate::FieldReader<bool, bool>);
impl SRSS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRSS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRSS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRSS_EN` writer - "]
pub struct SRSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn srss_sel(&self) -> SRSS_SEL_R {
        SRSS_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn srss_amuxa(&self) -> SRSS_AMUXA_R {
        SRSS_AMUXA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn srss_amuxb(&self) -> SRSS_AMUXB_R {
        SRSS_AMUXB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn bleed_en(&self) -> BLEED_EN_R {
        BLEED_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn por_trigger(&self) -> POR_TRIGGER_R {
        POR_TRIGGER_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn srss_en(&self) -> SRSS_EN_R {
        SRSS_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn srss_sel(&mut self) -> SRSS_SEL_W {
        SRSS_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn srss_amuxa(&mut self) -> SRSS_AMUXA_W {
        SRSS_AMUXA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn srss_amuxb(&mut self) -> SRSS_AMUXB_W {
        SRSS_AMUXB_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn bleed_en(&mut self) -> BLEED_EN_W {
        BLEED_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn por_trigger(&mut self) -> POR_TRIGGER_W {
        POR_TRIGGER_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn srss_en(&mut self) -> SRSS_EN_W {
        SRSS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_adft_select](index.html) module"]
pub struct PWR_ADFT_SELECT_SPEC;
impl crate::RegisterSpec for PWR_ADFT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_adft_select::R](R) reader structure"]
impl crate::Readable for PWR_ADFT_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_adft_select::W](W) writer structure"]
impl crate::Writable for PWR_ADFT_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_ADFT_SELECT to value 0"]
impl crate::Resettable for PWR_ADFT_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
