#[doc = "Register `PTM_CTL` reader"]
pub struct R(crate::R<PTM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTM_CTL` writer"]
pub struct W(crate::W<PTM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTM_CTL_SPEC>;
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
impl From<crate::W<PTM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTM_EN` reader - 0': SWD mode. '1': PTM mode. This bit is typically set to '1' through the SWD interface to switch to the PTM interface and it is typically set to '0' through the PTM interface to switch to the SWD interface. This bit replaces the m0s8tst IP's CTRL.PTM_MODE_EN MMIO register field, which is rendered ineffective when using CPUSSv2."]
pub struct PTM_EN_R(crate::FieldReader<bool, bool>);
impl PTM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTM_EN` writer - 0': SWD mode. '1': PTM mode. This bit is typically set to '1' through the SWD interface to switch to the PTM interface and it is typically set to '0' through the PTM interface to switch to the SWD interface. This bit replaces the m0s8tst IP's CTRL.PTM_MODE_EN MMIO register field, which is rendered ineffective when using CPUSSv2."]
pub struct PTM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTM_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 0': SWD mode. '1': PTM mode. This bit is typically set to '1' through the SWD interface to switch to the PTM interface and it is typically set to '0' through the PTM interface to switch to the SWD interface. This bit replaces the m0s8tst IP's CTRL.PTM_MODE_EN MMIO register field, which is rendered ineffective when using CPUSSv2."]
    #[inline(always)]
    pub fn ptm_en(&self) -> PTM_EN_R {
        PTM_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0': SWD mode. '1': PTM mode. This bit is typically set to '1' through the SWD interface to switch to the PTM interface and it is typically set to '0' through the PTM interface to switch to the SWD interface. This bit replaces the m0s8tst IP's CTRL.PTM_MODE_EN MMIO register field, which is rendered ineffective when using CPUSSv2."]
    #[inline(always)]
    pub fn ptm_en(&mut self) -> PTM_EN_W {
        PTM_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel Test Mode Control Register This register determines the test interface (SWD or PTM) that connects the ATE to the device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptm_ctl](index.html) module"]
pub struct PTM_CTL_SPEC;
impl crate::RegisterSpec for PTM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptm_ctl::R](R) reader structure"]
impl crate::Readable for PTM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptm_ctl::W](W) writer structure"]
impl crate::Writable for PTM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTM_CTL to value 0"]
impl crate::Resettable for PTM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
