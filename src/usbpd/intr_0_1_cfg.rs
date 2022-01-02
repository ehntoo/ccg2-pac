#[doc = "Register `INTR_0_1_CFG` reader"]
pub struct R(crate::R<INTR_0_1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_0_1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_0_1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_0_1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_0_1_CFG` writer"]
pub struct W(crate::W<INTR_0_1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_0_1_CFG_SPEC>;
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
impl From<crate::W<INTR_0_1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_0_1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR0_TX_PACKET_DONE_CFG` reader - Source of INTR0.TX_PACKET_DONE 0: TX EOP 1: CC Output Enable de-assert"]
pub struct INTR0_TX_PACKET_DONE_CFG_R(crate::FieldReader<bool, bool>);
impl INTR0_TX_PACKET_DONE_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR0_TX_PACKET_DONE_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR0_TX_PACKET_DONE_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTR0_TX_PACKET_DONE_CFG` writer - Source of INTR0.TX_PACKET_DONE 0: TX EOP 1: CC Output Enable de-assert"]
pub struct INTR0_TX_PACKET_DONE_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR0_TX_PACKET_DONE_CFG_W<'a> {
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
#[doc = "Field `INTR0_TX_CC_DATA_OEN_CFG` reader - Source of INTR0.TX_CC_DATA_OEN 0: CC Output Enable de-assert 1: TX EOP"]
pub struct INTR0_TX_CC_DATA_OEN_CFG_R(crate::FieldReader<bool, bool>);
impl INTR0_TX_CC_DATA_OEN_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTR0_TX_CC_DATA_OEN_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR0_TX_CC_DATA_OEN_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTR0_TX_CC_DATA_OEN_CFG` writer - Source of INTR0.TX_CC_DATA_OEN 0: CC Output Enable de-assert 1: TX EOP"]
pub struct INTR0_TX_CC_DATA_OEN_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR0_TX_CC_DATA_OEN_CFG_W<'a> {
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
#[doc = "Edge detect2 positive/negative enable/disable\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VCONN1_CFG_A {
    #[doc = "0: `0`"]
    POS_EDG_DIS_NEG_EDG_DIS = 0,
    #[doc = "1: `1`"]
    POS_EDG_DIS_NEG_EDG_EN = 1,
    #[doc = "2: `10`"]
    POS_EDG_EN_NEG_EDG_DIS = 2,
    #[doc = "3: `11`"]
    POS_EDG_EN_NEG_EDG_EN = 3,
}
impl From<VCONN1_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: VCONN1_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VCONN1_CFG` reader - Edge detect2 positive/negative enable/disable"]
pub struct VCONN1_CFG_R(crate::FieldReader<u8, VCONN1_CFG_A>);
impl VCONN1_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCONN1_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCONN1_CFG_A {
        match self.bits {
            0 => VCONN1_CFG_A::POS_EDG_DIS_NEG_EDG_DIS,
            1 => VCONN1_CFG_A::POS_EDG_DIS_NEG_EDG_EN,
            2 => VCONN1_CFG_A::POS_EDG_EN_NEG_EDG_DIS,
            3 => VCONN1_CFG_A::POS_EDG_EN_NEG_EDG_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_dis(&self) -> bool {
        **self == VCONN1_CFG_A::POS_EDG_DIS_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_en(&self) -> bool {
        **self == VCONN1_CFG_A::POS_EDG_DIS_NEG_EDG_EN
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_dis(&self) -> bool {
        **self == VCONN1_CFG_A::POS_EDG_EN_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_en(&self) -> bool {
        **self == VCONN1_CFG_A::POS_EDG_EN_NEG_EDG_EN
    }
}
impl core::ops::Deref for VCONN1_CFG_R {
    type Target = crate::FieldReader<u8, VCONN1_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN1_CFG` writer - Edge detect2 positive/negative enable/disable"]
