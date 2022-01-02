#[doc = "Register `DPSLP_REF_CTRL` reader"]
pub struct R(crate::R<DPSLP_REF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSLP_REF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSLP_REF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSLP_REF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSLP_REF_CTRL` writer"]
pub struct W(crate::W<DPSLP_REF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSLP_REF_CTRL_SPEC>;
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
impl From<crate::W<DPSLP_REF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSLP_REF_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IGEN_EN` reader - Setting this bit will enable the deepsleep current reference outputs."]
pub struct IGEN_EN_R(crate::FieldReader<bool, bool>);
impl IGEN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IGEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGEN_EN` writer - Setting this bit will enable the deepsleep current reference outputs."]
pub struct IGEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IGEN_EN_W<'a> {
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
#[doc = "Field `DPSLP_ADFT_EN` reader - Setting this bit will enable the deepsleep reference generator ADFT mode."]
pub struct DPSLP_ADFT_EN_R(crate::FieldReader<bool, bool>);
impl DPSLP_ADFT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_ADFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_ADFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_ADFT_EN` writer - Setting this bit will enable the deepsleep reference generator ADFT mode."]
pub struct DPSLP_ADFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_ADFT_EN_W<'a> {
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
#[doc = "Field `ADFT_CTRL` reader - Controls the Deep Sleep reference ADFT mode 0: ganged 7 iref current sources 1: vrefdpslp voltage reference"]
pub struct ADFT_CTRL_R(crate::FieldReader<bool, bool>);
impl ADFT_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADFT_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADFT_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADFT_CTRL` writer - Controls the Deep Sleep reference ADFT mode 0: ganged 7 iref current sources 1: vrefdpslp voltage reference"]
pub struct ADFT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFT_CTRL_W<'a> {
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
#[doc = "Field `PD_DPSLP` reader - Block enable input 1 - All analog and DC paths cut off, outputs forced to known value This completely disables the CC Transceiver/Detect block. 0 - Normal functionality"]
pub struct PD_DPSLP_R(crate::FieldReader<bool, bool>);
impl PD_DPSLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_DPSLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_DPSLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_DPSLP` writer - Block enable input 1 - All analog and DC paths cut off, outputs forced to known value This completely disables the CC Transceiver/Detect block. 0 - Normal functionality"]
pub struct PD_DPSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_DPSLP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Setting this bit will enable the deepsleep current reference outputs."]
    #[inline(always)]
    pub fn igen_en(&self) -> IGEN_EN_R {
        IGEN_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting this bit will enable the deepsleep reference generator ADFT mode."]
    #[inline(always)]
    pub fn dpslp_adft_en(&self) -> DPSLP_ADFT_EN_R {
        DPSLP_ADFT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls the Deep Sleep reference ADFT mode 0: ganged 7 iref current sources 1: vrefdpslp voltage reference"]
    #[inline(always)]
    pub fn adft_ctrl(&self) -> ADFT_CTRL_R {
        ADFT_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Block enable input 1 - All analog and DC paths cut off, outputs forced to known value This completely disables the CC Transceiver/Detect block. 0 - Normal functionality"]
    #[inline(always)]
    pub fn pd_dpslp(&self) -> PD_DPSLP_R {
        PD_DPSLP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will enable the deepsleep current reference outputs."]
    #[inline(always)]
    pub fn igen_en(&mut self) -> IGEN_EN_W {
        IGEN_EN_W { w: self }
    }
    #[doc = "Bit 1 - Setting this bit will enable the deepsleep reference generator ADFT mode."]
    #[inline(always)]
    pub fn dpslp_adft_en(&mut self) -> DPSLP_ADFT_EN_W {
        DPSLP_ADFT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Controls the Deep Sleep reference ADFT mode 0: ganged 7 iref current sources 1: vrefdpslp voltage reference"]
    #[inline(always)]
    pub fn adft_ctrl(&mut self) -> ADFT_CTRL_W {
        ADFT_CTRL_W { w: self }
    }
    #[doc = "Bit 3 - Block enable input 1 - All analog and DC paths cut off, outputs forced to known value This completely disables the CC Transceiver/Detect block. 0 - Normal functionality"]
    #[inline(always)]
    pub fn pd_dpslp(&mut self) -> PD_DPSLP_W {
        PD_DPSLP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD DeepSleep-Reference Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpslp_ref_ctrl](index.html) module"]
pub struct DPSLP_REF_CTRL_SPEC;
impl crate::RegisterSpec for DPSLP_REF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpslp_ref_ctrl::R](R) reader structure"]
impl crate::Readable for DPSLP_REF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpslp_ref_ctrl::W](W) writer structure"]
impl crate::Writable for DPSLP_REF_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPSLP_REF_CTRL to value 0x09"]
impl crate::Resettable for DPSLP_REF_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09
    }
}
