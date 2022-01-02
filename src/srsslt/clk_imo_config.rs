#[doc = "Register `CLK_IMO_CONFIG` reader"]
pub struct R(crate::R<CLK_IMO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_IMO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_IMO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_IMO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_IMO_CONFIG` writer"]
pub struct W(crate::W<CLK_IMO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_IMO_CONFIG_SPEC>;
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
impl From<crate::W<CLK_IMO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_IMO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Master enable for IMO oscillator. Clearing this bit will disable the IMO. Don't do this if the system is running off it."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Master enable for IMO oscillator. Clearing this bit will disable the IMO. Don't do this if the system is running off it."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Master enable for IMO oscillator. Clearing this bit will disable the IMO. Don't do this if the system is running off it."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Master enable for IMO oscillator. Clearing this bit will disable the IMO. Don't do this if the system is running off it."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMO Configuration Internal high speed R/C oscillator configuration register. Note that this oscillator comes up active on power up. The oscillator provides the primary system clock (HFCLK) on power up until firmware configures differently. This oscillator is also used before system start to count out power up delays. This is done in fast IMO (FIMO) mode that does not require any external references and runs at a fixed 12MHz.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_imo_config](index.html) module"]
pub struct CLK_IMO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_IMO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_imo_config::R](R) reader structure"]
impl crate::Readable for CLK_IMO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_imo_config::W](W) writer structure"]
impl crate::Writable for CLK_IMO_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_IMO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for CLK_IMO_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
