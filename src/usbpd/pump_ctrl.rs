#[doc = "Register `PUMP_CTRL` reader"]
pub struct R(crate::R<PUMP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUMP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUMP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUMP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUMP_CTRL` writer"]
pub struct W(crate::W<PUMP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUMP_CTRL_SPEC>;
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
impl From<crate::W<PUMP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUMP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADFT` reader - "]
pub struct ADFT_R(crate::FieldReader<u8, u8>);
impl ADFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADFT` writer - "]
pub struct ADFT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `BYPASS_LV` reader - "]
pub struct BYPASS_LV_R(crate::FieldReader<bool, bool>);
impl BYPASS_LV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS_LV` writer - "]
pub struct BYPASS_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_LV_W<'a> {
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
#[doc = "Field `CLK_SEL` reader - "]
pub struct CLK_SEL_R(crate::FieldReader<bool, bool>);
impl CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SEL` writer - "]
pub struct CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_W<'a> {
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
#[doc = "Field `PD_PUMP` reader - "]
pub struct PD_PUMP_R(crate::FieldReader<bool, bool>);
impl PD_PUMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_PUMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_PUMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_PUMP` writer - "]
pub struct PD_PUMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_PUMP_W<'a> {
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
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adft(&self) -> ADFT_R {
        ADFT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bypass_lv(&self) -> BYPASS_LV_R {
        BYPASS_LV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pd_pump(&self) -> PD_PUMP_R {
        PD_PUMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adft(&mut self) -> ADFT_W {
        ADFT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bypass_lv(&mut self) -> BYPASS_LV_W {
        BYPASS_LV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pd_pump(&mut self) -> PD_PUMP_W {
        PD_PUMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pump_ctrl](index.html) module"]
pub struct PUMP_CTRL_SPEC;
impl crate::RegisterSpec for PUMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pump_ctrl::R](R) reader structure"]
impl crate::Readable for PUMP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pump_ctrl::W](W) writer structure"]
impl crate::Writable for PUMP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUMP_CTRL to value 0x14"]
impl crate::Resettable for PUMP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
