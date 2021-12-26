#[doc = "Register `RX_CC` reader"]
pub struct R(crate::R<RX_CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CC` writer"]
pub struct W(crate::W<RX_CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CC_SPEC>;
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
impl From<crate::W<RX_CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_CNT_MAX` reader - "]
pub struct RX_CNT_MAX_R(crate::FieldReader<u8, u8>);
impl RX_CNT_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_CNT_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CNT_MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CNT_MAX` writer - "]
pub struct RX_CNT_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CNT_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RX_UI_BOUNDARY_DELTA` reader - "]
pub struct RX_UI_BOUNDARY_DELTA_R(crate::FieldReader<u8, u8>);
impl RX_UI_BOUNDARY_DELTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_UI_BOUNDARY_DELTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_UI_BOUNDARY_DELTA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_UI_BOUNDARY_DELTA` writer - "]
pub struct RX_UI_BOUNDARY_DELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UI_BOUNDARY_DELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `RX_UI_TRANS_TO_ZERO` reader - "]
pub struct RX_UI_TRANS_TO_ZERO_R(crate::FieldReader<u8, u8>);
impl RX_UI_TRANS_TO_ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_UI_TRANS_TO_ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_UI_TRANS_TO_ZERO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_UI_TRANS_TO_ZERO` writer - "]
pub struct RX_UI_TRANS_TO_ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UI_TRANS_TO_ZERO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | ((value as u32 & 0x3f) << 14);
        self.w
    }
}
#[doc = "Field `RX_VALID_NOISE_QUAL_ENABLE` reader - "]
pub struct RX_VALID_NOISE_QUAL_ENABLE_R(crate::FieldReader<bool, bool>);
impl RX_VALID_NOISE_QUAL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_VALID_NOISE_QUAL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_VALID_NOISE_QUAL_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_VALID_NOISE_QUAL_ENABLE` writer - "]
pub struct RX_VALID_NOISE_QUAL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_VALID_NOISE_QUAL_ENABLE_W<'a> {
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
#[doc = "Field `VCMP_UP_COMP_ENABLE` reader - "]
pub struct VCMP_UP_COMP_ENABLE_R(crate::FieldReader<bool, bool>);
impl VCMP_UP_COMP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_UP_COMP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_UP_COMP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_UP_COMP_ENABLE` writer - "]
pub struct VCMP_UP_COMP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_UP_COMP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `VCMP_LA_COMP_ENABLE` reader - "]
pub struct VCMP_LA_COMP_ENABLE_R(crate::FieldReader<bool, bool>);
impl VCMP_LA_COMP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_LA_COMP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_LA_COMP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_LA_COMP_ENABLE` writer - "]
pub struct VCMP_LA_COMP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_LA_COMP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RX_NOISE_CHECK_ENABLE` reader - "]
pub struct RX_NOISE_CHECK_ENABLE_R(crate::FieldReader<bool, bool>);
impl RX_NOISE_CHECK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_NOISE_CHECK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_NOISE_CHECK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_NOISE_CHECK_ENABLE` writer - "]
pub struct RX_NOISE_CHECK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NOISE_CHECK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NOISE_EDGE_COUNT_A {
    #[doc = "0: `0`"]
    THREE_TRANS_TRIG = 0,
    #[doc = "1: `1`"]
    FOUR_TRANS_TRIG = 1,
    #[doc = "2: `10`"]
    FIVE_TRANS_TRIG = 2,
    #[doc = "3: `11`"]
    SIX_TRANS_TRIG = 3,
}
impl From<NOISE_EDGE_COUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: NOISE_EDGE_COUNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NOISE_EDGE_COUNT` reader - "]
pub struct NOISE_EDGE_COUNT_R(crate::FieldReader<u8, NOISE_EDGE_COUNT_A>);
impl NOISE_EDGE_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NOISE_EDGE_COUNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOISE_EDGE_COUNT_A {
        match self.bits {
            0 => NOISE_EDGE_COUNT_A::THREE_TRANS_TRIG,
            1 => NOISE_EDGE_COUNT_A::FOUR_TRANS_TRIG,
            2 => NOISE_EDGE_COUNT_A::FIVE_TRANS_TRIG,
            3 => NOISE_EDGE_COUNT_A::SIX_TRANS_TRIG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THREE_TRANS_TRIG`"]
    #[inline(always)]
    pub fn is_three_trans_trig(&self) -> bool {
        **self == NOISE_EDGE_COUNT_A::THREE_TRANS_TRIG
    }
    #[doc = "Checks if the value of the field is `FOUR_TRANS_TRIG`"]
    #[inline(always)]
    pub fn is_four_trans_trig(&self) -> bool {
        **self == NOISE_EDGE_COUNT_A::FOUR_TRANS_TRIG
    }
    #[doc = "Checks if the value of the field is `FIVE_TRANS_TRIG`"]
    #[inline(always)]
    pub fn is_five_trans_trig(&self) -> bool {
        **self == NOISE_EDGE_COUNT_A::FIVE_TRANS_TRIG
    }
    #[doc = "Checks if the value of the field is `SIX_TRANS_TRIG`"]
    #[inline(always)]
    pub fn is_six_trans_trig(&self) -> bool {
        **self == NOISE_EDGE_COUNT_A::SIX_TRANS_TRIG
    }
}
impl core::ops::Deref for NOISE_EDGE_COUNT_R {
    type Target = crate::FieldReader<u8, NOISE_EDGE_COUNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOISE_EDGE_COUNT` writer - "]
pub struct NOISE_EDGE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOISE_EDGE_COUNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOISE_EDGE_COUNT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn three_trans_trig(self) -> &'a mut W {
        self.variant(NOISE_EDGE_COUNT_A::THREE_TRANS_TRIG)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn four_trans_trig(self) -> &'a mut W {
        self.variant(NOISE_EDGE_COUNT_A::FOUR_TRANS_TRIG)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn five_trans_trig(self) -> &'a mut W {
        self.variant(NOISE_EDGE_COUNT_A::FIVE_TRANS_TRIG)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn six_trans_trig(self) -> &'a mut W {
        self.variant(NOISE_EDGE_COUNT_A::SIX_TRANS_TRIG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rx_cnt_max(&self) -> RX_CNT_MAX_R {
        RX_CNT_MAX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rx_ui_boundary_delta(&self) -> RX_UI_BOUNDARY_DELTA_R {
        RX_UI_BOUNDARY_DELTA_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn rx_ui_trans_to_zero(&self) -> RX_UI_TRANS_TO_ZERO_R {
        RX_UI_TRANS_TO_ZERO_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_valid_noise_qual_enable(&self) -> RX_VALID_NOISE_QUAL_ENABLE_R {
        RX_VALID_NOISE_QUAL_ENABLE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn vcmp_up_comp_enable(&self) -> VCMP_UP_COMP_ENABLE_R {
        VCMP_UP_COMP_ENABLE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn vcmp_la_comp_enable(&self) -> VCMP_LA_COMP_ENABLE_R {
        VCMP_LA_COMP_ENABLE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_noise_check_enable(&self) -> RX_NOISE_CHECK_ENABLE_R {
        RX_NOISE_CHECK_ENABLE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn noise_edge_count(&self) -> NOISE_EDGE_COUNT_R {
        NOISE_EDGE_COUNT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rx_cnt_max(&mut self) -> RX_CNT_MAX_W {
        RX_CNT_MAX_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rx_ui_boundary_delta(&mut self) -> RX_UI_BOUNDARY_DELTA_W {
        RX_UI_BOUNDARY_DELTA_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn rx_ui_trans_to_zero(&mut self) -> RX_UI_TRANS_TO_ZERO_W {
        RX_UI_TRANS_TO_ZERO_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_valid_noise_qual_enable(&mut self) -> RX_VALID_NOISE_QUAL_ENABLE_W {
        RX_VALID_NOISE_QUAL_ENABLE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn vcmp_up_comp_enable(&mut self) -> VCMP_UP_COMP_ENABLE_W {
        VCMP_UP_COMP_ENABLE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn vcmp_la_comp_enable(&mut self) -> VCMP_LA_COMP_ENABLE_W {
        VCMP_LA_COMP_ENABLE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_noise_check_enable(&mut self) -> RX_NOISE_CHECK_ENABLE_W {
        RX_NOISE_CHECK_ENABLE_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn noise_edge_count(&mut self) -> NOISE_EDGE_COUNT_W {
        NOISE_EDGE_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_cc](index.html) module"]
pub struct RX_CC_SPEC;
impl crate::RegisterSpec for RX_CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_cc::R](R) reader structure"]
impl crate::Readable for RX_CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_cc::W](W) writer structure"]
impl crate::Writable for RX_CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CC to value 0x0580"]
impl crate::Resettable for RX_CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0580
    }
}
