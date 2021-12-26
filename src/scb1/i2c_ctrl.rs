#[doc = "Register `I2C_CTRL` reader"]
pub struct R(crate::R<I2C_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTRL` writer"]
pub struct W(crate::W<I2C_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTRL_SPEC>;
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
impl From<crate::W<I2C_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIGH_PHASE_OVS` reader - "]
pub struct HIGH_PHASE_OVS_R(crate::FieldReader<u8, u8>);
impl HIGH_PHASE_OVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HIGH_PHASE_OVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGH_PHASE_OVS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGH_PHASE_OVS` writer - "]
pub struct HIGH_PHASE_OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_PHASE_OVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `LOW_PHASE_OVS` reader - "]
pub struct LOW_PHASE_OVS_R(crate::FieldReader<u8, u8>);
impl LOW_PHASE_OVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LOW_PHASE_OVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_PHASE_OVS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW_PHASE_OVS` writer - "]
pub struct LOW_PHASE_OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_PHASE_OVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `M_READY_DATA_ACK` reader - "]
pub struct M_READY_DATA_ACK_R(crate::FieldReader<bool, bool>);
impl M_READY_DATA_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_READY_DATA_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_READY_DATA_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_READY_DATA_ACK` writer - "]
pub struct M_READY_DATA_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_READY_DATA_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `M_NOT_READY_DATA_NACK` reader - "]
pub struct M_NOT_READY_DATA_NACK_R(crate::FieldReader<bool, bool>);
impl M_NOT_READY_DATA_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_NOT_READY_DATA_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_NOT_READY_DATA_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_NOT_READY_DATA_NACK` writer - "]
pub struct M_NOT_READY_DATA_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_NOT_READY_DATA_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `S_GENERAL_IGNORE` reader - "]
pub struct S_GENERAL_IGNORE_R(crate::FieldReader<bool, bool>);
impl S_GENERAL_IGNORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_GENERAL_IGNORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_GENERAL_IGNORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_GENERAL_IGNORE` writer - "]
pub struct S_GENERAL_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> S_GENERAL_IGNORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `S_READY_ADDR_ACK` reader - "]
pub struct S_READY_ADDR_ACK_R(crate::FieldReader<bool, bool>);
impl S_READY_ADDR_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_READY_ADDR_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_READY_ADDR_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_READY_ADDR_ACK` writer - "]
pub struct S_READY_ADDR_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_READY_ADDR_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `S_READY_DATA_ACK` reader - "]
pub struct S_READY_DATA_ACK_R(crate::FieldReader<bool, bool>);
impl S_READY_DATA_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_READY_DATA_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_READY_DATA_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_READY_DATA_ACK` writer - "]
pub struct S_READY_DATA_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_READY_DATA_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `S_NOT_READY_ADDR_NACK` reader - "]
pub struct S_NOT_READY_ADDR_NACK_R(crate::FieldReader<bool, bool>);
impl S_NOT_READY_ADDR_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_NOT_READY_ADDR_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_NOT_READY_ADDR_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_NOT_READY_ADDR_NACK` writer - "]
pub struct S_NOT_READY_ADDR_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_NOT_READY_ADDR_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `S_NOT_READY_DATA_NACK` reader - "]
pub struct S_NOT_READY_DATA_NACK_R(crate::FieldReader<bool, bool>);
impl S_NOT_READY_DATA_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_NOT_READY_DATA_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_NOT_READY_DATA_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_NOT_READY_DATA_NACK` writer - "]
pub struct S_NOT_READY_DATA_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_NOT_READY_DATA_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `LOOPBACK` reader - "]
pub struct LOOPBACK_R(crate::FieldReader<bool, bool>);
impl LOOPBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPBACK` writer - "]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SLAVE_MODE` reader - "]
pub struct SLAVE_MODE_R(crate::FieldReader<bool, bool>);
impl SLAVE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_MODE` writer - "]
pub struct SLAVE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `MASTER_MODE` reader - "]
pub struct MASTER_MODE_R(crate::FieldReader<bool, bool>);
impl MASTER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_MODE` writer - "]
pub struct MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_MODE_W<'a> {
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
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn high_phase_ovs(&self) -> HIGH_PHASE_OVS_R {
        HIGH_PHASE_OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn low_phase_ovs(&self) -> LOW_PHASE_OVS_R {
        LOW_PHASE_OVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn m_ready_data_ack(&self) -> M_READY_DATA_ACK_R {
        M_READY_DATA_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&self) -> M_NOT_READY_DATA_NACK_R {
        M_NOT_READY_DATA_NACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn s_general_ignore(&self) -> S_GENERAL_IGNORE_R {
        S_GENERAL_IGNORE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&self) -> S_READY_ADDR_ACK_R {
        S_READY_ADDR_ACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn s_ready_data_ack(&self) -> S_READY_DATA_ACK_R {
        S_READY_DATA_ACK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&self) -> S_NOT_READY_ADDR_NACK_R {
        S_NOT_READY_ADDR_NACK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&self) -> S_NOT_READY_DATA_NACK_R {
        S_NOT_READY_DATA_NACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn high_phase_ovs(&mut self) -> HIGH_PHASE_OVS_W {
        HIGH_PHASE_OVS_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn low_phase_ovs(&mut self) -> LOW_PHASE_OVS_W {
        LOW_PHASE_OVS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn m_ready_data_ack(&mut self) -> M_READY_DATA_ACK_W {
        M_READY_DATA_ACK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&mut self) -> M_NOT_READY_DATA_NACK_W {
        M_NOT_READY_DATA_NACK_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn s_general_ignore(&mut self) -> S_GENERAL_IGNORE_W {
        S_GENERAL_IGNORE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&mut self) -> S_READY_ADDR_ACK_W {
        S_READY_ADDR_ACK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn s_ready_data_ack(&mut self) -> S_READY_DATA_ACK_W {
        S_READY_DATA_ACK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&mut self) -> S_NOT_READY_ADDR_NACK_W {
        S_NOT_READY_ADDR_NACK_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&mut self) -> S_NOT_READY_DATA_NACK_W {
        S_NOT_READY_DATA_NACK_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W {
        SLAVE_MODE_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctrl](index.html) module"]
pub struct I2C_CTRL_SPEC;
impl crate::RegisterSpec for I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctrl::R](R) reader structure"]
impl crate::Readable for I2C_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl::W](W) writer structure"]
impl crate::Writable for I2C_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CTRL to value 0xfb88"]
impl crate::Resettable for I2C_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfb88
    }
}
