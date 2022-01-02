#[doc = "Register `PWR_DDFT_XRES` reader"]
pub struct R(crate::R<PWR_DDFT_XRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_DDFT_XRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_DDFT_XRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_DDFT_XRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_DDFT_XRES` writer"]
pub struct W(crate::W<PWR_DDFT_XRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_DDFT_XRES_SPEC>;
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
impl From<crate::W<PWR_DDFT_XRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_DDFT_XRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_IN` reader - Tied to the XRES DfT key observation shift register input."]
pub struct KEY_IN_R(crate::FieldReader<bool, bool>);
impl KEY_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_IN` writer - Tied to the XRES DfT key observation shift register input."]
pub struct KEY_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_IN_W<'a> {
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
#[doc = "Field `KEY_CLK` reader - Tied to the XRES DfT key observation shift register clock."]
pub struct KEY_CLK_R(crate::FieldReader<bool, bool>);
impl KEY_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_CLK` writer - Tied to the XRES DfT key observation shift register clock."]
pub struct KEY_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_CLK_W<'a> {
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
#[doc = "Field `HARD_KEY_OUT` reader - Output of the 32-bit hard key shift register"]
pub struct HARD_KEY_OUT_R(crate::FieldReader<bool, bool>);
impl HARD_KEY_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HARD_KEY_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HARD_KEY_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK` reader - Set this to 1 to block all test_key_* signals (blocks both logic side effects and bits in this register)"]
pub struct BLOCK_R(crate::FieldReader<bool, bool>);
impl BLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK` writer - Set this to 1 to block all test_key_* signals (blocks both logic side effects and bits in this register)"]
pub struct BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_W<'a> {
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
#[doc = "Field `KEY_DFT_EN` reader - Hooked up to test_key_dft_en"]
pub struct KEY_DFT_EN_R(crate::FieldReader<bool, bool>);
impl KEY_DFT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_DFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_DFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_REG_DISABLE` reader - Hooked up to test_key_reg_disable"]
pub struct KEY_REG_DISABLE_R(crate::FieldReader<bool, bool>);
impl KEY_REG_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_REG_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_REG_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_SAFE_MODE` reader - Hooked up to test_key_safe_mode"]
pub struct KEY_SAFE_MODE_R(crate::FieldReader<bool, bool>);
impl KEY_SAFE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_SAFE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_SAFE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_POR_CIRCUIT` reader - Hooked up to test_key_por_circuit"]
pub struct KEY_POR_CIRCUIT_R(crate::FieldReader<bool, bool>);
impl KEY_POR_CIRCUIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_POR_CIRCUIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_POR_CIRCUIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_CLK_EXT` reader - Hooked up to test_key_clk_ext"]
pub struct KEY_CLK_EXT_R(crate::FieldReader<bool, bool>);
impl KEY_CLK_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_CLK_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_CLK_EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HARD_KEY_OK` reader - Indicates that the 32-bit shift register has observed the correct key"]
pub struct HARD_KEY_OK_R(crate::FieldReader<bool, bool>);
impl HARD_KEY_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HARD_KEY_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HARD_KEY_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_KEY_OK` reader - Indicates that the 125-bit key observer has observed the correct key"]
pub struct SOFT_KEY_OK_R(crate::FieldReader<bool, bool>);
impl SOFT_KEY_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_KEY_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_KEY_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Tied to the XRES DfT key observation shift register input."]
    #[inline(always)]
    pub fn key_in(&self) -> KEY_IN_R {
        KEY_IN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tied to the XRES DfT key observation shift register clock."]
    #[inline(always)]
    pub fn key_clk(&self) -> KEY_CLK_R {
        KEY_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output of the 32-bit hard key shift register"]
    #[inline(always)]
    pub fn hard_key_out(&self) -> HARD_KEY_OUT_R {
        HARD_KEY_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this to 1 to block all test_key_* signals (blocks both logic side effects and bits in this register)"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Hooked up to test_key_dft_en"]
    #[inline(always)]
    pub fn key_dft_en(&self) -> KEY_DFT_EN_R {
        KEY_DFT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hooked up to test_key_reg_disable"]
    #[inline(always)]
    pub fn key_reg_disable(&self) -> KEY_REG_DISABLE_R {
        KEY_REG_DISABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Hooked up to test_key_safe_mode"]
    #[inline(always)]
    pub fn key_safe_mode(&self) -> KEY_SAFE_MODE_R {
        KEY_SAFE_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hooked up to test_key_por_circuit"]
    #[inline(always)]
    pub fn key_por_circuit(&self) -> KEY_POR_CIRCUIT_R {
        KEY_POR_CIRCUIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hooked up to test_key_clk_ext"]
    #[inline(always)]
    pub fn key_clk_ext(&self) -> KEY_CLK_EXT_R {
        KEY_CLK_EXT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Indicates that the 32-bit shift register has observed the correct key"]
    #[inline(always)]
    pub fn hard_key_ok(&self) -> HARD_KEY_OK_R {
        HARD_KEY_OK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Indicates that the 125-bit key observer has observed the correct key"]
    #[inline(always)]
    pub fn soft_key_ok(&self) -> SOFT_KEY_OK_R {
        SOFT_KEY_OK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tied to the XRES DfT key observation shift register input."]
    #[inline(always)]
    pub fn key_in(&mut self) -> KEY_IN_W {
        KEY_IN_W { w: self }
    }
    #[doc = "Bit 1 - Tied to the XRES DfT key observation shift register clock."]
    #[inline(always)]
    pub fn key_clk(&mut self) -> KEY_CLK_W {
        KEY_CLK_W { w: self }
    }
    #[doc = "Bit 3 - Set this to 1 to block all test_key_* signals (blocks both logic side effects and bits in this register)"]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XRES DfT Key observer logic test register This register is used to test the XRES TestMode key logic. It allows a test routine (firmware or ATE driven) to stimulate the key listener and observe its functionality. Extreme case must be taken in these tests, since they will result in actual test mode entry. For example, shifting in a scan mode key, will transition the system into scan mode immediately. Note that test_scan_mode is not observable in this register for that reason.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ddft_xres](index.html) module"]
pub struct PWR_DDFT_XRES_SPEC;
impl crate::RegisterSpec for PWR_DDFT_XRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ddft_xres::R](R) reader structure"]
impl crate::Readable for PWR_DDFT_XRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ddft_xres::W](W) writer structure"]
impl crate::Writable for PWR_DDFT_XRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_DDFT_XRES to value 0"]
impl crate::Resettable for PWR_DDFT_XRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
