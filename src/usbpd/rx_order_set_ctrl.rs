#[doc = "Register `RX_ORDER_SET_CTRL` reader"]
pub struct R(crate::R<RX_ORDER_SET_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ORDER_SET_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ORDER_SET_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ORDER_SET_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_ORDER_SET_CTRL` writer"]
pub struct W(crate::W<RX_ORDER_SET_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_ORDER_SET_CTRL_SPEC>;
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
impl From<crate::W<RX_ORDER_SET_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_ORDER_SET_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOP_CMP_OPT` reader - This register is used for SOP, SOP',SOP'\", DEBUG SOP', DEBUG SOP\" and RX_RESERVED1/2_ORDER_SET(if configured for SOP) oder set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
pub struct SOP_CMP_OPT_R(crate::FieldReader<bool, bool>);
impl SOP_CMP_OPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOP_CMP_OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOP_CMP_OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOP_CMP_OPT` writer - This register is used for SOP, SOP',SOP'\", DEBUG SOP', DEBUG SOP\" and RX_RESERVED1/2_ORDER_SET(if configured for SOP) oder set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
pub struct SOP_CMP_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOP_CMP_OPT_W<'a> {
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
#[doc = "Field `RST_CMP_OPT` reader - This register is used for Cable RST, Hard RST and RX_RESERVED1/2_ORDER_SET(if configure for RST) order set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
pub struct RST_CMP_OPT_R(crate::FieldReader<bool, bool>);
impl RST_CMP_OPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_CMP_OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_CMP_OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_CMP_OPT` writer - This register is used for Cable RST, Hard RST and RX_RESERVED1/2_ORDER_SET(if configure for RST) order set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
pub struct RST_CMP_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_CMP_OPT_W<'a> {
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
#[doc = "Field `PREAMBLE_SOP_EN` reader - This register is used to enable/disdable 16-bit preamble detection for SOP detection. 0: SOP Detection: SOP logic detection 1: SOP detection: Preamble(16-bit)+ SOP logic detection"]
pub struct PREAMBLE_SOP_EN_R(crate::FieldReader<bool, bool>);
impl PREAMBLE_SOP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREAMBLE_SOP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_SOP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREAMBLE_SOP_EN` writer - This register is used to enable/disdable 16-bit preamble detection for SOP detection. 0: SOP Detection: SOP logic detection 1: SOP detection: Preamble(16-bit)+ SOP logic detection"]
pub struct PREAMBLE_SOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_SOP_EN_W<'a> {
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
#[doc = "Field `PREAMBLE_RST_EN` reader - This register is used to enable/disdable 16-bit preamble detection for RST detection. 0: RST Detection: RST logic detection 1: RST detection: Preamble(16-bit)+ RST logic detection"]
pub struct PREAMBLE_RST_EN_R(crate::FieldReader<bool, bool>);
impl PREAMBLE_RST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREAMBLE_RST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_RST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREAMBLE_RST_EN` writer - This register is used to enable/disdable 16-bit preamble detection for RST detection. 0: RST Detection: RST logic detection 1: RST detection: Preamble(16-bit)+ RST logic detection"]
pub struct PREAMBLE_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_RST_EN_W<'a> {
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
#[doc = "Host Mode: F/W can enable SOP, SOP�, SOP� and Hard Reset Detection. Device Mode: F/W should enable only SOP and Hard Reset Detection. Cable Mode: Either SOP� or SOP� based on VCONN, Hard Reset and Cable Reset should be enabled.\n\nValue on reset: 97"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SOP_RST_EN_A {
    #[doc = "1: `1`"]
    EN_DEFAULT_SOP_DET = 1,
    #[doc = "2: `10`"]
    EN_PRIME_SOP_DET = 2,
    #[doc = "4: `100`"]
    EN_DBL_PRIME_SOP_DET = 4,
    #[doc = "8: `1000`"]
    EN_DEBUG_PRIME_SOP_DET = 8,
    #[doc = "16: `10000`"]
    EN_DEBUG_DBL_PRIME_DET = 16,
    #[doc = "32: `100000`"]
    EN_RX_CABLE_RESET_DET = 32,
    #[doc = "64: `1000000`"]
    EN_RX_HARD_RESET_DET = 64,
    #[doc = "128: `10000000`"]
    RESERVED1_CFG = 128,
    #[doc = "256: `100000000`"]
    EN_RESERVED1 = 256,
    #[doc = "512: `1000000000`"]
    RESERVED2_CFG = 512,
    #[doc = "1024: `10000000000`"]
    EN_RESERVED2 = 1024,
}
impl From<SOP_RST_EN_A> for u16 {
    #[inline(always)]
    fn from(variant: SOP_RST_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SOP_RST_EN` reader - Host Mode: F/W can enable SOP, SOP�, SOP� and Hard Reset Detection. Device Mode: F/W should enable only SOP and Hard Reset Detection. Cable Mode: Either SOP� or SOP� based on VCONN, Hard Reset and Cable Reset should be enabled."]
pub struct SOP_RST_EN_R(crate::FieldReader<u16, SOP_RST_EN_A>);
impl SOP_RST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SOP_RST_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOP_RST_EN_A> {
        match self.bits {
            1 => Some(SOP_RST_EN_A::EN_DEFAULT_SOP_DET),
            2 => Some(SOP_RST_EN_A::EN_PRIME_SOP_DET),
            4 => Some(SOP_RST_EN_A::EN_DBL_PRIME_SOP_DET),
            8 => Some(SOP_RST_EN_A::EN_DEBUG_PRIME_SOP_DET),
            16 => Some(SOP_RST_EN_A::EN_DEBUG_DBL_PRIME_DET),
            32 => Some(SOP_RST_EN_A::EN_RX_CABLE_RESET_DET),
            64 => Some(SOP_RST_EN_A::EN_RX_HARD_RESET_DET),
            128 => Some(SOP_RST_EN_A::RESERVED1_CFG),
            256 => Some(SOP_RST_EN_A::EN_RESERVED1),
            512 => Some(SOP_RST_EN_A::RESERVED2_CFG),
            1024 => Some(SOP_RST_EN_A::EN_RESERVED2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EN_DEFAULT_SOP_DET`"]
    #[inline(always)]
    pub fn is_en_default_sop_det(&self) -> bool {
        **self == SOP_RST_EN_A::EN_DEFAULT_SOP_DET
    }
    #[doc = "Checks if the value of the field is `EN_PRIME_SOP_DET`"]
    #[inline(always)]
    pub fn is_en_prime_sop_det(&self) -> bool {
        **self == SOP_RST_EN_A::EN_PRIME_SOP_DET
    }
    #[doc = "Checks if the value of the field is `EN_DBL_PRIME_SOP_DET`"]
    #[inline(always)]
    pub fn is_en_dbl_prime_sop_det(&self) -> bool {
        **self == SOP_RST_EN_A::EN_DBL_PRIME_SOP_DET
    }
    #[doc = "Checks if the value of the field is `EN_DEBUG_PRIME_SOP_DET`"]
    #[inline(always)]
    pub fn is_en_debug_prime_sop_det(&self) -> bool {
        **self == SOP_RST_EN_A::EN_DEBUG_PRIME_SOP_DET
    }
    #[doc = "Checks if the value of the field is `EN_DEBUG_DBL_PRIME_DET`"]
    #[inline(always)]
    pub fn is_en_debug_dbl_prime_det(&self) -> bool {
        **self == SOP_RST_EN_A::EN_DEBUG_DBL_PRIME_DET
    }
    #[doc = "Checks if the value of the field is `EN_RX_CABLE_RESET_DET`"]
    #[inline(always)]
    pub fn is_en_rx_cable_reset_det(&self) -> bool {
        **self == SOP_RST_EN_A::EN_RX_CABLE_RESET_DET
    }
    #[doc = "Checks if the value of the field is `EN_RX_HARD_RESET_DET`"]
    #[inline(always)]
    pub fn is_en_rx_hard_reset_det(&self) -> bool {
        **self == SOP_RST_EN_A::EN_RX_HARD_RESET_DET
    }
    #[doc = "Checks if the value of the field is `RESERVED1_CFG`"]
    #[inline(always)]
    pub fn is_reserved1_cfg(&self) -> bool {
        **self == SOP_RST_EN_A::RESERVED1_CFG
    }
    #[doc = "Checks if the value of the field is `EN_RESERVED1`"]
    #[inline(always)]
    pub fn is_en_reserved1(&self) -> bool {
        **self == SOP_RST_EN_A::EN_RESERVED1
    }
    #[doc = "Checks if the value of the field is `RESERVED2_CFG`"]
    #[inline(always)]
    pub fn is_reserved2_cfg(&self) -> bool {
        **self == SOP_RST_EN_A::RESERVED2_CFG
    }
    #[doc = "Checks if the value of the field is `EN_RESERVED2`"]
    #[inline(always)]
    pub fn is_en_reserved2(&self) -> bool {
        **self == SOP_RST_EN_A::EN_RESERVED2
    }
}
impl core::ops::Deref for SOP_RST_EN_R {
    type Target = crate::FieldReader<u16, SOP_RST_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOP_RST_EN` writer - Host Mode: F/W can enable SOP, SOP�, SOP� and Hard Reset Detection. Device Mode: F/W should enable only SOP and Hard Reset Detection. Cable Mode: Either SOP� or SOP� based on VCONN, Hard Reset and Cable Reset should be enabled."]
pub struct SOP_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOP_RST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOP_RST_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn en_default_sop_det(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_DEFAULT_SOP_DET)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn en_prime_sop_det(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_PRIME_SOP_DET)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn en_dbl_prime_sop_det(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_DBL_PRIME_SOP_DET)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn en_debug_prime_sop_det(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_DEBUG_PRIME_SOP_DET)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn en_debug_dbl_prime_det(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_DEBUG_DBL_PRIME_DET)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn en_rx_cable_reset_det(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_RX_CABLE_RESET_DET)
    }
    #[doc = "`1000000`"]
    #[inline(always)]
    pub fn en_rx_hard_reset_det(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_RX_HARD_RESET_DET)
    }
    #[doc = "`10000000`"]
    #[inline(always)]
    pub fn reserved1_cfg(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::RESERVED1_CFG)
    }
    #[doc = "`100000000`"]
    #[inline(always)]
    pub fn en_reserved1(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_RESERVED1)
    }
    #[doc = "`1000000000`"]
    #[inline(always)]
    pub fn reserved2_cfg(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::RESERVED2_CFG)
    }
    #[doc = "`10000000000`"]
    #[inline(always)]
    pub fn en_reserved2(self) -> &'a mut W {
        self.variant(SOP_RST_EN_A::EN_RESERVED2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 8)) | ((value as u32 & 0x07ff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This register is used for SOP, SOP',SOP'\", DEBUG SOP', DEBUG SOP\" and RX_RESERVED1/2_ORDER_SET(if configured for SOP) oder set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
    #[inline(always)]
    pub fn sop_cmp_opt(&self) -> SOP_CMP_OPT_R {
        SOP_CMP_OPT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This register is used for Cable RST, Hard RST and RX_RESERVED1/2_ORDER_SET(if configure for RST) order set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
    #[inline(always)]
    pub fn rst_cmp_opt(&self) -> RST_CMP_OPT_R {
        RST_CMP_OPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This register is used to enable/disdable 16-bit preamble detection for SOP detection. 0: SOP Detection: SOP logic detection 1: SOP detection: Preamble(16-bit)+ SOP logic detection"]
    #[inline(always)]
    pub fn preamble_sop_en(&self) -> PREAMBLE_SOP_EN_R {
        PREAMBLE_SOP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This register is used to enable/disdable 16-bit preamble detection for RST detection. 0: RST Detection: RST logic detection 1: RST detection: Preamble(16-bit)+ RST logic detection"]
    #[inline(always)]
    pub fn preamble_rst_en(&self) -> PREAMBLE_RST_EN_R {
        PREAMBLE_RST_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:18 - Host Mode: F/W can enable SOP, SOP�, SOP� and Hard Reset Detection. Device Mode: F/W should enable only SOP and Hard Reset Detection. Cable Mode: Either SOP� or SOP� based on VCONN, Hard Reset and Cable Reset should be enabled."]
    #[inline(always)]
    pub fn sop_rst_en(&self) -> SOP_RST_EN_R {
        SOP_RST_EN_R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used for SOP, SOP',SOP'\", DEBUG SOP', DEBUG SOP\" and RX_RESERVED1/2_ORDER_SET(if configured for SOP) oder set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
    #[inline(always)]
    pub fn sop_cmp_opt(&mut self) -> SOP_CMP_OPT_W {
        SOP_CMP_OPT_W { w: self }
    }
    #[doc = "Bit 1 - This register is used for Cable RST, Hard RST and RX_RESERVED1/2_ORDER_SET(if configure for RST) order set detection. It is recommended that CPU program this register to 1 ( 4 out of 4 option). 0: Compare 3 out of 4 order sets 1: Compare 4 out of 4 order sets"]
    #[inline(always)]
    pub fn rst_cmp_opt(&mut self) -> RST_CMP_OPT_W {
        RST_CMP_OPT_W { w: self }
    }
    #[doc = "Bit 2 - This register is used to enable/disdable 16-bit preamble detection for SOP detection. 0: SOP Detection: SOP logic detection 1: SOP detection: Preamble(16-bit)+ SOP logic detection"]
    #[inline(always)]
    pub fn preamble_sop_en(&mut self) -> PREAMBLE_SOP_EN_W {
        PREAMBLE_SOP_EN_W { w: self }
    }
    #[doc = "Bit 3 - This register is used to enable/disdable 16-bit preamble detection for RST detection. 0: RST Detection: RST logic detection 1: RST detection: Preamble(16-bit)+ RST logic detection"]
    #[inline(always)]
    pub fn preamble_rst_en(&mut self) -> PREAMBLE_RST_EN_W {
        PREAMBLE_RST_EN_W { w: self }
    }
    #[doc = "Bits 8:18 - Host Mode: F/W can enable SOP, SOP�, SOP� and Hard Reset Detection. Device Mode: F/W should enable only SOP and Hard Reset Detection. Cable Mode: Either SOP� or SOP� based on VCONN, Hard Reset and Cable Reset should be enabled."]
    #[inline(always)]
    pub fn sop_rst_en(&mut self) -> SOP_RST_EN_W {
        SOP_RST_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive SOPs and RSTs order set control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_order_set_ctrl](index.html) module"]
pub struct RX_ORDER_SET_CTRL_SPEC;
impl crate::RegisterSpec for RX_ORDER_SET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_order_set_ctrl::R](R) reader structure"]
impl crate::Readable for RX_ORDER_SET_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_order_set_ctrl::W](W) writer structure"]
impl crate::Writable for RX_ORDER_SET_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_ORDER_SET_CTRL to value 0x6103"]
impl crate::Resettable for RX_ORDER_SET_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6103
    }
}
