#[doc = "Register `ADC_SAR_CTRL` reader"]
pub struct R(crate::R<ADC_SAR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SAR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SAR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SAR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_SAR_CTRL` writer"]
pub struct W(crate::W<ADC_SAR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SAR_CTRL_SPEC>;
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
impl From<crate::W<ADC_SAR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SAR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_EN` reader - "]
pub struct SAR_EN_R(crate::FieldReader<bool, bool>);
impl SAR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_EN` writer - "]
pub struct SAR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_EN_W<'a> {
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
#[doc = "Field `MID_VAL` reader - "]
pub struct MID_VAL_R(crate::FieldReader<u8, u8>);
impl MID_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MID_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MID_VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MID_VAL` writer - "]
pub struct MID_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MID_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SAR_OUT` reader - "]
pub struct SAR_OUT_R(crate::FieldReader<u8, u8>);
impl SAR_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTR_CMP_SEL_AW {
    #[doc = "0: `0`"]
    DIS_INT0_CMP_OUT_DET = 0,
    #[doc = "1: `1`"]
    EN_RISE_EDGE = 1,
    #[doc = "2: `10`"]
    EN_FALL_EDGE = 2,
    #[doc = "3: `11`"]
    EN_BOTH_EDGE = 3,
}
impl From<INTR_CMP_SEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: INTR_CMP_SEL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `INTR_CMP_SEL` writer - "]
pub struct INTR_CMP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_CMP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTR_CMP_SEL_AW) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dis_int0_cmp_out_det(self) -> &'a mut W {
        self.variant(INTR_CMP_SEL_AW::DIS_INT0_CMP_OUT_DET)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn en_rise_edge(self) -> &'a mut W {
        self.variant(INTR_CMP_SEL_AW::EN_RISE_EDGE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn en_fall_edge(self) -> &'a mut W {
        self.variant(INTR_CMP_SEL_AW::EN_FALL_EDGE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn en_both_edge(self) -> &'a mut W {
        self.variant(INTR_CMP_SEL_AW::EN_BOTH_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar_en(&self) -> SAR_EN_R {
        SAR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mid_val(&self) -> MID_VAL_R {
        MID_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sar_out(&self) -> SAR_OUT_R {
        SAR_OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar_en(&mut self) -> SAR_EN_W {
        SAR_EN_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mid_val(&mut self) -> MID_VAL_W {
        MID_VAL_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn intr_cmp_sel(&mut self) -> INTR_CMP_SEL_W {
        INTR_CMP_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_sar_ctrl](index.html) module"]
pub struct ADC_SAR_CTRL_SPEC;
impl crate::RegisterSpec for ADC_SAR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_sar_ctrl::R](R) reader structure"]
impl crate::Readable for ADC_SAR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_sar_ctrl::W](W) writer structure"]
impl crate::Writable for ADC_SAR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_SAR_CTRL to value 0x8000"]
impl crate::Resettable for ADC_SAR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
