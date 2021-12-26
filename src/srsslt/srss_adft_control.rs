#[doc = "Register `SRSS_ADFT_CONTROL` reader"]
pub struct R(crate::R<SRSS_ADFT_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSS_ADFT_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSS_ADFT_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSS_ADFT_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSS_ADFT_CONTROL` writer"]
pub struct W(crate::W<SRSS_ADFT_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSS_ADFT_CONTROL_SPEC>;
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
impl From<crate::W<SRSS_ADFT_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSS_ADFT_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_REF_EN` reader - "]
pub struct ACT_REF_EN_R(crate::FieldReader<bool, bool>);
impl ACT_REF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACT_REF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REF_EN` writer - "]
pub struct ACT_REF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_EN_W<'a> {
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
#[doc = "Field `ACT_COMP_EN` reader - "]
pub struct ACT_COMP_EN_R(crate::FieldReader<bool, bool>);
impl ACT_COMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACT_COMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_COMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_COMP_EN` writer - "]
pub struct ACT_COMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_COMP_EN_W<'a> {
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
#[doc = "Field `DPSLP_REF_EN` reader - "]
pub struct DPSLP_REF_EN_R(crate::FieldReader<bool, bool>);
impl DPSLP_REF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_REF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_REF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_REF_EN` writer - "]
pub struct DPSLP_REF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_EN_W<'a> {
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
#[doc = "Field `DPSLP_REG_EN` reader - "]
pub struct DPSLP_REG_EN_R(crate::FieldReader<bool, bool>);
impl DPSLP_REG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_REG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_REG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_REG_EN` writer - "]
pub struct DPSLP_REG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DPSLP_COMP_EN` reader - "]
pub struct DPSLP_COMP_EN_R(crate::FieldReader<bool, bool>);
impl DPSLP_COMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_COMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_COMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_COMP_EN` writer - "]
pub struct DPSLP_COMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_COMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DPSLP_REF_MODE` reader - "]
pub struct DPSLP_REF_MODE_R(crate::FieldReader<bool, bool>);
impl DPSLP_REF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_REF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_REF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_REF_MODE` writer - "]
pub struct DPSLP_REF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ACT_REF_VALID` reader - "]
pub struct ACT_REF_VALID_R(crate::FieldReader<bool, bool>);
impl ACT_REF_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACT_REF_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REF_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REG_VALID` reader - "]
pub struct ACT_REG_VALID_R(crate::FieldReader<bool, bool>);
impl ACT_REG_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACT_REG_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REG_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_COMP_OUT` reader - "]
pub struct ACT_COMP_OUT_R(crate::FieldReader<bool, bool>);
impl ACT_COMP_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACT_COMP_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_COMP_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_COMP_OUT` reader - "]
pub struct DPSLP_COMP_OUT_R(crate::FieldReader<bool, bool>);
impl DPSLP_COMP_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_COMP_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_COMP_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn act_ref_en(&self) -> ACT_REF_EN_R {
        ACT_REF_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn act_comp_en(&self) -> ACT_COMP_EN_R {
        ACT_COMP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dpslp_ref_en(&self) -> DPSLP_REF_EN_R {
        DPSLP_REF_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dpslp_reg_en(&self) -> DPSLP_REG_EN_R {
        DPSLP_REG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dpslp_comp_en(&self) -> DPSLP_COMP_EN_R {
        DPSLP_COMP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dpslp_ref_mode(&self) -> DPSLP_REF_MODE_R {
        DPSLP_REF_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn act_ref_valid(&self) -> ACT_REF_VALID_R {
        ACT_REF_VALID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn act_reg_valid(&self) -> ACT_REG_VALID_R {
        ACT_REG_VALID_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn act_comp_out(&self) -> ACT_COMP_OUT_R {
        ACT_COMP_OUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dpslp_comp_out(&self) -> DPSLP_COMP_OUT_R {
        DPSLP_COMP_OUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn act_ref_en(&mut self) -> ACT_REF_EN_W {
        ACT_REF_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn act_comp_en(&mut self) -> ACT_COMP_EN_W {
        ACT_COMP_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dpslp_ref_en(&mut self) -> DPSLP_REF_EN_W {
        DPSLP_REF_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dpslp_reg_en(&mut self) -> DPSLP_REG_EN_W {
        DPSLP_REG_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dpslp_comp_en(&mut self) -> DPSLP_COMP_EN_W {
        DPSLP_COMP_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dpslp_ref_mode(&mut self) -> DPSLP_REF_MODE_W {
        DPSLP_REF_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_adft_control](index.html) module"]
pub struct SRSS_ADFT_CONTROL_SPEC;
impl crate::RegisterSpec for SRSS_ADFT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srss_adft_control::R](R) reader structure"]
impl crate::Readable for SRSS_ADFT_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srss_adft_control::W](W) writer structure"]
impl crate::Writable for SRSS_ADFT_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSS_ADFT_CONTROL to value 0x3f"]
impl crate::Resettable for SRSS_ADFT_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
