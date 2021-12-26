#[doc = "Register `RX_SOP_GOOD_CRC_EN_CTRL` reader"]
pub struct R(crate::R<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_SOP_GOOD_CRC_EN_CTRL` writer"]
pub struct W(crate::W<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>;
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
impl From<crate::W<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SOP_GOOD_CRC_EN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SEND_DEFAULT_SOP_GOOD_CRC_EN` reader - "]
pub struct TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_R(crate::FieldReader<bool, bool>);
impl TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SEND_DEFAULT_SOP_GOOD_CRC_EN` writer - "]
pub struct TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_W<'a> {
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
#[doc = "Field `TX_SEND_PRIME_SOP_GOOD_CRC_EN` reader - "]
pub struct TX_SEND_PRIME_SOP_GOOD_CRC_EN_R(crate::FieldReader<bool, bool>);
impl TX_SEND_PRIME_SOP_GOOD_CRC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SEND_PRIME_SOP_GOOD_CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SEND_PRIME_SOP_GOOD_CRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SEND_PRIME_SOP_GOOD_CRC_EN` writer - "]
pub struct TX_SEND_PRIME_SOP_GOOD_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SEND_PRIME_SOP_GOOD_CRC_EN_W<'a> {
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
#[doc = "Field `TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN` reader - "]
pub struct TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_R(crate::FieldReader<bool, bool>);
impl TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN` writer - "]
pub struct TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_W<'a> {
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
#[doc = "Field `SEND_GOOD_CRC_BAD_EOP` reader - "]
pub struct SEND_GOOD_CRC_BAD_EOP_R(crate::FieldReader<bool, bool>);
impl SEND_GOOD_CRC_BAD_EOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_GOOD_CRC_BAD_EOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_GOOD_CRC_BAD_EOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_GOOD_CRC_BAD_EOP` writer - "]
pub struct SEND_GOOD_CRC_BAD_EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_GOOD_CRC_BAD_EOP_W<'a> {
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
#[doc = "Field `SEND_GOOD_CRC_BAD_KCHAR` reader - "]
pub struct SEND_GOOD_CRC_BAD_KCHAR_R(crate::FieldReader<bool, bool>);
impl SEND_GOOD_CRC_BAD_KCHAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_GOOD_CRC_BAD_KCHAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_GOOD_CRC_BAD_KCHAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_GOOD_CRC_BAD_KCHAR` writer - "]
pub struct SEND_GOOD_CRC_BAD_KCHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_GOOD_CRC_BAD_KCHAR_W<'a> {
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
#[doc = "Field `SEND_GOOD_CRC_NOISE_PKT` reader - "]
pub struct SEND_GOOD_CRC_NOISE_PKT_R(crate::FieldReader<bool, bool>);
impl SEND_GOOD_CRC_NOISE_PKT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_GOOD_CRC_NOISE_PKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_GOOD_CRC_NOISE_PKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_GOOD_CRC_NOISE_PKT` writer - "]
pub struct SEND_GOOD_CRC_NOISE_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_GOOD_CRC_NOISE_PKT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_send_default_sop_good_crc_en(&self) -> TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_R {
        TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_send_prime_sop_good_crc_en(&self) -> TX_SEND_PRIME_SOP_GOOD_CRC_EN_R {
        TX_SEND_PRIME_SOP_GOOD_CRC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_send_dbl_prime_sop_good_crc_en(&self) -> TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_R {
        TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn send_good_crc_bad_eop(&self) -> SEND_GOOD_CRC_BAD_EOP_R {
        SEND_GOOD_CRC_BAD_EOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_good_crc_bad_kchar(&self) -> SEND_GOOD_CRC_BAD_KCHAR_R {
        SEND_GOOD_CRC_BAD_KCHAR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_good_crc_noise_pkt(&self) -> SEND_GOOD_CRC_NOISE_PKT_R {
        SEND_GOOD_CRC_NOISE_PKT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_send_default_sop_good_crc_en(&mut self) -> TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_W {
        TX_SEND_DEFAULT_SOP_GOOD_CRC_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_send_prime_sop_good_crc_en(&mut self) -> TX_SEND_PRIME_SOP_GOOD_CRC_EN_W {
        TX_SEND_PRIME_SOP_GOOD_CRC_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_send_dbl_prime_sop_good_crc_en(&mut self) -> TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_W {
        TX_SEND_DBL_PRIME_SOP_GOOD_CRC_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn send_good_crc_bad_eop(&mut self) -> SEND_GOOD_CRC_BAD_EOP_W {
        SEND_GOOD_CRC_BAD_EOP_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_good_crc_bad_kchar(&mut self) -> SEND_GOOD_CRC_BAD_KCHAR_W {
        SEND_GOOD_CRC_BAD_KCHAR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_good_crc_noise_pkt(&mut self) -> SEND_GOOD_CRC_NOISE_PKT_W {
        SEND_GOOD_CRC_NOISE_PKT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_sop_good_crc_en_ctrl](index.html) module"]
pub struct RX_SOP_GOOD_CRC_EN_CTRL_SPEC;
impl crate::RegisterSpec for RX_SOP_GOOD_CRC_EN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_sop_good_crc_en_ctrl::R](R) reader structure"]
impl crate::Readable for RX_SOP_GOOD_CRC_EN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_sop_good_crc_en_ctrl::W](W) writer structure"]
impl crate::Writable for RX_SOP_GOOD_CRC_EN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_SOP_GOOD_CRC_EN_CTRL to value 0x38"]
impl crate::Resettable for RX_SOP_GOOD_CRC_EN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x38
    }
}
