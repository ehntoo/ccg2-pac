#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYPASS_EN` reader - "]
pub struct TX_BYPASS_EN_R(crate::FieldReader<bool, bool>);
impl TX_BYPASS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BYPASS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BYPASS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BYPASS_EN` writer - "]
pub struct TX_BYPASS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BYPASS_EN_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_BYPASS_EN_A {
    #[doc = "0: `0`"]
    NO_BYPASS = 0,
    #[doc = "1: `1`"]
    RX_SOP_ALLIGN = 1,
    #[doc = "2: `10`"]
    NO_RX_SOP_ALLGN = 2,
    #[doc = "3: `11`"]
    RSVD_BYPASS = 3,
}
impl From<RX_BYPASS_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_BYPASS_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_BYPASS_EN` reader - "]
pub struct RX_BYPASS_EN_R(crate::FieldReader<u8, RX_BYPASS_EN_A>);
impl RX_BYPASS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_BYPASS_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_BYPASS_EN_A {
        match self.bits {
            0 => RX_BYPASS_EN_A::NO_BYPASS,
            1 => RX_BYPASS_EN_A::RX_SOP_ALLIGN,
            2 => RX_BYPASS_EN_A::NO_RX_SOP_ALLGN,
            3 => RX_BYPASS_EN_A::RSVD_BYPASS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_BYPASS`"]
    #[inline(always)]
    pub fn is_no_bypass(&self) -> bool {
        **self == RX_BYPASS_EN_A::NO_BYPASS
    }
    #[doc = "Checks if the value of the field is `RX_SOP_ALLIGN`"]
    #[inline(always)]
    pub fn is_rx_sop_allign(&self) -> bool {
        **self == RX_BYPASS_EN_A::RX_SOP_ALLIGN
    }
    #[doc = "Checks if the value of the field is `NO_RX_SOP_ALLGN`"]
    #[inline(always)]
    pub fn is_no_rx_sop_allgn(&self) -> bool {
        **self == RX_BYPASS_EN_A::NO_RX_SOP_ALLGN
    }
    #[doc = "Checks if the value of the field is `RSVD_BYPASS`"]
    #[inline(always)]
    pub fn is_rsvd_bypass(&self) -> bool {
        **self == RX_BYPASS_EN_A::RSVD_BYPASS
    }
}
impl core::ops::Deref for RX_BYPASS_EN_R {
    type Target = crate::FieldReader<u8, RX_BYPASS_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BYPASS_EN` writer - "]
pub struct RX_BYPASS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BYPASS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_BYPASS_EN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_bypass(self) -> &'a mut W {
        self.variant(RX_BYPASS_EN_A::NO_BYPASS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rx_sop_allign(self) -> &'a mut W {
        self.variant(RX_BYPASS_EN_A::RX_SOP_ALLIGN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn no_rx_sop_allgn(self) -> &'a mut W {
        self.variant(RX_BYPASS_EN_A::NO_RX_SOP_ALLGN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rsvd_bypass(self) -> &'a mut W {
        self.variant(RX_BYPASS_EN_A::RSVD_BYPASS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `IP_ENABLED` reader - "]
pub struct IP_ENABLED_R(crate::FieldReader<bool, bool>);
impl IP_ENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IP_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_ENABLED` writer - "]
pub struct IP_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_ENABLED_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_bypass_en(&self) -> TX_BYPASS_EN_R {
        TX_BYPASS_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn rx_bypass_en(&self) -> RX_BYPASS_EN_R {
        RX_BYPASS_EN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ip_enabled(&self) -> IP_ENABLED_R {
        IP_ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_bypass_en(&mut self) -> TX_BYPASS_EN_W {
        TX_BYPASS_EN_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn rx_bypass_en(&mut self) -> RX_BYPASS_EN_W {
        RX_BYPASS_EN_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ip_enabled(&mut self) -> IP_ENABLED_W {
        IP_ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
