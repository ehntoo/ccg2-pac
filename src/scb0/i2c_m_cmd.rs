#[doc = "Register `I2C_M_CMD` reader"]
pub struct R(crate::R<I2C_M_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_M_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_M_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_M_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_M_CMD` writer"]
pub struct W(crate::W<I2C_M_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_M_CMD_SPEC>;
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
impl From<crate::W<I2C_M_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_M_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_START` reader - "]
pub struct M_START_R(crate::FieldReader<bool, bool>);
impl M_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_START` writer - "]
pub struct M_START_W<'a> {
    w: &'a mut W,
}
impl<'a> M_START_W<'a> {
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
#[doc = "Field `M_START_ON_IDLE` reader - "]
pub struct M_START_ON_IDLE_R(crate::FieldReader<bool, bool>);
impl M_START_ON_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_START_ON_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_START_ON_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_START_ON_IDLE` writer - "]
pub struct M_START_ON_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> M_START_ON_IDLE_W<'a> {
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
#[doc = "Field `M_ACK` reader - "]
pub struct M_ACK_R(crate::FieldReader<bool, bool>);
impl M_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_ACK` writer - "]
pub struct M_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_ACK_W<'a> {
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
#[doc = "Field `M_NACK` reader - "]
pub struct M_NACK_R(crate::FieldReader<bool, bool>);
impl M_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_NACK` writer - "]
pub struct M_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_NACK_W<'a> {
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
#[doc = "Field `M_STOP` reader - "]
pub struct M_STOP_R(crate::FieldReader<bool, bool>);
impl M_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_STOP` writer - "]
pub struct M_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> M_STOP_W<'a> {
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
    pub fn m_start(&self) -> M_START_R {
        M_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m_start_on_idle(&self) -> M_START_ON_IDLE_R {
        M_START_ON_IDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn m_ack(&self) -> M_ACK_R {
        M_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn m_nack(&self) -> M_NACK_R {
        M_NACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn m_stop(&self) -> M_STOP_R {
        M_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn m_start(&mut self) -> M_START_W {
        M_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m_start_on_idle(&mut self) -> M_START_ON_IDLE_W {
        M_START_ON_IDLE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn m_ack(&mut self) -> M_ACK_W {
        M_ACK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn m_nack(&mut self) -> M_NACK_W {
        M_NACK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn m_stop(&mut self) -> M_STOP_W {
        M_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_m_cmd](index.html) module"]
pub struct I2C_M_CMD_SPEC;
impl crate::RegisterSpec for I2C_M_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_m_cmd::R](R) reader structure"]
impl crate::Readable for I2C_M_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_m_cmd::W](W) writer structure"]
impl crate::Writable for I2C_M_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_M_CMD to value 0"]
impl crate::Resettable for I2C_M_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
