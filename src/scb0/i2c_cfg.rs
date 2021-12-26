#[doc = "Register `I2C_CFG` reader"]
pub struct R(crate::R<I2C_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CFG` writer"]
pub struct W(crate::W<I2C_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CFG_SPEC>;
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
impl From<crate::W<I2C_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_IN_FILT_TRIM` reader - "]
pub struct SDA_IN_FILT_TRIM_R(crate::FieldReader<u8, u8>);
impl SDA_IN_FILT_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDA_IN_FILT_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_IN_FILT_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_IN_FILT_TRIM` writer - "]
pub struct SDA_IN_FILT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_IN_FILT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SDA_IN_FILT_SEL` reader - "]
pub struct SDA_IN_FILT_SEL_R(crate::FieldReader<bool, bool>);
impl SDA_IN_FILT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDA_IN_FILT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_IN_FILT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_IN_FILT_SEL` writer - "]
pub struct SDA_IN_FILT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_IN_FILT_SEL_W<'a> {
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
#[doc = "Field `SCL_IN_FILT_TRIM` reader - "]
pub struct SCL_IN_FILT_TRIM_R(crate::FieldReader<u8, u8>);
impl SCL_IN_FILT_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_IN_FILT_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_IN_FILT_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_IN_FILT_TRIM` writer - "]
pub struct SCL_IN_FILT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_IN_FILT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SCL_IN_FILT_SEL` reader - "]
pub struct SCL_IN_FILT_SEL_R(crate::FieldReader<bool, bool>);
impl SCL_IN_FILT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_IN_FILT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_IN_FILT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_IN_FILT_SEL` writer - "]
pub struct SCL_IN_FILT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_IN_FILT_SEL_W<'a> {
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
#[doc = "Field `SDA_OUT_FILT0_TRIM` reader - "]
pub struct SDA_OUT_FILT0_TRIM_R(crate::FieldReader<u8, u8>);
impl SDA_OUT_FILT0_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDA_OUT_FILT0_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_OUT_FILT0_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_OUT_FILT0_TRIM` writer - "]
pub struct SDA_OUT_FILT0_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT0_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `SDA_OUT_FILT1_TRIM` reader - "]
pub struct SDA_OUT_FILT1_TRIM_R(crate::FieldReader<u8, u8>);
impl SDA_OUT_FILT1_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDA_OUT_FILT1_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_OUT_FILT1_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_OUT_FILT1_TRIM` writer - "]
pub struct SDA_OUT_FILT1_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT1_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `SDA_OUT_FILT2_TRIM` reader - "]
pub struct SDA_OUT_FILT2_TRIM_R(crate::FieldReader<u8, u8>);
impl SDA_OUT_FILT2_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDA_OUT_FILT2_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_OUT_FILT2_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_OUT_FILT2_TRIM` writer - "]
pub struct SDA_OUT_FILT2_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT2_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `SDA_OUT_FILT_SEL` reader - "]
pub struct SDA_OUT_FILT_SEL_R(crate::FieldReader<u8, u8>);
impl SDA_OUT_FILT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDA_OUT_FILT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_OUT_FILT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_OUT_FILT_SEL` writer - "]
pub struct SDA_OUT_FILT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&self) -> SDA_IN_FILT_TRIM_R {
        SDA_IN_FILT_TRIM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sda_in_filt_sel(&self) -> SDA_IN_FILT_SEL_R {
        SDA_IN_FILT_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&self) -> SCL_IN_FILT_TRIM_R {
        SCL_IN_FILT_TRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn scl_in_filt_sel(&self) -> SCL_IN_FILT_SEL_R {
        SCL_IN_FILT_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&self) -> SDA_OUT_FILT0_TRIM_R {
        SDA_OUT_FILT0_TRIM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&self) -> SDA_OUT_FILT1_TRIM_R {
        SDA_OUT_FILT1_TRIM_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&self) -> SDA_OUT_FILT2_TRIM_R {
        SDA_OUT_FILT2_TRIM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sda_out_filt_sel(&self) -> SDA_OUT_FILT_SEL_R {
        SDA_OUT_FILT_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&mut self) -> SDA_IN_FILT_TRIM_W {
        SDA_IN_FILT_TRIM_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sda_in_filt_sel(&mut self) -> SDA_IN_FILT_SEL_W {
        SDA_IN_FILT_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&mut self) -> SCL_IN_FILT_TRIM_W {
        SCL_IN_FILT_TRIM_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn scl_in_filt_sel(&mut self) -> SCL_IN_FILT_SEL_W {
        SCL_IN_FILT_SEL_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&mut self) -> SDA_OUT_FILT0_TRIM_W {
        SDA_OUT_FILT0_TRIM_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&mut self) -> SDA_OUT_FILT1_TRIM_W {
        SDA_OUT_FILT1_TRIM_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&mut self) -> SDA_OUT_FILT2_TRIM_W {
        SDA_OUT_FILT2_TRIM_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sda_out_filt_sel(&mut self) -> SDA_OUT_FILT_SEL_W {
        SDA_OUT_FILT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cfg](index.html) module"]
pub struct I2C_CFG_SPEC;
impl crate::RegisterSpec for I2C_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_cfg::R](R) reader structure"]
impl crate::Readable for I2C_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_cfg::W](W) writer structure"]
impl crate::Writable for I2C_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CFG to value 0x002a_1013"]
impl crate::Resettable for I2C_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x002a_1013
    }
}
