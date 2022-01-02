#[doc = "Register `TR_CTRL0` reader"]
pub struct R(crate::R<TR_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTRL0` writer"]
pub struct W(crate::W<TR_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTRL0_SPEC>;
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
impl From<crate::W<TR_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE_SEL` reader - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
pub struct CAPTURE_SEL_R(crate::FieldReader<u8, u8>);
impl CAPTURE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPTURE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTURE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE_SEL` writer - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
pub struct CAPTURE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `COUNT_SEL` reader - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
pub struct COUNT_SEL_R(crate::FieldReader<u8, u8>);
impl COUNT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT_SEL` writer - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
pub struct COUNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `RELOAD_SEL` reader - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with the value in the TCPWM_CNTn_PERIOD register."]
pub struct RELOAD_SEL_R(crate::FieldReader<u8, u8>);
impl RELOAD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RELOAD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOAD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOAD_SEL` writer - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with the value in the TCPWM_CNTn_PERIOD register."]
pub struct RELOAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `STOP_SEL` reader - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
pub struct STOP_SEL_R(crate::FieldReader<u8, u8>);
impl STOP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STOP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_SEL` writer - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
pub struct STOP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `START_SEL` reader - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
pub struct START_SEL_R(crate::FieldReader<u8, u8>);
impl START_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        START_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_SEL` writer - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
pub struct START_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture_sel(&self) -> CAPTURE_SEL_R {
        CAPTURE_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn count_sel(&self) -> COUNT_SEL_R {
        COUNT_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with the value in the TCPWM_CNTn_PERIOD register."]
    #[inline(always)]
    pub fn reload_sel(&self) -> RELOAD_SEL_R {
        RELOAD_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&self) -> STOP_SEL_R {
        STOP_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&self) -> START_SEL_R {
        START_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture_sel(&mut self) -> CAPTURE_SEL_W {
        CAPTURE_SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn count_sel(&mut self) -> COUNT_SEL_W {
        COUNT_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with the value in the TCPWM_CNTn_PERIOD register."]
    #[inline(always)]
    pub fn reload_sel(&mut self) -> RELOAD_SEL_W {
        RELOAD_SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&mut self) -> STOP_SEL_W {
        STOP_SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&mut self) -> START_SEL_W {
        START_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter trigger control register 0 Used to select triggers for specific counter events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctrl0](index.html) module"]
pub struct TR_CTRL0_SPEC;
impl crate::RegisterSpec for TR_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctrl0::R](R) reader structure"]
impl crate::Readable for TR_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctrl0::W](W) writer structure"]
impl crate::Writable for TR_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_CTRL0 to value 0x10"]
impl crate::Resettable for TR_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
