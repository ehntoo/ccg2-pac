#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_BUSY` reader - "]
pub struct RX_BUSY_R(crate::FieldReader<bool, bool>);
impl RX_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BUSY` reader - "]
pub struct TX_BUSY_R(crate::FieldReader<bool, bool>);
impl TX_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_DATA_VALID` reader - "]
pub struct CC_DATA_VALID_R(crate::FieldReader<bool, bool>);
impl CC_DATA_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_DATA_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_DATA_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOP_TYPE_DETECTED_A {
    #[doc = "0: `0`"]
    NO_SOP_RCVD = 0,
    #[doc = "1: `1`"]
    DEFAULT_SOP_RCVD = 1,
    #[doc = "2: `10`"]
    PRIME_SOP_RCVD = 2,
    #[doc = "3: `11`"]
    DBL_PRIME_SOP_RCVD = 3,
    #[doc = "4: `100`"]
    DBG_PRIME_SOP_RCVD = 4,
    #[doc = "5: `101`"]
    DBG_DBL_PRIME_SOP_RCVD = 5,
    #[doc = "6: `110`"]
    RSVD1_SOP_RCVD = 6,
    #[doc = "7: `111`"]
    RSVD2_SOP_RCVD = 7,
}
impl From<SOP_TYPE_DETECTED_A> for u8 {
    #[inline(always)]
    fn from(variant: SOP_TYPE_DETECTED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SOP_TYPE_DETECTED` reader - "]
pub struct SOP_TYPE_DETECTED_R(crate::FieldReader<u8, SOP_TYPE_DETECTED_A>);
impl SOP_TYPE_DETECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SOP_TYPE_DETECTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOP_TYPE_DETECTED_A {
        match self.bits {
            0 => SOP_TYPE_DETECTED_A::NO_SOP_RCVD,
            1 => SOP_TYPE_DETECTED_A::DEFAULT_SOP_RCVD,
            2 => SOP_TYPE_DETECTED_A::PRIME_SOP_RCVD,
            3 => SOP_TYPE_DETECTED_A::DBL_PRIME_SOP_RCVD,
            4 => SOP_TYPE_DETECTED_A::DBG_PRIME_SOP_RCVD,
            5 => SOP_TYPE_DETECTED_A::DBG_DBL_PRIME_SOP_RCVD,
            6 => SOP_TYPE_DETECTED_A::RSVD1_SOP_RCVD,
            7 => SOP_TYPE_DETECTED_A::RSVD2_SOP_RCVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_no_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::NO_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `DEFAULT_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_default_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::DEFAULT_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_prime_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `DBL_PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_dbl_prime_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::DBL_PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `DBG_PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_dbg_prime_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::DBG_PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `DBG_DBL_PRIME_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_dbg_dbl_prime_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::DBG_DBL_PRIME_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `RSVD1_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_rsvd1_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::RSVD1_SOP_RCVD
    }
    #[doc = "Checks if the value of the field is `RSVD2_SOP_RCVD`"]
    #[inline(always)]
    pub fn is_rsvd2_sop_rcvd(&self) -> bool {
        **self == SOP_TYPE_DETECTED_A::RSVD2_SOP_RCVD
    }
}
impl core::ops::Deref for SOP_TYPE_DETECTED_R {
    type Target = crate::FieldReader<u8, SOP_TYPE_DETECTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GOODCRC_MSG_SOP_TYPE_DETECTED_A {
    #[doc = "0: `0`"]
    NO_SOP_GOODCRC_RCVD = 0,
    #[doc = "1: `1`"]
    DEFAULT_SOP_GOODCRC_RCVD = 1,
    #[doc = "2: `10`"]
    PRIME_SOP_GOODCRC_RCVD = 2,
    #[doc = "3: `11`"]
    DBL_PRIME_SOP_GOODCRC_RCVD = 3,
    #[doc = "4: `100`"]
    DBG_PRIME_SOP_GOODCRC_RCVD = 4,
    #[doc = "5: `101`"]
    DBG_DBL_PRIME_SOP_GOODCRC_RCVD = 5,
    #[doc = "6: `110`"]
    RSVD1_SOP_GOODCRC_RCVD = 6,
    #[doc = "7: `111`"]
    RSVD2_SOP_GOODCRC_RCVD = 7,
}
impl From<GOODCRC_MSG_SOP_TYPE_DETECTED_A> for u8 {
    #[inline(always)]
    fn from(variant: GOODCRC_MSG_SOP_TYPE_DETECTED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GOODCRC_MSG_SOP_TYPE_DETECTED` reader - "]
pub struct GOODCRC_MSG_SOP_TYPE_DETECTED_R(crate::FieldReader<u8, GOODCRC_MSG_SOP_TYPE_DETECTED_A>);
impl GOODCRC_MSG_SOP_TYPE_DETECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GOODCRC_MSG_SOP_TYPE_DETECTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GOODCRC_MSG_SOP_TYPE_DETECTED_A {
        match self.bits {
            0 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::NO_SOP_GOODCRC_RCVD,
            1 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::DEFAULT_SOP_GOODCRC_RCVD,
            2 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::PRIME_SOP_GOODCRC_RCVD,
            3 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::DBL_PRIME_SOP_GOODCRC_RCVD,
            4 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::DBG_PRIME_SOP_GOODCRC_RCVD,
            5 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::DBG_DBL_PRIME_SOP_GOODCRC_RCVD,
            6 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::RSVD1_SOP_GOODCRC_RCVD,
            7 => GOODCRC_MSG_SOP_TYPE_DETECTED_A::RSVD2_SOP_GOODCRC_RCVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_no_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::NO_SOP_GOODCRC_RCVD
    }
    #[doc = "Checks if the value of the field is `DEFAULT_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_default_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::DEFAULT_SOP_GOODCRC_RCVD
    }
    #[doc = "Checks if the value of the field is `PRIME_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_prime_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::PRIME_SOP_GOODCRC_RCVD
    }
    #[doc = "Checks if the value of the field is `DBL_PRIME_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_dbl_prime_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::DBL_PRIME_SOP_GOODCRC_RCVD
    }
    #[doc = "Checks if the value of the field is `DBG_PRIME_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_dbg_prime_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::DBG_PRIME_SOP_GOODCRC_RCVD
    }
    #[doc = "Checks if the value of the field is `DBG_DBL_PRIME_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_dbg_dbl_prime_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::DBG_DBL_PRIME_SOP_GOODCRC_RCVD
    }
    #[doc = "Checks if the value of the field is `RSVD1_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_rsvd1_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::RSVD1_SOP_GOODCRC_RCVD
    }
    #[doc = "Checks if the value of the field is `RSVD2_SOP_GOODCRC_RCVD`"]
    #[inline(always)]
    pub fn is_rsvd2_sop_goodcrc_rcvd(&self) -> bool {
        **self == GOODCRC_MSG_SOP_TYPE_DETECTED_A::RSVD2_SOP_GOODCRC_RCVD
    }
}
impl core::ops::Deref for GOODCRC_MSG_SOP_TYPE_DETECTED_R {
    type Target = crate::FieldReader<u8, GOODCRC_MSG_SOP_TYPE_DETECTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RST_TYPE_DET_A {
    #[doc = "0: `0`"]
    RESERVED_RESET = 0,
    #[doc = "1: `1`"]
    HARD_RESET_RCVD = 1,
    #[doc = "2: `10`"]
    CABLE_RESET_RCVD = 2,
    #[doc = "3: `11`"]
    RSVD1_RESET_RCVD = 3,
    #[doc = "4: `100`"]
    RSVD2_RESET_RCVD = 4,
}
impl From<RST_TYPE_DET_A> for u8 {
    #[inline(always)]
    fn from(variant: RST_TYPE_DET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RST_TYPE_DET` reader - "]
pub struct RST_TYPE_DET_R(crate::FieldReader<u8, RST_TYPE_DET_A>);
impl RST_TYPE_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RST_TYPE_DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RST_TYPE_DET_A> {
        match self.bits {
            0 => Some(RST_TYPE_DET_A::RESERVED_RESET),
            1 => Some(RST_TYPE_DET_A::HARD_RESET_RCVD),
            2 => Some(RST_TYPE_DET_A::CABLE_RESET_RCVD),
            3 => Some(RST_TYPE_DET_A::RSVD1_RESET_RCVD),
            4 => Some(RST_TYPE_DET_A::RSVD2_RESET_RCVD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESERVED_RESET`"]
    #[inline(always)]
    pub fn is_reserved_reset(&self) -> bool {
        **self == RST_TYPE_DET_A::RESERVED_RESET
    }
    #[doc = "Checks if the value of the field is `HARD_RESET_RCVD`"]
    #[inline(always)]
    pub fn is_hard_reset_rcvd(&self) -> bool {
        **self == RST_TYPE_DET_A::HARD_RESET_RCVD
    }
    #[doc = "Checks if the value of the field is `CABLE_RESET_RCVD`"]
    #[inline(always)]
    pub fn is_cable_reset_rcvd(&self) -> bool {
        **self == RST_TYPE_DET_A::CABLE_RESET_RCVD
    }
    #[doc = "Checks if the value of the field is `RSVD1_RESET_RCVD`"]
    #[inline(always)]
    pub fn is_rsvd1_reset_rcvd(&self) -> bool {
        **self == RST_TYPE_DET_A::RSVD1_RESET_RCVD
    }
    #[doc = "Checks if the value of the field is `RSVD2_RESET_RCVD`"]
    #[inline(always)]
    pub fn is_rsvd2_reset_rcvd(&self) -> bool {
        **self == RST_TYPE_DET_A::RSVD2_RESET_RCVD
    }
}
impl core::ops::Deref for RST_TYPE_DET_R {
    type Target = crate::FieldReader<u8, RST_TYPE_DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN1_STATUS` reader - "]
pub struct VCONN1_STATUS_R(crate::FieldReader<bool, bool>);
impl VCONN1_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN1_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN1_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN2_STATUS` reader - "]
pub struct VCONN2_STATUS_R(crate::FieldReader<bool, bool>);
impl VCONN2_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN2_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN2_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1_STATUS` reader - "]
pub struct CC1_STATUS_R(crate::FieldReader<bool, bool>);
impl CC1_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC1_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2_STATUS` reader - "]
pub struct CC2_STATUS_R(crate::FieldReader<bool, bool>);
impl CC2_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC2_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_LA_STATUS` reader - "]
pub struct VCMP_LA_STATUS_R(crate::FieldReader<bool, bool>);
impl VCMP_LA_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_LA_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_LA_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_UP_STATUS` reader - "]
pub struct VCMP_UP_STATUS_R(crate::FieldReader<bool, bool>);
impl VCMP_UP_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_UP_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_UP_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_DN_STATUS` reader - "]
pub struct VCMP_DN_STATUS_R(crate::FieldReader<bool, bool>);
impl VCMP_DN_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_DN_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_DN_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOISE_DETECTED` reader - "]
pub struct NOISE_DETECTED_R(crate::FieldReader<bool, bool>);
impl NOISE_DETECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOISE_DETECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOISE_DETECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOISE_CC_DATA_VALID` reader - "]
pub struct NOISE_CC_DATA_VALID_R(crate::FieldReader<bool, bool>);
impl NOISE_CC_DATA_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOISE_CC_DATA_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOISE_CC_DATA_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_RX_DATA` reader - "]
pub struct CC_RX_DATA_R(crate::FieldReader<bool, bool>);
impl CC_RX_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_RX_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_RX_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cc_data_valid(&self) -> CC_DATA_VALID_R {
        CC_DATA_VALID_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn sop_type_detected(&self) -> SOP_TYPE_DETECTED_R {
        SOP_TYPE_DETECTED_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn goodcrc_msg_sop_type_detected(&self) -> GOODCRC_MSG_SOP_TYPE_DETECTED_R {
        GOODCRC_MSG_SOP_TYPE_DETECTED_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rst_type_det(&self) -> RST_TYPE_DET_R {
        RST_TYPE_DET_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn vconn1_status(&self) -> VCONN1_STATUS_R {
        VCONN1_STATUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn vconn2_status(&self) -> VCONN2_STATUS_R {
        VCONN2_STATUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cc1_status(&self) -> CC1_STATUS_R {
        CC1_STATUS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cc2_status(&self) -> CC2_STATUS_R {
        CC2_STATUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn vcmp_la_status(&self) -> VCMP_LA_STATUS_R {
        VCMP_LA_STATUS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn vcmp_up_status(&self) -> VCMP_UP_STATUS_R {
        VCMP_UP_STATUS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn vcmp_dn_status(&self) -> VCMP_DN_STATUS_R {
        VCMP_DN_STATUS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn noise_detected(&self) -> NOISE_DETECTED_R {
        NOISE_DETECTED_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn noise_cc_data_valid(&self) -> NOISE_CC_DATA_VALID_R {
        NOISE_CC_DATA_VALID_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cc_rx_data(&self) -> CC_RX_DATA_R {
        CC_RX_DATA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
