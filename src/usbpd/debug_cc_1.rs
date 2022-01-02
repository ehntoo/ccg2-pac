#[doc = "Register `DEBUG_CC_1` reader"]
pub struct R(crate::R<DEBUG_CC_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CC_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CC_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CC_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_CC_1` writer"]
pub struct W(crate::W<DEBUG_CC_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CC_1_SPEC>;
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
impl From<crate::W<DEBUG_CC_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CC_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUM_PREAMBLE_IGNORE` reader - Number of preamble bits to ignore at the beginning of the RX packet."]
pub struct NUM_PREAMBLE_IGNORE_R(crate::FieldReader<u8, u8>);
impl NUM_PREAMBLE_IGNORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUM_PREAMBLE_IGNORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_PREAMBLE_IGNORE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_PREAMBLE_IGNORE` writer - Number of preamble bits to ignore at the beginning of the RX packet."]
pub struct NUM_PREAMBLE_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_PREAMBLE_IGNORE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Number of preamble bits to be used in the RX for averaging CDR frequency. Any time the value of these bits are changed, the values of NUM_TRANS_AVG will need to be updated.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUM_PREAMBLE_AVG_A {
    #[doc = "0: `0`"]
    FOUR_PREAM_AVG0 = 0,
    #[doc = "1: `1`"]
    FOUR_PREAM_AVG1 = 1,
    #[doc = "2: `10`"]
    FOUR_PREAM_AVG2 = 2,
    #[doc = "3: `11`"]
    EIGHT_PREAM_AVG = 3,
    #[doc = "4: `100`"]
    SIXTEEN_PREAM_AVG = 4,
    #[doc = "5: `101`"]
    THIRTY_TWO_PREAM_AVG = 5,
    #[doc = "6: `110`"]
    RESERVED6_PREAM_AVG = 6,
    #[doc = "7: `111`"]
    RESERVED7_PREAM_AVG = 7,
}
impl From<NUM_PREAMBLE_AVG_A> for u8 {
    #[inline(always)]
    fn from(variant: NUM_PREAMBLE_AVG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NUM_PREAMBLE_AVG` reader - Number of preamble bits to be used in the RX for averaging CDR frequency. Any time the value of these bits are changed, the values of NUM_TRANS_AVG will need to be updated."]
pub struct NUM_PREAMBLE_AVG_R(crate::FieldReader<u8, NUM_PREAMBLE_AVG_A>);
impl NUM_PREAMBLE_AVG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUM_PREAMBLE_AVG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUM_PREAMBLE_AVG_A {
        match self.bits {
            0 => NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG0,
            1 => NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG1,
            2 => NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG2,
            3 => NUM_PREAMBLE_AVG_A::EIGHT_PREAM_AVG,
            4 => NUM_PREAMBLE_AVG_A::SIXTEEN_PREAM_AVG,
            5 => NUM_PREAMBLE_AVG_A::THIRTY_TWO_PREAM_AVG,
            6 => NUM_PREAMBLE_AVG_A::RESERVED6_PREAM_AVG,
            7 => NUM_PREAMBLE_AVG_A::RESERVED7_PREAM_AVG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FOUR_PREAM_AVG0`"]
    #[inline(always)]
    pub fn is_four_pream_avg0(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG0
    }
    #[doc = "Checks if the value of the field is `FOUR_PREAM_AVG1`"]
    #[inline(always)]
    pub fn is_four_pream_avg1(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG1
    }
    #[doc = "Checks if the value of the field is `FOUR_PREAM_AVG2`"]
    #[inline(always)]
    pub fn is_four_pream_avg2(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG2
    }
    #[doc = "Checks if the value of the field is `EIGHT_PREAM_AVG`"]
    #[inline(always)]
    pub fn is_eight_pream_avg(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::EIGHT_PREAM_AVG
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_PREAM_AVG`"]
    #[inline(always)]
    pub fn is_sixteen_pream_avg(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::SIXTEEN_PREAM_AVG
    }
    #[doc = "Checks if the value of the field is `THIRTY_TWO_PREAM_AVG`"]
    #[inline(always)]
    pub fn is_thirty_two_pream_avg(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::THIRTY_TWO_PREAM_AVG
    }
    #[doc = "Checks if the value of the field is `RESERVED6_PREAM_AVG`"]
    #[inline(always)]
    pub fn is_reserved6_pream_avg(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::RESERVED6_PREAM_AVG
    }
    #[doc = "Checks if the value of the field is `RESERVED7_PREAM_AVG`"]
    #[inline(always)]
    pub fn is_reserved7_pream_avg(&self) -> bool {
        **self == NUM_PREAMBLE_AVG_A::RESERVED7_PREAM_AVG
    }
}
impl core::ops::Deref for NUM_PREAMBLE_AVG_R {
    type Target = crate::FieldReader<u8, NUM_PREAMBLE_AVG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_PREAMBLE_AVG` writer - Number of preamble bits to be used in the RX for averaging CDR frequency. Any time the value of these bits are changed, the values of NUM_TRANS_AVG will need to be updated."]
pub struct NUM_PREAMBLE_AVG_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_PREAMBLE_AVG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUM_PREAMBLE_AVG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn four_pream_avg0(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn four_pream_avg1(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn four_pream_avg2(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::FOUR_PREAM_AVG2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn eight_pream_avg(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::EIGHT_PREAM_AVG)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sixteen_pream_avg(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::SIXTEEN_PREAM_AVG)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn thirty_two_pream_avg(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::THIRTY_TWO_PREAM_AVG)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn reserved6_pream_avg(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::RESERVED6_PREAM_AVG)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn reserved7_pream_avg(self) -> &'a mut W {
        self.variant(NUM_PREAMBLE_AVG_A::RESERVED7_PREAM_AVG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `NUM_TRANS_AVG` reader - Number of transitions required to complete averaging in the receiver. This register will need to be updated any time values of NUM_PREAMBLE_AVG is changed. The values programmed into this register comes from the following table: NUM_PREAMBLE_AVG = 000 : Use 0x19 NUM_PREAMBLE_AVG = 001 : Use 0x19 NUM_PREAMBLE_AVG = 010 : Use 0x7 NUM_PREAMBLE_AVG = 011 : Use 0xd NUM_PREAMBLE_AVG = 100 : Use 0x19 NUM_PREAMBLE_AVG = 101 : Use 0x31"]
pub struct NUM_TRANS_AVG_R(crate::FieldReader<u8, u8>);
impl NUM_TRANS_AVG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUM_TRANS_AVG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_TRANS_AVG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_TRANS_AVG` writer - Number of transitions required to complete averaging in the receiver. This register will need to be updated any time values of NUM_PREAMBLE_AVG is changed. The values programmed into this register comes from the following table: NUM_PREAMBLE_AVG = 000 : Use 0x19 NUM_PREAMBLE_AVG = 001 : Use 0x19 NUM_PREAMBLE_AVG = 010 : Use 0x7 NUM_PREAMBLE_AVG = 011 : Use 0xd NUM_PREAMBLE_AVG = 100 : Use 0x19 NUM_PREAMBLE_AVG = 101 : Use 0x31"]
pub struct NUM_TRANS_AVG_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_TRANS_AVG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `RX_DISABLE_AUTO_ADJ` reader - 0: Automatic bit rate calculation by HW 1: Disables the RX-CC automatic bit rate detection and the RX_UI_PRERIOD register is used for RX UI period."]
pub struct RX_DISABLE_AUTO_ADJ_R(crate::FieldReader<bool, bool>);
impl RX_DISABLE_AUTO_ADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DISABLE_AUTO_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DISABLE_AUTO_ADJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DISABLE_AUTO_ADJ` writer - 0: Automatic bit rate calculation by HW 1: Disables the RX-CC automatic bit rate detection and the RX_UI_PRERIOD register is used for RX UI period."]
pub struct RX_DISABLE_AUTO_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DISABLE_AUTO_ADJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RX_UI_PERIOD` reader - When RX_DISABLE_AUTO_ADJ is set, this register value will define RX UI period."]
pub struct RX_UI_PERIOD_R(crate::FieldReader<u8, u8>);
impl RX_UI_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_UI_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_UI_PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_UI_PERIOD` writer - When RX_DISABLE_AUTO_ADJ is set, this register value will define RX UI period."]
pub struct RX_UI_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UI_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of preamble bits to ignore at the beginning of the RX packet."]
    #[inline(always)]
    pub fn num_preamble_ignore(&self) -> NUM_PREAMBLE_IGNORE_R {
        NUM_PREAMBLE_IGNORE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Number of preamble bits to be used in the RX for averaging CDR frequency. Any time the value of these bits are changed, the values of NUM_TRANS_AVG will need to be updated."]
    #[inline(always)]
    pub fn num_preamble_avg(&self) -> NUM_PREAMBLE_AVG_R {
        NUM_PREAMBLE_AVG_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:13 - Number of transitions required to complete averaging in the receiver. This register will need to be updated any time values of NUM_PREAMBLE_AVG is changed. The values programmed into this register comes from the following table: NUM_PREAMBLE_AVG = 000 : Use 0x19 NUM_PREAMBLE_AVG = 001 : Use 0x19 NUM_PREAMBLE_AVG = 010 : Use 0x7 NUM_PREAMBLE_AVG = 011 : Use 0xd NUM_PREAMBLE_AVG = 100 : Use 0x19 NUM_PREAMBLE_AVG = 101 : Use 0x31"]
    #[inline(always)]
    pub fn num_trans_avg(&self) -> NUM_TRANS_AVG_R {
        NUM_TRANS_AVG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 0: Automatic bit rate calculation by HW 1: Disables the RX-CC automatic bit rate detection and the RX_UI_PRERIOD register is used for RX UI period."]
    #[inline(always)]
    pub fn rx_disable_auto_adj(&self) -> RX_DISABLE_AUTO_ADJ_R {
        RX_DISABLE_AUTO_ADJ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - When RX_DISABLE_AUTO_ADJ is set, this register value will define RX UI period."]
    #[inline(always)]
    pub fn rx_ui_period(&self) -> RX_UI_PERIOD_R {
        RX_UI_PERIOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of preamble bits to ignore at the beginning of the RX packet."]
    #[inline(always)]
    pub fn num_preamble_ignore(&mut self) -> NUM_PREAMBLE_IGNORE_W {
        NUM_PREAMBLE_IGNORE_W { w: self }
    }
    #[doc = "Bits 2:4 - Number of preamble bits to be used in the RX for averaging CDR frequency. Any time the value of these bits are changed, the values of NUM_TRANS_AVG will need to be updated."]
    #[inline(always)]
    pub fn num_preamble_avg(&mut self) -> NUM_PREAMBLE_AVG_W {
        NUM_PREAMBLE_AVG_W { w: self }
    }
    #[doc = "Bits 8:13 - Number of transitions required to complete averaging in the receiver. This register will need to be updated any time values of NUM_PREAMBLE_AVG is changed. The values programmed into this register comes from the following table: NUM_PREAMBLE_AVG = 000 : Use 0x19 NUM_PREAMBLE_AVG = 001 : Use 0x19 NUM_PREAMBLE_AVG = 010 : Use 0x7 NUM_PREAMBLE_AVG = 011 : Use 0xd NUM_PREAMBLE_AVG = 100 : Use 0x19 NUM_PREAMBLE_AVG = 101 : Use 0x31"]
    #[inline(always)]
    pub fn num_trans_avg(&mut self) -> NUM_TRANS_AVG_W {
        NUM_TRANS_AVG_W { w: self }
    }
    #[doc = "Bit 14 - 0: Automatic bit rate calculation by HW 1: Disables the RX-CC automatic bit rate detection and the RX_UI_PRERIOD register is used for RX UI period."]
    #[inline(always)]
    pub fn rx_disable_auto_adj(&mut self) -> RX_DISABLE_AUTO_ADJ_W {
        RX_DISABLE_AUTO_ADJ_W { w: self }
    }
    #[doc = "Bits 16:23 - When RX_DISABLE_AUTO_ADJ is set, this register value will define RX UI period."]
    #[inline(always)]
    pub fn rx_ui_period(&mut self) -> RX_UI_PERIOD_W {
        RX_UI_PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "C-Connector Debug control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_cc_1](index.html) module"]
pub struct DEBUG_CC_1_SPEC;
impl crate::RegisterSpec for DEBUG_CC_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_cc_1::R](R) reader structure"]
impl crate::Readable for DEBUG_CC_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_cc_1::W](W) writer structure"]
impl crate::Writable for DEBUG_CC_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_CC_1 to value 0x0050_1911"]
impl crate::Resettable for DEBUG_CC_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0050_1911
    }
}
