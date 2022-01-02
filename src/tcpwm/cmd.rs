#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER_CAPTURE` reader - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub struct COUNTER_CAPTURE_R(crate::FieldReader<u8, u8>);
impl COUNTER_CAPTURE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER_CAPTURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_CAPTURE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_CAPTURE` writer - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub struct COUNTER_CAPTURE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_CAPTURE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `COUNTER_RELOAD` reader - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_RELOAD_R(crate::FieldReader<u8, u8>);
impl COUNTER_RELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER_RELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_RELOAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_RELOAD` writer - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `COUNTER_STOP` reader - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_STOP_R(crate::FieldReader<u8, u8>);
impl COUNTER_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_STOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_STOP` writer - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `COUNTER_START` reader - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_START_R(crate::FieldReader<u8, u8>);
impl COUNTER_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_START_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_START` writer - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_START_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&self) -> COUNTER_CAPTURE_R {
        COUNTER_CAPTURE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&self) -> COUNTER_RELOAD_R {
        COUNTER_RELOAD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&self) -> COUNTER_STOP_R {
        COUNTER_STOP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&self) -> COUNTER_START_R {
        COUNTER_START_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&mut self) -> COUNTER_CAPTURE_W {
        COUNTER_CAPTURE_W { w: self }
    }
    #[doc = "Bits 8:13 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&mut self) -> COUNTER_RELOAD_W {
        COUNTER_RELOAD_W { w: self }
    }
    #[doc = "Bits 16:21 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&mut self) -> COUNTER_STOP_W {
        COUNTER_STOP_W { w: self }
    }
    #[doc = "Bits 24:29 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&mut self) -> COUNTER_START_W {
        COUNTER_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCPWM command register. Enables software controlled counter operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
