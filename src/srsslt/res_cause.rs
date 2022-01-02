#[doc = "Register `RES_CAUSE` reader"]
pub struct R(crate::R<RES_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES_CAUSE` writer"]
pub struct W(crate::W<RES_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_CAUSE_SPEC>;
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
impl From<crate::W<RES_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_WDT` reader - A WatchDog Timer reset has occurred since last power cycle."]
pub struct RESET_WDT_R(crate::FieldReader<bool, bool>);
impl RESET_WDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_WDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_WDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_WDT` writer - A WatchDog Timer reset has occurred since last power cycle."]
pub struct RESET_WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_WDT_W<'a> {
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
#[doc = "Field `RESET_PROT_FAULT` reader - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
pub struct RESET_PROT_FAULT_R(crate::FieldReader<bool, bool>);
impl RESET_PROT_FAULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_PROT_FAULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_PROT_FAULT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_PROT_FAULT` writer - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
pub struct RESET_PROT_FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_PROT_FAULT_W<'a> {
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
#[doc = "Field `RESET_SOFT` reader - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub struct RESET_SOFT_R(crate::FieldReader<bool, bool>);
impl RESET_SOFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_SOFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_SOFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_SOFT` writer - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub struct RESET_SOFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_SOFT_W<'a> {
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
    #[doc = "Bit 0 - A WatchDog Timer reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&self) -> RESET_WDT_R {
        RESET_WDT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
    #[inline(always)]
    pub fn reset_prot_fault(&self) -> RESET_PROT_FAULT_R {
        RESET_PROT_FAULT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&self) -> RESET_SOFT_R {
        RESET_SOFT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A WatchDog Timer reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&mut self) -> RESET_WDT_W {
        RESET_WDT_W { w: self }
    }
    #[doc = "Bit 3 - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
    #[inline(always)]
    pub fn reset_prot_fault(&mut self) -> RESET_PROT_FAULT_W {
        RESET_PROT_FAULT_W { w: self }
    }
    #[doc = "Bit 4 - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&mut self) -> RESET_SOFT_W {
        RESET_SOFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Cause Observation Register Indicates the cause for the latest reset(s) that occurred in the system. Note that resets due to power up and brown-outs below state retention voltages in regulated and unregulated domains cannot be distinguished from eachother. All bits in this register assert when the corresponding reset cause occurs and must be cleared by firmware. These bits are cleared by hardware only during XRES, POR or after a detected brown-out.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause](index.html) module"]
pub struct RES_CAUSE_SPEC;
impl crate::RegisterSpec for RES_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res_cause::R](R) reader structure"]
impl crate::Readable for RES_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res_cause::W](W) writer structure"]
impl crate::Writable for RES_CAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES_CAUSE to value 0"]
impl crate::Resettable for RES_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
