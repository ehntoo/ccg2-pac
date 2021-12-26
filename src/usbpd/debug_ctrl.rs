#[doc = "Register `DEBUG_CTRL` reader"]
pub struct R(crate::R<DEBUG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_CTRL` writer"]
pub struct W(crate::W<DEBUG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CTRL_SPEC>;
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
impl From<crate::W<DEBUG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_RX` reader - "]
pub struct RESET_RX_R(crate::FieldReader<bool, bool>);
impl RESET_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_RX` writer - "]
pub struct RESET_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_RX_W<'a> {
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
pub enum RX_MSG_CAL_STATE_A {
    #[doc = "0: `0`"]
    MSGSM_IDLE = 0,
    #[doc = "1: `1`"]
    MSGSM_PRE_HDR = 1,
    #[doc = "2: `10`"]
    MSGSM_HDR = 2,
    #[doc = "3: `11`"]
    MSGSM_DATA = 3,
    #[doc = "4: `100`"]
    MSGSM_CRC = 4,
    #[doc = "5: `101`"]
    MSGSM_EOP = 5,
    #[doc = "6: `110`"]
    MSGSM_STAT = 6,
}
impl From<RX_MSG_CAL_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_MSG_CAL_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_MSG_CAL_STATE` reader - "]
pub struct RX_MSG_CAL_STATE_R(crate::FieldReader<u8, RX_MSG_CAL_STATE_A>);
impl RX_MSG_CAL_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_MSG_CAL_STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_MSG_CAL_STATE_A> {
        match self.bits {
            0 => Some(RX_MSG_CAL_STATE_A::MSGSM_IDLE),
            1 => Some(RX_MSG_CAL_STATE_A::MSGSM_PRE_HDR),
            2 => Some(RX_MSG_CAL_STATE_A::MSGSM_HDR),
            3 => Some(RX_MSG_CAL_STATE_A::MSGSM_DATA),
            4 => Some(RX_MSG_CAL_STATE_A::MSGSM_CRC),
            5 => Some(RX_MSG_CAL_STATE_A::MSGSM_EOP),
            6 => Some(RX_MSG_CAL_STATE_A::MSGSM_STAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MSGSM_IDLE`"]
    #[inline(always)]
    pub fn is_msgsm_idle(&self) -> bool {
        **self == RX_MSG_CAL_STATE_A::MSGSM_IDLE
    }
    #[doc = "Checks if the value of the field is `MSGSM_PRE_HDR`"]
    #[inline(always)]
    pub fn is_msgsm_pre_hdr(&self) -> bool {
        **self == RX_MSG_CAL_STATE_A::MSGSM_PRE_HDR
    }
    #[doc = "Checks if the value of the field is `MSGSM_HDR`"]
    #[inline(always)]
    pub fn is_msgsm_hdr(&self) -> bool {
        **self == RX_MSG_CAL_STATE_A::MSGSM_HDR
    }
    #[doc = "Checks if the value of the field is `MSGSM_DATA`"]
    #[inline(always)]
    pub fn is_msgsm_data(&self) -> bool {
        **self == RX_MSG_CAL_STATE_A::MSGSM_DATA
    }
    #[doc = "Checks if the value of the field is `MSGSM_CRC`"]
    #[inline(always)]
    pub fn is_msgsm_crc(&self) -> bool {
        **self == RX_MSG_CAL_STATE_A::MSGSM_CRC
    }
    #[doc = "Checks if the value of the field is `MSGSM_EOP`"]
    #[inline(always)]
    pub fn is_msgsm_eop(&self) -> bool {
        **self == RX_MSG_CAL_STATE_A::MSGSM_EOP
    }
    #[doc = "Checks if the value of the field is `MSGSM_STAT`"]
    #[inline(always)]
    pub fn is_msgsm_stat(&self) -> bool {
        **self == RX_MSG_CAL_STATE_A::MSGSM_STAT
    }
}
impl core::ops::Deref for RX_MSG_CAL_STATE_R {
    type Target = crate::FieldReader<u8, RX_MSG_CAL_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_TX` reader - "]
pub struct RESET_TX_R(crate::FieldReader<bool, bool>);
impl RESET_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_TX` writer - "]
pub struct RESET_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_TX_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_MSG_STATE_A {
    #[doc = "0: `0`"]
    MSG_IDLE = 0,
    #[doc = "1: `1`"]
    MSG_CAL = 1,
    #[doc = "2: `10`"]
    MSG_SYN = 2,
    #[doc = "3: `11`"]
    MSG_HDR = 3,
    #[doc = "4: `100`"]
    MSG_DOBJ = 4,
    #[doc = "5: `101`"]
    MSG_CRC = 5,
    #[doc = "6: `110`"]
    MSG_EOP = 6,
    #[doc = "7: `111`"]
    MSG_RST = 7,
    #[doc = "8: `1000`"]
    MSG_BIDLE = 8,
}
impl From<TX_MSG_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_MSG_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX_MSG_STATE` reader - "]
pub struct TX_MSG_STATE_R(crate::FieldReader<u8, TX_MSG_STATE_A>);
impl TX_MSG_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_MSG_STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_MSG_STATE_A> {
        match self.bits {
            0 => Some(TX_MSG_STATE_A::MSG_IDLE),
            1 => Some(TX_MSG_STATE_A::MSG_CAL),
            2 => Some(TX_MSG_STATE_A::MSG_SYN),
            3 => Some(TX_MSG_STATE_A::MSG_HDR),
            4 => Some(TX_MSG_STATE_A::MSG_DOBJ),
            5 => Some(TX_MSG_STATE_A::MSG_CRC),
            6 => Some(TX_MSG_STATE_A::MSG_EOP),
            7 => Some(TX_MSG_STATE_A::MSG_RST),
            8 => Some(TX_MSG_STATE_A::MSG_BIDLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MSG_IDLE`"]
    #[inline(always)]
    pub fn is_msg_idle(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_IDLE
    }
    #[doc = "Checks if the value of the field is `MSG_CAL`"]
    #[inline(always)]
    pub fn is_msg_cal(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_CAL
    }
    #[doc = "Checks if the value of the field is `MSG_SYN`"]
    #[inline(always)]
    pub fn is_msg_syn(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_SYN
    }
    #[doc = "Checks if the value of the field is `MSG_HDR`"]
    #[inline(always)]
    pub fn is_msg_hdr(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_HDR
    }
    #[doc = "Checks if the value of the field is `MSG_DOBJ`"]
    #[inline(always)]
    pub fn is_msg_dobj(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_DOBJ
    }
    #[doc = "Checks if the value of the field is `MSG_CRC`"]
    #[inline(always)]
    pub fn is_msg_crc(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_CRC
    }
    #[doc = "Checks if the value of the field is `MSG_EOP`"]
    #[inline(always)]
    pub fn is_msg_eop(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_EOP
    }
    #[doc = "Checks if the value of the field is `MSG_RST`"]
    #[inline(always)]
    pub fn is_msg_rst(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_RST
    }
    #[doc = "Checks if the value of the field is `MSG_BIDLE`"]
    #[inline(always)]
    pub fn is_msg_bidle(&self) -> bool {
        **self == TX_MSG_STATE_A::MSG_BIDLE
    }
}
impl core::ops::Deref for TX_MSG_STATE_R {
    type Target = crate::FieldReader<u8, TX_MSG_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_SRC_SEL_STATE_A {
    #[doc = "0: `0`"]
    IDLE_SEL = 0,
    #[doc = "1: `1`"]
    CAL_SEL = 1,
    #[doc = "2: `10`"]
    SYN_SEL = 2,
    #[doc = "3: `11`"]
    KCHAR_SEL = 3,
    #[doc = "4: `100`"]
    EOP_SEL = 4,
    #[doc = "5: `101`"]
    RST_SEL = 5,
}
impl From<TX_SRC_SEL_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_SRC_SEL_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX_SRC_SEL_STATE` reader - "]
pub struct TX_SRC_SEL_STATE_R(crate::FieldReader<u8, TX_SRC_SEL_STATE_A>);
impl TX_SRC_SEL_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_SRC_SEL_STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_SRC_SEL_STATE_A> {
        match self.bits {
            0 => Some(TX_SRC_SEL_STATE_A::IDLE_SEL),
            1 => Some(TX_SRC_SEL_STATE_A::CAL_SEL),
            2 => Some(TX_SRC_SEL_STATE_A::SYN_SEL),
            3 => Some(TX_SRC_SEL_STATE_A::KCHAR_SEL),
            4 => Some(TX_SRC_SEL_STATE_A::EOP_SEL),
            5 => Some(TX_SRC_SEL_STATE_A::RST_SEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_SEL`"]
    #[inline(always)]
    pub fn is_idle_sel(&self) -> bool {
        **self == TX_SRC_SEL_STATE_A::IDLE_SEL
    }
    #[doc = "Checks if the value of the field is `CAL_SEL`"]
    #[inline(always)]
    pub fn is_cal_sel(&self) -> bool {
        **self == TX_SRC_SEL_STATE_A::CAL_SEL
    }
    #[doc = "Checks if the value of the field is `SYN_SEL`"]
    #[inline(always)]
    pub fn is_syn_sel(&self) -> bool {
        **self == TX_SRC_SEL_STATE_A::SYN_SEL
    }
    #[doc = "Checks if the value of the field is `KCHAR_SEL`"]
    #[inline(always)]
    pub fn is_kchar_sel(&self) -> bool {
        **self == TX_SRC_SEL_STATE_A::KCHAR_SEL
    }
    #[doc = "Checks if the value of the field is `EOP_SEL`"]
    #[inline(always)]
    pub fn is_eop_sel(&self) -> bool {
        **self == TX_SRC_SEL_STATE_A::EOP_SEL
    }
    #[doc = "Checks if the value of the field is `RST_SEL`"]
    #[inline(always)]
    pub fn is_rst_sel(&self) -> bool {
        **self == TX_SRC_SEL_STATE_A::RST_SEL
    }
}
impl core::ops::Deref for TX_SRC_SEL_STATE_R {
    type Target = crate::FieldReader<u8, TX_SRC_SEL_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PREAMBLE_CNT` reader - "]
pub struct TX_PREAMBLE_CNT_R(crate::FieldReader<u8, u8>);
impl TX_PREAMBLE_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PREAMBLE_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PREAMBLE_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PREAMBLE_CNT` writer - "]
pub struct TX_PREAMBLE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PREAMBLE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `EOP_VALUE` reader - "]
pub struct EOP_VALUE_R(crate::FieldReader<u8, u8>);
impl EOP_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EOP_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOP_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOP_VALUE` writer - "]
pub struct EOP_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | ((value as u32 & 0x1f) << 22);
        self.w
    }
}
#[doc = "Field `QUALIFY_RX_VALID_WITH_NOISE` reader - "]
pub struct QUALIFY_RX_VALID_WITH_NOISE_R(crate::FieldReader<bool, bool>);
impl QUALIFY_RX_VALID_WITH_NOISE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUALIFY_RX_VALID_WITH_NOISE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUALIFY_RX_VALID_WITH_NOISE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUALIFY_RX_VALID_WITH_NOISE` writer - "]
pub struct QUALIFY_RX_VALID_WITH_NOISE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUALIFY_RX_VALID_WITH_NOISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_rx(&self) -> RESET_RX_R {
        RESET_RX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn rx_msg_cal_state(&self) -> RX_MSG_CAL_STATE_R {
        RX_MSG_CAL_STATE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reset_tx(&self) -> RESET_TX_R {
        RESET_TX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn tx_msg_state(&self) -> TX_MSG_STATE_R {
        TX_MSG_STATE_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn tx_src_sel_state(&self) -> TX_SRC_SEL_STATE_R {
        TX_SRC_SEL_STATE_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tx_preamble_cnt(&self) -> TX_PREAMBLE_CNT_R {
        TX_PREAMBLE_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn eop_value(&self) -> EOP_VALUE_R {
        EOP_VALUE_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn qualify_rx_valid_with_noise(&self) -> QUALIFY_RX_VALID_WITH_NOISE_R {
        QUALIFY_RX_VALID_WITH_NOISE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_rx(&mut self) -> RESET_RX_W {
        RESET_RX_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reset_tx(&mut self) -> RESET_TX_W {
        RESET_TX_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tx_preamble_cnt(&mut self) -> TX_PREAMBLE_CNT_W {
        TX_PREAMBLE_CNT_W { w: self }
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn eop_value(&mut self) -> EOP_VALUE_W {
        EOP_VALUE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn qualify_rx_valid_with_noise(&mut self) -> QUALIFY_RX_VALID_WITH_NOISE_W {
        QUALIFY_RX_VALID_WITH_NOISE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_ctrl](index.html) module"]
pub struct DEBUG_CTRL_SPEC;
impl crate::RegisterSpec for DEBUG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_ctrl::R](R) reader structure"]
impl crate::Readable for DEBUG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_ctrl::W](W) writer structure"]
impl crate::Writable for DEBUG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_CTRL to value 0x037f_0000"]
impl crate::Resettable for DEBUG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x037f_0000
    }
}