pub struct VCONN1_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VCONN1_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCONN1_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCONN1_CFG_A::POS_EDG_DIS_NEG_EDG_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_en(self) -> &'a mut W {
        self.variant(VCONN1_CFG_A::POS_EDG_DIS_NEG_EDG_EN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCONN1_CFG_A::POS_EDG_EN_NEG_EDG_DIS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_en(self) -> &'a mut W {
        self.variant(VCONN1_CFG_A::POS_EDG_EN_NEG_EDG_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Edge detect2 positive/negative enable/disable\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VCONN2_CFG_A {
    #[doc = "0: `0`"]
    POS_EDG_DIS_NEG_EDG_DIS = 0,
    #[doc = "1: `1`"]
    POS_EDG_DIS_NEG_EDG_EN = 1,
    #[doc = "2: `10`"]
    POS_EDG_EN_NEG_EDG_DIS = 2,
    #[doc = "3: `11`"]
    POS_EDG_EN_NEG_EDG_EN = 3,
}
impl From<VCONN2_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: VCONN2_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VCONN2_CFG` reader - Edge detect2 positive/negative enable/disable"]
pub struct VCONN2_CFG_R(crate::FieldReader<u8, VCONN2_CFG_A>);
impl VCONN2_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCONN2_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCONN2_CFG_A {
        match self.bits {
            0 => VCONN2_CFG_A::POS_EDG_DIS_NEG_EDG_DIS,
            1 => VCONN2_CFG_A::POS_EDG_DIS_NEG_EDG_EN,
            2 => VCONN2_CFG_A::POS_EDG_EN_NEG_EDG_DIS,
            3 => VCONN2_CFG_A::POS_EDG_EN_NEG_EDG_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_dis(&self) -> bool {
        **self == VCONN2_CFG_A::POS_EDG_DIS_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_en(&self) -> bool {
        **self == VCONN2_CFG_A::POS_EDG_DIS_NEG_EDG_EN
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_dis(&self) -> bool {
        **self == VCONN2_CFG_A::POS_EDG_EN_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_en(&self) -> bool {
        **self == VCONN2_CFG_A::POS_EDG_EN_NEG_EDG_EN
    }
}
impl core::ops::Deref for VCONN2_CFG_R {
    type Target = crate::FieldReader<u8, VCONN2_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN2_CFG` writer - Edge detect2 positive/negative enable/disable"]
pub struct VCONN2_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VCONN2_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCONN2_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCONN2_CFG_A::POS_EDG_DIS_NEG_EDG_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_en(self) -> &'a mut W {
        self.variant(VCONN2_CFG_A::POS_EDG_DIS_NEG_EDG_EN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCONN2_CFG_A::POS_EDG_EN_NEG_EDG_DIS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_en(self) -> &'a mut W {
        self.variant(VCONN2_CFG_A::POS_EDG_EN_NEG_EDG_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Edge detect2 positive/negative enable/disable\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1_CFG_A {
    #[doc = "0: `0`"]
    POS_EDG_DIS_NEG_EDG_DIS = 0,
    #[doc = "1: `1`"]
    POS_EDG_DIS_NEG_EDG_EN = 1,
    #[doc = "2: `10`"]
    POS_EDG_EN_NEG_EDG_DIS = 2,
    #[doc = "3: `11`"]
    POS_EDG_EN_NEG_EDG_EN = 3,
}
impl From<CC1_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CC1_CFG` reader - Edge detect2 positive/negative enable/disable"]
pub struct CC1_CFG_R(crate::FieldReader<u8, CC1_CFG_A>);
impl CC1_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC1_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1_CFG_A {
        match self.bits {
            0 => CC1_CFG_A::POS_EDG_DIS_NEG_EDG_DIS,
            1 => CC1_CFG_A::POS_EDG_DIS_NEG_EDG_EN,
            2 => CC1_CFG_A::POS_EDG_EN_NEG_EDG_DIS,
            3 => CC1_CFG_A::POS_EDG_EN_NEG_EDG_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_dis(&self) -> bool {
        **self == CC1_CFG_A::POS_EDG_DIS_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_en(&self) -> bool {
        **self == CC1_CFG_A::POS_EDG_DIS_NEG_EDG_EN
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_dis(&self) -> bool {
        **self == CC1_CFG_A::POS_EDG_EN_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_en(&self) -> bool {
        **self == CC1_CFG_A::POS_EDG_EN_NEG_EDG_EN
    }
}
impl core::ops::Deref for CC1_CFG_R {
    type Target = crate::FieldReader<u8, CC1_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1_CFG` writer - Edge detect2 positive/negative enable/disable"]
pub struct CC1_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_dis(self) -> &'a mut W {
        self.variant(CC1_CFG_A::POS_EDG_DIS_NEG_EDG_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_en(self) -> &'a mut W {
        self.variant(CC1_CFG_A::POS_EDG_DIS_NEG_EDG_EN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_dis(self) -> &'a mut W {
        self.variant(CC1_CFG_A::POS_EDG_EN_NEG_EDG_DIS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_en(self) -> &'a mut W {
        self.variant(CC1_CFG_A::POS_EDG_EN_NEG_EDG_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Edge detect2 positive/negative enable/disable\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC2_CFG_A {
    #[doc = "0: `0`"]
    POS_EDG_DIS_NEG_EDG_DIS = 0,
    #[doc = "1: `1`"]
    POS_EDG_DIS_NEG_EDG_EN = 1,
    #[doc = "2: `10`"]
    POS_EDG_EN_NEG_EDG_DIS = 2,
    #[doc = "3: `11`"]
    POS_EDG_EN_NEG_EDG_EN = 3,
}
impl From<CC2_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CC2_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CC2_CFG` reader - Edge detect2 positive/negative enable/disable"]
pub struct CC2_CFG_R(crate::FieldReader<u8, CC2_CFG_A>);
impl CC2_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC2_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC2_CFG_A {
        match self.bits {
            0 => CC2_CFG_A::POS_EDG_DIS_NEG_EDG_DIS,
            1 => CC2_CFG_A::POS_EDG_DIS_NEG_EDG_EN,
            2 => CC2_CFG_A::POS_EDG_EN_NEG_EDG_DIS,
            3 => CC2_CFG_A::POS_EDG_EN_NEG_EDG_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_dis(&self) -> bool {
        **self == CC2_CFG_A::POS_EDG_DIS_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_en(&self) -> bool {
        **self == CC2_CFG_A::POS_EDG_DIS_NEG_EDG_EN
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_dis(&self) -> bool {
        **self == CC2_CFG_A::POS_EDG_EN_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_en(&self) -> bool {
        **self == CC2_CFG_A::POS_EDG_EN_NEG_EDG_EN
    }
}
impl core::ops::Deref for CC2_CFG_R {
    type Target = crate::FieldReader<u8, CC2_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2_CFG` writer - Edge detect2 positive/negative enable/disable"]
pub struct CC2_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_dis(self) -> &'a mut W {
        self.variant(CC2_CFG_A::POS_EDG_DIS_NEG_EDG_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_en(self) -> &'a mut W {
        self.variant(CC2_CFG_A::POS_EDG_DIS_NEG_EDG_EN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_dis(self) -> &'a mut W {
        self.variant(CC2_CFG_A::POS_EDG_EN_NEG_EDG_DIS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_en(self) -> &'a mut W {
        self.variant(CC2_CFG_A::POS_EDG_EN_NEG_EDG_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Edge detect2 positive/negative enable/disable\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VCMP_UP_CFG_A {
    #[doc = "0: `0`"]
    POS_EDG_DIS_NEG_EDG_DIS = 0,
    #[doc = "1: `1`"]
    POS_EDG_DIS_NEG_EDG_EN = 1,
    #[doc = "2: `10`"]
    POS_EDG_EN_NEG_EDG_DIS = 2,
    #[doc = "3: `11`"]
    POS_EDG_EN_NEG_EDG_EN = 3,
}
impl From<VCMP_UP_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: VCMP_UP_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VCMP_UP_CFG` reader - Edge detect2 positive/negative enable/disable"]
pub struct VCMP_UP_CFG_R(crate::FieldReader<u8, VCMP_UP_CFG_A>);
impl VCMP_UP_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCMP_UP_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCMP_UP_CFG_A {
        match self.bits {
            0 => VCMP_UP_CFG_A::POS_EDG_DIS_NEG_EDG_DIS,
            1 => VCMP_UP_CFG_A::POS_EDG_DIS_NEG_EDG_EN,
            2 => VCMP_UP_CFG_A::POS_EDG_EN_NEG_EDG_DIS,
            3 => VCMP_UP_CFG_A::POS_EDG_EN_NEG_EDG_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_dis(&self) -> bool {
        **self == VCMP_UP_CFG_A::POS_EDG_DIS_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_en(&self) -> bool {
        **self == VCMP_UP_CFG_A::POS_EDG_DIS_NEG_EDG_EN
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_dis(&self) -> bool {
        **self == VCMP_UP_CFG_A::POS_EDG_EN_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_en(&self) -> bool {
        **self == VCMP_UP_CFG_A::POS_EDG_EN_NEG_EDG_EN
    }
}
impl core::ops::Deref for VCMP_UP_CFG_R {
    type Target = crate::FieldReader<u8, VCMP_UP_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_UP_CFG` writer - Edge detect2 positive/negative enable/disable"]
pub struct VCMP_UP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_UP_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCMP_UP_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCMP_UP_CFG_A::POS_EDG_DIS_NEG_EDG_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_en(self) -> &'a mut W {
        self.variant(VCMP_UP_CFG_A::POS_EDG_DIS_NEG_EDG_EN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCMP_UP_CFG_A::POS_EDG_EN_NEG_EDG_DIS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_en(self) -> &'a mut W {
        self.variant(VCMP_UP_CFG_A::POS_EDG_EN_NEG_EDG_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Edge detect2 positive/negative enable/disable\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VCMP_DN_CFG_A {
    #[doc = "0: `0`"]
    POS_EDG_DIS_NEG_EDG_DIS = 0,
    #[doc = "1: `1`"]
    POS_EDG_DIS_NEG_EDG_EN = 1,
    #[doc = "2: `10`"]
    POS_EDG_EN_NEG_EDG_DIS = 2,
    #[doc = "3: `11`"]
    POS_EDG_EN_NEG_EDG_EN = 3,
}
impl From<VCMP_DN_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: VCMP_DN_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VCMP_DN_CFG` reader - Edge detect2 positive/negative enable/disable"]
pub struct VCMP_DN_CFG_R(crate::FieldReader<u8, VCMP_DN_CFG_A>);
impl VCMP_DN_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCMP_DN_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCMP_DN_CFG_A {
        match self.bits {
            0 => VCMP_DN_CFG_A::POS_EDG_DIS_NEG_EDG_DIS,
            1 => VCMP_DN_CFG_A::POS_EDG_DIS_NEG_EDG_EN,
            2 => VCMP_DN_CFG_A::POS_EDG_EN_NEG_EDG_DIS,
            3 => VCMP_DN_CFG_A::POS_EDG_EN_NEG_EDG_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_dis(&self) -> bool {
        **self == VCMP_DN_CFG_A::POS_EDG_DIS_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_en(&self) -> bool {
        **self == VCMP_DN_CFG_A::POS_EDG_DIS_NEG_EDG_EN
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_dis(&self) -> bool {
        **self == VCMP_DN_CFG_A::POS_EDG_EN_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_en(&self) -> bool {
        **self == VCMP_DN_CFG_A::POS_EDG_EN_NEG_EDG_EN
    }
}
impl core::ops::Deref for VCMP_DN_CFG_R {
    type Target = crate::FieldReader<u8, VCMP_DN_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_DN_CFG` writer - Edge detect2 positive/negative enable/disable"]
pub struct VCMP_DN_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_DN_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCMP_DN_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCMP_DN_CFG_A::POS_EDG_DIS_NEG_EDG_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_en(self) -> &'a mut W {
        self.variant(VCMP_DN_CFG_A::POS_EDG_DIS_NEG_EDG_EN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCMP_DN_CFG_A::POS_EDG_EN_NEG_EDG_DIS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_en(self) -> &'a mut W {
        self.variant(VCMP_DN_CFG_A::POS_EDG_EN_NEG_EDG_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Edge detect2 positive/negative enable/disable\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VCMP_LA_CFG_A {
    #[doc = "0: `0`"]
    POS_EDG_DIS_NEG_EDG_DIS = 0,
    #[doc = "1: `1`"]
    POS_EDG_DIS_NEG_EDG_EN = 1,
    #[doc = "2: `10`"]
    POS_EDG_EN_NEG_EDG_DIS = 2,
    #[doc = "3: `11`"]
    POS_EDG_EN_NEG_EDG_EN = 3,
}
impl From<VCMP_LA_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: VCMP_LA_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VCMP_LA_CFG` reader - Edge detect2 positive/negative enable/disable"]
pub struct VCMP_LA_CFG_R(crate::FieldReader<u8, VCMP_LA_CFG_A>);
impl VCMP_LA_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCMP_LA_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCMP_LA_CFG_A {
        match self.bits {
            0 => VCMP_LA_CFG_A::POS_EDG_DIS_NEG_EDG_DIS,
            1 => VCMP_LA_CFG_A::POS_EDG_DIS_NEG_EDG_EN,
            2 => VCMP_LA_CFG_A::POS_EDG_EN_NEG_EDG_DIS,
            3 => VCMP_LA_CFG_A::POS_EDG_EN_NEG_EDG_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_dis(&self) -> bool {
        **self == VCMP_LA_CFG_A::POS_EDG_DIS_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_DIS_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_dis_neg_edg_en(&self) -> bool {
        **self == VCMP_LA_CFG_A::POS_EDG_DIS_NEG_EDG_EN
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_DIS`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_dis(&self) -> bool {
        **self == VCMP_LA_CFG_A::POS_EDG_EN_NEG_EDG_DIS
    }
    #[doc = "Checks if the value of the field is `POS_EDG_EN_NEG_EDG_EN`"]
    #[inline(always)]
    pub fn is_pos_edg_en_neg_edg_en(&self) -> bool {
        **self == VCMP_LA_CFG_A::POS_EDG_EN_NEG_EDG_EN
    }
}
impl core::ops::Deref for VCMP_LA_CFG_R {
    type Target = crate::FieldReader<u8, VCMP_LA_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_LA_CFG` writer - Edge detect2 positive/negative enable/disable"]
pub struct VCMP_LA_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_LA_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCMP_LA_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCMP_LA_CFG_A::POS_EDG_DIS_NEG_EDG_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pos_edg_dis_neg_edg_en(self) -> &'a mut W {
        self.variant(VCMP_LA_CFG_A::POS_EDG_DIS_NEG_EDG_EN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_dis(self) -> &'a mut W {
        self.variant(VCMP_LA_CFG_A::POS_EDG_EN_NEG_EDG_DIS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pos_edg_en_neg_edg_en(self) -> &'a mut W {
        self.variant(VCMP_LA_CFG_A::POS_EDG_EN_NEG_EDG_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `VCONN1_FILT_EN` reader - Filtering the VCONN1_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCONN1_FILT_EN_R(crate::FieldReader<bool, bool>);
impl VCONN1_FILT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN1_FILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN1_FILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN1_FILT_EN` writer - Filtering the VCONN1_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCONN1_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCONN1_FILT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `VCONN2_FILT_EN` reader - Filtering the VCONN2_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCONN2_FILT_EN_R(crate::FieldReader<bool, bool>);
impl VCONN2_FILT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN2_FILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN2_FILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN2_FILT_EN` writer - Filtering the VCONN2_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCONN2_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCONN2_FILT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CC1_CFG_FILT_EN` reader - Filtering the CC1_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct CC1_CFG_FILT_EN_R(crate::FieldReader<bool, bool>);
impl CC1_CFG_FILT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC1_CFG_FILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1_CFG_FILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1_CFG_FILT_EN` writer - Filtering the CC1_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct CC1_CFG_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_CFG_FILT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CC2_FILT_EN` reader - Filtering the CC2_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct CC2_FILT_EN_R(crate::FieldReader<bool, bool>);
impl CC2_FILT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC2_FILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2_FILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2_FILT_EN` writer - Filtering the CC2_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct CC2_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2_FILT_EN_W<'a> {
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
#[doc = "Field `VCMP_UP_FILT_EN` reader - Filtering the VCMP_UP from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCMP_UP_FILT_EN_R(crate::FieldReader<bool, bool>);
impl VCMP_UP_FILT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_UP_FILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_UP_FILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_UP_FILT_EN` writer - Filtering the VCMP_UP from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCMP_UP_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_UP_FILT_EN_W<'a> {
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
#[doc = "Field `VCMP_DN_FILT_EN` reader - Filtering the VCMP_DNfrom s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCMP_DN_FILT_EN_R(crate::FieldReader<bool, bool>);
impl VCMP_DN_FILT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_DN_FILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_DN_FILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_DN_FILT_EN` writer - Filtering the VCMP_DNfrom s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
pub struct VCMP_DN_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_DN_FILT_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Source of INTR0.TX_PACKET_DONE 0: TX EOP 1: CC Output Enable de-assert"]
    #[inline(always)]
    pub fn intr0_tx_packet_done_cfg(&self) -> INTR0_TX_PACKET_DONE_CFG_R {
        INTR0_TX_PACKET_DONE_CFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Source of INTR0.TX_CC_DATA_OEN 0: CC Output Enable de-assert 1: TX EOP"]
    #[inline(always)]
    pub fn intr0_tx_cc_data_oen_cfg(&self) -> INTR0_TX_CC_DATA_OEN_CFG_R {
        INTR0_TX_CC_DATA_OEN_CFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vconn1_cfg(&self) -> VCONN1_CFG_R {
        VCONN1_CFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vconn2_cfg(&self) -> VCONN2_CFG_R {
        VCONN2_CFG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn cc1_cfg(&self) -> CC1_CFG_R {
        CC1_CFG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn cc2_cfg(&self) -> CC2_CFG_R {
        CC2_CFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vcmp_up_cfg(&self) -> VCMP_UP_CFG_R {
        VCMP_UP_CFG_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vcmp_dn_cfg(&self) -> VCMP_DN_CFG_R {
        VCMP_DN_CFG_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vcmp_la_cfg(&self) -> VCMP_LA_CFG_R {
        VCMP_LA_CFG_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Filtering the VCONN1_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vconn1_filt_en(&self) -> VCONN1_FILT_EN_R {
        VCONN1_FILT_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filtering the VCONN2_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vconn2_filt_en(&self) -> VCONN2_FILT_EN_R {
        VCONN2_FILT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filtering the CC1_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn cc1_cfg_filt_en(&self) -> CC1_CFG_FILT_EN_R {
        CC1_CFG_FILT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filtering the CC2_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn cc2_filt_en(&self) -> CC2_FILT_EN_R {
        CC2_FILT_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filtering the VCMP_UP from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vcmp_up_filt_en(&self) -> VCMP_UP_FILT_EN_R {
        VCMP_UP_FILT_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filtering the VCMP_DNfrom s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vcmp_dn_filt_en(&self) -> VCMP_DN_FILT_EN_R {
        VCMP_DN_FILT_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source of INTR0.TX_PACKET_DONE 0: TX EOP 1: CC Output Enable de-assert"]
    #[inline(always)]
    pub fn intr0_tx_packet_done_cfg(&mut self) -> INTR0_TX_PACKET_DONE_CFG_W {
        INTR0_TX_PACKET_DONE_CFG_W { w: self }
    }
    #[doc = "Bit 1 - Source of INTR0.TX_CC_DATA_OEN 0: CC Output Enable de-assert 1: TX EOP"]
    #[inline(always)]
    pub fn intr0_tx_cc_data_oen_cfg(&mut self) -> INTR0_TX_CC_DATA_OEN_CFG_W {
        INTR0_TX_CC_DATA_OEN_CFG_W { w: self }
    }
    #[doc = "Bits 2:3 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vconn1_cfg(&mut self) -> VCONN1_CFG_W {
        VCONN1_CFG_W { w: self }
    }
    #[doc = "Bits 4:5 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vconn2_cfg(&mut self) -> VCONN2_CFG_W {
        VCONN2_CFG_W { w: self }
    }
    #[doc = "Bits 6:7 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn cc1_cfg(&mut self) -> CC1_CFG_W {
        CC1_CFG_W { w: self }
    }
    #[doc = "Bits 8:9 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn cc2_cfg(&mut self) -> CC2_CFG_W {
        CC2_CFG_W { w: self }
    }
    #[doc = "Bits 10:11 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vcmp_up_cfg(&mut self) -> VCMP_UP_CFG_W {
        VCMP_UP_CFG_W { w: self }
    }
    #[doc = "Bits 12:13 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vcmp_dn_cfg(&mut self) -> VCMP_DN_CFG_W {
        VCMP_DN_CFG_W { w: self }
    }
    #[doc = "Bits 14:15 - Edge detect2 positive/negative enable/disable"]
    #[inline(always)]
    pub fn vcmp_la_cfg(&mut self) -> VCMP_LA_CFG_W {
        VCMP_LA_CFG_W { w: self }
    }
    #[doc = "Bit 16 - Filtering the VCONN1_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vconn1_filt_en(&mut self) -> VCONN1_FILT_EN_W {
        VCONN1_FILT_EN_W { w: self }
    }
    #[doc = "Bit 17 - Filtering the VCONN2_DET from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vconn2_filt_en(&mut self) -> VCONN2_FILT_EN_W {
        VCONN2_FILT_EN_W { w: self }
    }
    #[doc = "Bit 18 - Filtering the CC1_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn cc1_cfg_filt_en(&mut self) -> CC1_CFG_FILT_EN_W {
        CC1_CFG_FILT_EN_W { w: self }
    }
    #[doc = "Bit 19 - Filtering the CC2_ATTACH from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn cc2_filt_en(&mut self) -> CC2_FILT_EN_W {
        CC2_FILT_EN_W { w: self }
    }
    #[doc = "Bit 20 - Filtering the VCMP_UP from s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vcmp_up_filt_en(&mut self) -> VCMP_UP_FILT_EN_W {
        VCMP_UP_FILT_EN_W { w: self }
    }
    #[doc = "Bit 21 - Filtering the VCMP_DNfrom s8usbpd.: 0: No Filtering 1: Enable 3 cycles of clk_lf filtering"]
    #[inline(always)]
    pub fn vcmp_dn_filt_en(&mut self) -> VCMP_DN_FILT_EN_W {
        VCMP_DN_FILT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Interrupts edge and filter configuration and Intr0 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_0_1_cfg](index.html) module"]
pub struct INTR_0_1_CFG_SPEC;
impl crate::RegisterSpec for INTR_0_1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_0_1_cfg::R](R) reader structure"]
impl crate::Readable for INTR_0_1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_0_1_cfg::W](W) writer structure"]
impl crate::Writable for INTR_0_1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_0_1_CFG to value 0x003f_aaa8"]
impl crate::Resettable for INTR_0_1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003f_aaa8
    }
}
