#[doc = "Register `PC2` reader"]
pub struct R(crate::R<PC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC2` writer"]
pub struct W(crate::W<PC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC2_SPEC>;
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
impl From<crate::W<PC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP_DIS0` reader - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
pub struct INP_DIS0_R(crate::FieldReader<bool, bool>);
impl INP_DIS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS0` writer - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
pub struct INP_DIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS0_W<'a> {
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
#[doc = "Field `INP_DIS1` reader - Disables the input buffer for IO pad 1."]
pub struct INP_DIS1_R(crate::FieldReader<bool, bool>);
impl INP_DIS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS1` writer - Disables the input buffer for IO pad 1."]
pub struct INP_DIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
    #[inline(always)]
    pub fn inp_dis0(&self) -> INP_DIS0_R {
        INP_DIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disables the input buffer for IO pad 1."]
    #[inline(always)]
    pub fn inp_dis1(&self) -> INP_DIS1_R {
        INP_DIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
    #[inline(always)]
    pub fn inp_dis0(&mut self) -> INP_DIS0_W {
        INP_DIS0_W { w: self }
    }
    #[doc = "Bit 1 - Disables the input buffer for IO pad 1."]
    #[inline(always)]
    pub fn inp_dis1(&mut self) -> INP_DIS1_W {
        INP_DIS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register 2 Configures the input disable for each pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc2](index.html) module"]
pub struct PC2_SPEC;
impl crate::RegisterSpec for PC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc2::R](R) reader structure"]
impl crate::Readable for PC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc2::W](W) writer structure"]
impl crate::Writable for PC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC2 to value 0"]
impl crate::Resettable for PC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
