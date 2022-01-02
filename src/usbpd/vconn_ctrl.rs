#[doc = "Register `VCONN_CTRL` reader"]
pub struct R(crate::R<VCONN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCONN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCONN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCONN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCONN_CTRL` writer"]
pub struct W(crate::W<VCONN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCONN_CTRL_SPEC>;
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
impl From<crate::W<VCONN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCONN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUMP_EN` reader - Negative Charge Pump enable signal 0 - Pump disabled: Ra termination is present on both VCONN1 and VCONN2 1 - Pump enabled: Ra termination is cutoff on VCONN1 only if the EN_COMP1 is set Ra termination is cutoff on VCONN2 only if the EN_COMP2 is set"]
pub struct PUMP_EN_R(crate::FieldReader<bool, bool>);
impl PUMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUMP_EN` writer - Negative Charge Pump enable signal 0 - Pump disabled: Ra termination is present on both VCONN1 and VCONN2 1 - Pump enabled: Ra termination is cutoff on VCONN1 only if the EN_COMP1 is set Ra termination is cutoff on VCONN2 only if the EN_COMP2 is set"]
pub struct PUMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_EN_W<'a> {
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
#[doc = "Field `EN_COMP1` reader - Enable VCONN1 comparator"]
pub struct EN_COMP1_R(crate::FieldReader<bool, bool>);
impl EN_COMP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_COMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_COMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_COMP1` writer - Enable VCONN1 comparator"]
pub struct EN_COMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_COMP1_W<'a> {
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
#[doc = "Field `EN_COMP2` reader - Enable VCONN2 comparator"]
pub struct EN_COMP2_R(crate::FieldReader<bool, bool>);
impl EN_COMP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_COMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_COMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_COMP2` writer - Enable VCONN2 comparator"]
pub struct EN_COMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_COMP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "VCONN1 leaker control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEAKER_CONFIG1_A {
    #[doc = "0: `0`"]
    LEAKER_DIS = 0,
    #[doc = "1: `1`"]
    SEL_200_UA = 1,
    #[doc = "2: `10`"]
    SEL_0_1_UF = 2,
    #[doc = "3: `11`"]
    SEL_0_5_UF = 3,
    #[doc = "4: `100`"]
    SEL_1_UF = 4,
    #[doc = "5: `101`"]
    SEL_2_UF = 5,
    #[doc = "6: `110`"]
    SEL_5_UF = 6,
    #[doc = "7: `111`"]
    SEL_10_UF = 7,
}
impl From<LEAKER_CONFIG1_A> for u8 {
    #[inline(always)]
    fn from(variant: LEAKER_CONFIG1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEAKER_CONFIG1` reader - VCONN1 leaker control"]
pub struct LEAKER_CONFIG1_R(crate::FieldReader<u8, LEAKER_CONFIG1_A>);
impl LEAKER_CONFIG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEAKER_CONFIG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAKER_CONFIG1_A {
        match self.bits {
            0 => LEAKER_CONFIG1_A::LEAKER_DIS,
            1 => LEAKER_CONFIG1_A::SEL_200_UA,
            2 => LEAKER_CONFIG1_A::SEL_0_1_UF,
            3 => LEAKER_CONFIG1_A::SEL_0_5_UF,
            4 => LEAKER_CONFIG1_A::SEL_1_UF,
            5 => LEAKER_CONFIG1_A::SEL_2_UF,
            6 => LEAKER_CONFIG1_A::SEL_5_UF,
            7 => LEAKER_CONFIG1_A::SEL_10_UF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEAKER_DIS`"]
    #[inline(always)]
    pub fn is_leaker_dis(&self) -> bool {
        **self == LEAKER_CONFIG1_A::LEAKER_DIS
    }
    #[doc = "Checks if the value of the field is `SEL_200_UA`"]
    #[inline(always)]
    pub fn is_sel_200_ua(&self) -> bool {
        **self == LEAKER_CONFIG1_A::SEL_200_UA
    }
    #[doc = "Checks if the value of the field is `SEL_0_1_UF`"]
    #[inline(always)]
    pub fn is_sel_0_1_uf(&self) -> bool {
        **self == LEAKER_CONFIG1_A::SEL_0_1_UF
    }
    #[doc = "Checks if the value of the field is `SEL_0_5_UF`"]
    #[inline(always)]
    pub fn is_sel_0_5_uf(&self) -> bool {
        **self == LEAKER_CONFIG1_A::SEL_0_5_UF
    }
    #[doc = "Checks if the value of the field is `SEL_1_UF`"]
    #[inline(always)]
    pub fn is_sel_1_uf(&self) -> bool {
        **self == LEAKER_CONFIG1_A::SEL_1_UF
    }
    #[doc = "Checks if the value of the field is `SEL_2_UF`"]
    #[inline(always)]
    pub fn is_sel_2_uf(&self) -> bool {
        **self == LEAKER_CONFIG1_A::SEL_2_UF
    }
    #[doc = "Checks if the value of the field is `SEL_5_UF`"]
    #[inline(always)]
    pub fn is_sel_5_uf(&self) -> bool {
        **self == LEAKER_CONFIG1_A::SEL_5_UF
    }
    #[doc = "Checks if the value of the field is `SEL_10_UF`"]
    #[inline(always)]
    pub fn is_sel_10_uf(&self) -> bool {
        **self == LEAKER_CONFIG1_A::SEL_10_UF
    }
}
impl core::ops::Deref for LEAKER_CONFIG1_R {
    type Target = crate::FieldReader<u8, LEAKER_CONFIG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEAKER_CONFIG1` writer - VCONN1 leaker control"]
pub struct LEAKER_CONFIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAKER_CONFIG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAKER_CONFIG1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn leaker_dis(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::LEAKER_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_200_ua(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::SEL_200_UA)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_0_1_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::SEL_0_1_UF)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_0_5_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::SEL_0_5_UF)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_1_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::SEL_1_UF)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_2_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::SEL_2_UF)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_5_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::SEL_5_UF)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_10_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG1_A::SEL_10_UF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "VCONN2 leaker control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEAKER_CONFIG2_A {
    #[doc = "0: `0`"]
    LEAKER_DIS = 0,
    #[doc = "1: `1`"]
    SEL_200_UA = 1,
    #[doc = "2: `10`"]
    SEL_0_1_UF = 2,
    #[doc = "3: `11`"]
    SEL_0_5_UF = 3,
    #[doc = "4: `100`"]
    SEL_1_UF = 4,
    #[doc = "5: `101`"]
    SEL_2_UF = 5,
    #[doc = "6: `110`"]
    SEL_5_UF = 6,
    #[doc = "7: `111`"]
    SEL_10_UF = 7,
}
impl From<LEAKER_CONFIG2_A> for u8 {
    #[inline(always)]
    fn from(variant: LEAKER_CONFIG2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEAKER_CONFIG2` reader - VCONN2 leaker control"]
pub struct LEAKER_CONFIG2_R(crate::FieldReader<u8, LEAKER_CONFIG2_A>);
impl LEAKER_CONFIG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEAKER_CONFIG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAKER_CONFIG2_A {
        match self.bits {
            0 => LEAKER_CONFIG2_A::LEAKER_DIS,
            1 => LEAKER_CONFIG2_A::SEL_200_UA,
            2 => LEAKER_CONFIG2_A::SEL_0_1_UF,
            3 => LEAKER_CONFIG2_A::SEL_0_5_UF,
            4 => LEAKER_CONFIG2_A::SEL_1_UF,
            5 => LEAKER_CONFIG2_A::SEL_2_UF,
            6 => LEAKER_CONFIG2_A::SEL_5_UF,
            7 => LEAKER_CONFIG2_A::SEL_10_UF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEAKER_DIS`"]
    #[inline(always)]
    pub fn is_leaker_dis(&self) -> bool {
        **self == LEAKER_CONFIG2_A::LEAKER_DIS
    }
    #[doc = "Checks if the value of the field is `SEL_200_UA`"]
    #[inline(always)]
    pub fn is_sel_200_ua(&self) -> bool {
        **self == LEAKER_CONFIG2_A::SEL_200_UA
    }
    #[doc = "Checks if the value of the field is `SEL_0_1_UF`"]
    #[inline(always)]
    pub fn is_sel_0_1_uf(&self) -> bool {
        **self == LEAKER_CONFIG2_A::SEL_0_1_UF
    }
    #[doc = "Checks if the value of the field is `SEL_0_5_UF`"]
    #[inline(always)]
    pub fn is_sel_0_5_uf(&self) -> bool {
        **self == LEAKER_CONFIG2_A::SEL_0_5_UF
    }
    #[doc = "Checks if the value of the field is `SEL_1_UF`"]
    #[inline(always)]
    pub fn is_sel_1_uf(&self) -> bool {
        **self == LEAKER_CONFIG2_A::SEL_1_UF
    }
    #[doc = "Checks if the value of the field is `SEL_2_UF`"]
    #[inline(always)]
    pub fn is_sel_2_uf(&self) -> bool {
        **self == LEAKER_CONFIG2_A::SEL_2_UF
    }
    #[doc = "Checks if the value of the field is `SEL_5_UF`"]
    #[inline(always)]
    pub fn is_sel_5_uf(&self) -> bool {
        **self == LEAKER_CONFIG2_A::SEL_5_UF
    }
    #[doc = "Checks if the value of the field is `SEL_10_UF`"]
    #[inline(always)]
    pub fn is_sel_10_uf(&self) -> bool {
        **self == LEAKER_CONFIG2_A::SEL_10_UF
    }
}
impl core::ops::Deref for LEAKER_CONFIG2_R {
    type Target = crate::FieldReader<u8, LEAKER_CONFIG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEAKER_CONFIG2` writer - VCONN2 leaker control"]
pub struct LEAKER_CONFIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAKER_CONFIG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAKER_CONFIG2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn leaker_dis(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::LEAKER_DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_200_ua(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::SEL_200_UA)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_0_1_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::SEL_0_1_UF)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_0_5_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::SEL_0_5_UF)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_1_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::SEL_1_UF)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_2_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::SEL_2_UF)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_5_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::SEL_5_UF)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_10_uf(self) -> &'a mut W {
        self.variant(LEAKER_CONFIG2_A::SEL_10_UF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Negative Charge Pump enable signal 0 - Pump disabled: Ra termination is present on both VCONN1 and VCONN2 1 - Pump enabled: Ra termination is cutoff on VCONN1 only if the EN_COMP1 is set Ra termination is cutoff on VCONN2 only if the EN_COMP2 is set"]
    #[inline(always)]
    pub fn pump_en(&self) -> PUMP_EN_R {
        PUMP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable VCONN1 comparator"]
    #[inline(always)]
    pub fn en_comp1(&self) -> EN_COMP1_R {
        EN_COMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable VCONN2 comparator"]
    #[inline(always)]
    pub fn en_comp2(&self) -> EN_COMP2_R {
        EN_COMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - VCONN1 leaker control"]
    #[inline(always)]
    pub fn leaker_config1(&self) -> LEAKER_CONFIG1_R {
        LEAKER_CONFIG1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - VCONN2 leaker control"]
    #[inline(always)]
    pub fn leaker_config2(&self) -> LEAKER_CONFIG2_R {
        LEAKER_CONFIG2_R::new(((self.bits >> 6) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Negative Charge Pump enable signal 0 - Pump disabled: Ra termination is present on both VCONN1 and VCONN2 1 - Pump enabled: Ra termination is cutoff on VCONN1 only if the EN_COMP1 is set Ra termination is cutoff on VCONN2 only if the EN_COMP2 is set"]
    #[inline(always)]
    pub fn pump_en(&mut self) -> PUMP_EN_W {
        PUMP_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable VCONN1 comparator"]
    #[inline(always)]
    pub fn en_comp1(&mut self) -> EN_COMP1_W {
        EN_COMP1_W { w: self }
    }
    #[doc = "Bit 2 - Enable VCONN2 comparator"]
    #[inline(always)]
    pub fn en_comp2(&mut self) -> EN_COMP2_W {
        EN_COMP2_W { w: self }
    }
    #[doc = "Bits 3:5 - VCONN1 leaker control"]
    #[inline(always)]
    pub fn leaker_config1(&mut self) -> LEAKER_CONFIG1_W {
        LEAKER_CONFIG1_W { w: self }
    }
    #[doc = "Bits 6:8 - VCONN2 leaker control"]
    #[inline(always)]
    pub fn leaker_config2(&mut self) -> LEAKER_CONFIG2_W {
        LEAKER_CONFIG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD VCONN control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vconn_ctrl](index.html) module"]
pub struct VCONN_CTRL_SPEC;
impl crate::RegisterSpec for VCONN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vconn_ctrl::R](R) reader structure"]
impl crate::Readable for VCONN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vconn_ctrl::W](W) writer structure"]
impl crate::Writable for VCONN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VCONN_CTRL to value 0"]
impl crate::Resettable for VCONN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
