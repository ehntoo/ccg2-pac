#[doc = "Register `TIMER_TRIGGER` reader"]
pub struct R(crate::R<TIMER_TRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_TRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_TRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_TRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_TRIGGER` writer"]
pub struct W(crate::W<TIMER_TRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_TRIGGER_SPEC>;
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
impl From<crate::W<TIMER_TRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_TRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_TRIGGER0` reader - "]
pub struct EN_TRIGGER0_R(crate::FieldReader<bool, bool>);
impl EN_TRIGGER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_TRIGGER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_TRIGGER0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_TRIGGER0` writer - "]
pub struct EN_TRIGGER0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_TRIGGER0_W<'a> {
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
#[doc = "Field `EN_TRIGGER1` reader - "]
pub struct EN_TRIGGER1_R(crate::FieldReader<bool, bool>);
impl EN_TRIGGER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_TRIGGER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_TRIGGER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_TRIGGER1` writer - "]
pub struct EN_TRIGGER1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_TRIGGER1_W<'a> {
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
#[doc = "Field `EN_TRIGGER2` reader - "]
pub struct EN_TRIGGER2_R(crate::FieldReader<bool, bool>);
impl EN_TRIGGER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_TRIGGER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_TRIGGER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_TRIGGER2` writer - "]
pub struct EN_TRIGGER2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_TRIGGER2_W<'a> {
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
#[doc = "Field `EN_TRIGGER3` reader - "]
pub struct EN_TRIGGER3_R(crate::FieldReader<bool, bool>);
impl EN_TRIGGER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_TRIGGER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_TRIGGER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_TRIGGER3` writer - "]
pub struct EN_TRIGGER3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_TRIGGER3_W<'a> {
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
#[doc = "Field `EN_TRIGGER4` reader - "]
pub struct EN_TRIGGER4_R(crate::FieldReader<bool, bool>);
impl EN_TRIGGER4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_TRIGGER4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_TRIGGER4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_TRIGGER4` writer - "]
pub struct EN_TRIGGER4_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_TRIGGER4_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en_trigger0(&self) -> EN_TRIGGER0_R {
        EN_TRIGGER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_trigger1(&self) -> EN_TRIGGER1_R {
        EN_TRIGGER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn en_trigger2(&self) -> EN_TRIGGER2_R {
        EN_TRIGGER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn en_trigger3(&self) -> EN_TRIGGER3_R {
        EN_TRIGGER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn en_trigger4(&self) -> EN_TRIGGER4_R {
        EN_TRIGGER4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en_trigger0(&mut self) -> EN_TRIGGER0_W {
        EN_TRIGGER0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_trigger1(&mut self) -> EN_TRIGGER1_W {
        EN_TRIGGER1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn en_trigger2(&mut self) -> EN_TRIGGER2_W {
        EN_TRIGGER2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn en_trigger3(&mut self) -> EN_TRIGGER3_W {
        EN_TRIGGER3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn en_trigger4(&mut self) -> EN_TRIGGER4_W {
        EN_TRIGGER4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_trigger](index.html) module"]
pub struct TIMER_TRIGGER_SPEC;
impl crate::RegisterSpec for TIMER_TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_trigger::R](R) reader structure"]
impl crate::Readable for TIMER_TRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_trigger::W](W) writer structure"]
impl crate::Writable for TIMER_TRIGGER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_TRIGGER to value 0"]
impl crate::Resettable for TIMER_TRIGGER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
