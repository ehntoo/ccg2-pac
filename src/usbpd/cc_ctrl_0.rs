#[doc = "Register `CC_CTRL_0` reader"]
pub struct R(crate::R<CC_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC_CTRL_0` writer"]
pub struct W(crate::W<CC_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_CTRL_0_SPEC>;
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
impl From<crate::W<CC_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_EN` reader - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
pub struct TX_EN_R(crate::FieldReader<bool, bool>);
impl TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_EN` writer - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
pub struct TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EN_W<'a> {
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
#[doc = "Field `RX_EN` reader - Enables the Transceiver to receive data, Active High This bit should be set after CC Line active interrupt (wakeup) in DeepSleep. FW should set this bit at init and not change after across deep sleep and wake."]
pub struct RX_EN_R(crate::FieldReader<bool, bool>);
impl RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EN` writer - Enables the Transceiver to receive data, Active High This bit should be set after CC Line active interrupt (wakeup) in DeepSleep. FW should set this bit at init and not change after across deep sleep and wake."]
pub struct RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_W<'a> {
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
#[doc = "Field `CC_1V2` reader - Firmware detects cable attach and specifies whether CC1 or CC2 is connected to the CC-line of the cable. 0 - CC1 1 - CC2"]
pub struct CC_1V2_R(crate::FieldReader<bool, bool>);
impl CC_1V2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_1V2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_1V2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_1V2` writer - Firmware detects cable attach and specifies whether CC1 or CC2 is connected to the CC-line of the cable. 0 - CC1 1 - CC2"]
pub struct CC_1V2_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_1V2_W<'a> {
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
#[doc = "Field `CMP_EN` reader - CC line voltage Comparator enable"]
pub struct CMP_EN_R(crate::FieldReader<bool, bool>);
impl CMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_EN` writer - CC line voltage Comparator enable"]
pub struct CMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EN_W<'a> {
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
#[doc = "Field `CMP_DN_CC1V2` reader - Connects cmp_dn comparator to CC1/CC2 0 - CC1 1 - CC2"]
pub struct CMP_DN_CC1V2_R(crate::FieldReader<bool, bool>);
impl CMP_DN_CC1V2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_DN_CC1V2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_DN_CC1V2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_DN_CC1V2` writer - Connects cmp_dn comparator to CC1/CC2 0 - CC1 1 - CC2"]
pub struct CMP_DN_CC1V2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_DN_CC1V2_W<'a> {
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
#[doc = "Selects the voltage threshold for cmp_dn comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_DN_VSEL_A {
    #[doc = "0: `0`"]
    SEL_0_2_VOL = 0,
    #[doc = "1: `1`"]
    SEL_0_4_VOL = 1,
    #[doc = "2: `10`"]
    SEL_0_55_VOL = 2,
    #[doc = "3: `11`"]
    SEL_0_655_VOL = 3,
    #[doc = "4: `100`"]
    SEL_0_8_VOL = 4,
    #[doc = "5: `101`"]
    SEL_1_235_VOL = 5,
    #[doc = "6: `110`"]
    SEL_1_575_VOL = 6,
    #[doc = "7: `111`"]
    SEL_2_6_VOL = 7,
}
impl From<CMP_DN_VSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_DN_VSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_DN_VSEL` reader - Selects the voltage threshold for cmp_dn comparator"]
pub struct CMP_DN_VSEL_R(crate::FieldReader<u8, CMP_DN_VSEL_A>);
impl CMP_DN_VSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_DN_VSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_DN_VSEL_A {
        match self.bits {
            0 => CMP_DN_VSEL_A::SEL_0_2_VOL,
            1 => CMP_DN_VSEL_A::SEL_0_4_VOL,
            2 => CMP_DN_VSEL_A::SEL_0_55_VOL,
            3 => CMP_DN_VSEL_A::SEL_0_655_VOL,
            4 => CMP_DN_VSEL_A::SEL_0_8_VOL,
            5 => CMP_DN_VSEL_A::SEL_1_235_VOL,
            6 => CMP_DN_VSEL_A::SEL_1_575_VOL,
            7 => CMP_DN_VSEL_A::SEL_2_6_VOL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL_0_2_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_2_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_0_2_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_4_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_4_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_0_4_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_55_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_55_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_0_55_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_655_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_655_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_0_655_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_8_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_8_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_0_8_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_1_235_VOL`"]
    #[inline(always)]
    pub fn is_sel_1_235_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_1_235_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_1_575_VOL`"]
    #[inline(always)]
    pub fn is_sel_1_575_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_1_575_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_2_6_VOL`"]
    #[inline(always)]
    pub fn is_sel_2_6_vol(&self) -> bool {
        **self == CMP_DN_VSEL_A::SEL_2_6_VOL
    }
}
impl core::ops::Deref for CMP_DN_VSEL_R {
    type Target = crate::FieldReader<u8, CMP_DN_VSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_DN_VSEL` writer - Selects the voltage threshold for cmp_dn comparator"]
pub struct CMP_DN_VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_DN_VSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_DN_VSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_0_2_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_0_2_VOL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_0_4_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_0_4_VOL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_0_55_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_0_55_VOL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_0_655_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_0_655_VOL)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_0_8_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_0_8_VOL)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_1_235_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_1_235_VOL)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_1_575_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_1_575_VOL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_2_6_vol(self) -> &'a mut W {
        self.variant(CMP_DN_VSEL_A::SEL_2_6_VOL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `CMP_UP_CC1V2` reader - Connects cmp_up comparator to CC1/CC2 0 - CC1 1 - CC2"]
pub struct CMP_UP_CC1V2_R(crate::FieldReader<bool, bool>);
impl CMP_UP_CC1V2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_UP_CC1V2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_UP_CC1V2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_UP_CC1V2` writer - Connects cmp_up comparator to CC1/CC2 0 - CC1 1 - CC2"]
pub struct CMP_UP_CC1V2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_UP_CC1V2_W<'a> {
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
#[doc = "Selects the voltage threshold for cmp_up comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_UP_VSEL_A {
    #[doc = "0: `0`"]
    SEL_0_2_VOL = 0,
    #[doc = "1: `1`"]
    SEL_0_4_VOL = 1,
    #[doc = "2: `10`"]
    SEL_0_3_VOL = 2,
    #[doc = "3: `11`"]
    SEL_0_655_VOL = 3,
    #[doc = "4: `100`"]
    SEL_0_8_VOL = 4,
    #[doc = "5: `101`"]
    SEL_1_235_VOL = 5,
    #[doc = "6: `110`"]
    SEL_1_575_VOL = 6,
    #[doc = "7: `111`"]
    SEL_2_6_VOL = 7,
}
impl From<CMP_UP_VSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_UP_VSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_UP_VSEL` reader - Selects the voltage threshold for cmp_up comparator"]
pub struct CMP_UP_VSEL_R(crate::FieldReader<u8, CMP_UP_VSEL_A>);
impl CMP_UP_VSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_UP_VSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_UP_VSEL_A {
        match self.bits {
            0 => CMP_UP_VSEL_A::SEL_0_2_VOL,
            1 => CMP_UP_VSEL_A::SEL_0_4_VOL,
            2 => CMP_UP_VSEL_A::SEL_0_3_VOL,
            3 => CMP_UP_VSEL_A::SEL_0_655_VOL,
            4 => CMP_UP_VSEL_A::SEL_0_8_VOL,
            5 => CMP_UP_VSEL_A::SEL_1_235_VOL,
            6 => CMP_UP_VSEL_A::SEL_1_575_VOL,
            7 => CMP_UP_VSEL_A::SEL_2_6_VOL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL_0_2_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_2_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_0_2_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_4_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_4_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_0_4_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_3_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_3_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_0_3_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_655_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_655_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_0_655_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_8_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_8_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_0_8_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_1_235_VOL`"]
    #[inline(always)]
    pub fn is_sel_1_235_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_1_235_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_1_575_VOL`"]
    #[inline(always)]
    pub fn is_sel_1_575_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_1_575_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_2_6_VOL`"]
    #[inline(always)]
    pub fn is_sel_2_6_vol(&self) -> bool {
        **self == CMP_UP_VSEL_A::SEL_2_6_VOL
    }
}
impl core::ops::Deref for CMP_UP_VSEL_R {
    type Target = crate::FieldReader<u8, CMP_UP_VSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_UP_VSEL` writer - Selects the voltage threshold for cmp_up comparator"]
pub struct CMP_UP_VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_UP_VSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_UP_VSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_0_2_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_0_2_VOL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_0_4_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_0_4_VOL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_0_3_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_0_3_VOL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_0_655_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_0_655_VOL)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_0_8_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_0_8_VOL)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_1_235_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_1_235_VOL)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_1_575_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_1_575_VOL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_2_6_vol(self) -> &'a mut W {
        self.variant(CMP_UP_VSEL_A::SEL_2_6_VOL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Selects the cmp_up comparator offset:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_UP_OFFSET_A {
    #[doc = "0: `0`"]
    SEL_MINUS_50_MV = 0,
    #[doc = "1: `1`"]
    SEL_MINUS_100_MV = 1,
    #[doc = "2: `10`"]
    SEL_MINUS_150_MV = 2,
    #[doc = "3: `11`"]
    SEL_MINUS_200_MV = 3,
    #[doc = "4: `100`"]
    SEL_PLUS_50_MV = 4,
    #[doc = "5: `101`"]
    SEL_PLUS_100_MV = 5,
    #[doc = "6: `110`"]
    SEL_PLUS_150_MV = 6,
    #[doc = "7: `111`"]
    SEL_PLUS_200_MV = 7,
}
impl From<CMP_UP_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_UP_OFFSET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_UP_OFFSET` reader - Selects the cmp_up comparator offset:"]
pub struct CMP_UP_OFFSET_R(crate::FieldReader<u8, CMP_UP_OFFSET_A>);
impl CMP_UP_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_UP_OFFSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_UP_OFFSET_A {
        match self.bits {
            0 => CMP_UP_OFFSET_A::SEL_MINUS_50_MV,
            1 => CMP_UP_OFFSET_A::SEL_MINUS_100_MV,
            2 => CMP_UP_OFFSET_A::SEL_MINUS_150_MV,
            3 => CMP_UP_OFFSET_A::SEL_MINUS_200_MV,
            4 => CMP_UP_OFFSET_A::SEL_PLUS_50_MV,
            5 => CMP_UP_OFFSET_A::SEL_PLUS_100_MV,
            6 => CMP_UP_OFFSET_A::SEL_PLUS_150_MV,
            7 => CMP_UP_OFFSET_A::SEL_PLUS_200_MV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_50_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_50_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_MINUS_50_MV
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_100_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_100_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_MINUS_100_MV
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_150_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_150_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_MINUS_150_MV
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_200_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_200_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_MINUS_200_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_50_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_50_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_PLUS_50_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_100_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_100_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_PLUS_100_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_150_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_150_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_PLUS_150_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_200_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_200_mv(&self) -> bool {
        **self == CMP_UP_OFFSET_A::SEL_PLUS_200_MV
    }
}
impl core::ops::Deref for CMP_UP_OFFSET_R {
    type Target = crate::FieldReader<u8, CMP_UP_OFFSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_UP_OFFSET` writer - Selects the cmp_up comparator offset:"]
pub struct CMP_UP_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_UP_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_UP_OFFSET_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_minus_50_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_MINUS_50_MV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_minus_100_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_MINUS_100_MV)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_minus_150_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_MINUS_150_MV)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_minus_200_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_MINUS_200_MV)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_plus_50_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_PLUS_50_MV)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_plus_100_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_PLUS_100_MV)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_plus_150_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_PLUS_150_MV)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_plus_200_mv(self) -> &'a mut W {
        self.variant(CMP_UP_OFFSET_A::SEL_PLUS_200_MV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `CMP_UP_OFFSET_EN` reader - Enables the offset generator for the cmp_up comparator 0 - no offset 1 - offset enabled, see CMP_UP_OFFSET register for value"]
pub struct CMP_UP_OFFSET_EN_R(crate::FieldReader<bool, bool>);
impl CMP_UP_OFFSET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_UP_OFFSET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_UP_OFFSET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_UP_OFFSET_EN` writer - Enables the offset generator for the cmp_up comparator 0 - no offset 1 - offset enabled, see CMP_UP_OFFSET register for value"]
pub struct CMP_UP_OFFSET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_UP_OFFSET_EN_W<'a> {
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
#[doc = "Field `CMP_LA_CC1V2` reader - Connects cmp_la comparator to CC1/CC2 0 - CC1 1 - CC2"]
pub struct CMP_LA_CC1V2_R(crate::FieldReader<bool, bool>);
impl CMP_LA_CC1V2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_LA_CC1V2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LA_CC1V2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LA_CC1V2` writer - Connects cmp_la comparator to CC1/CC2 0 - CC1 1 - CC2"]
pub struct CMP_LA_CC1V2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LA_CC1V2_W<'a> {
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
#[doc = "Selects the voltage threshold for cmp_la comparator\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_LA_VSEL_A {
    #[doc = "0: `0`"]
    SEL_0_2_VOL = 0,
    #[doc = "1: `1`"]
    SEL_0_4_VOL = 1,
    #[doc = "2: `10`"]
    SEL_0_55_VOL = 2,
    #[doc = "3: `11`"]
    SEL_0_655_VOL = 3,
    #[doc = "4: `100`"]
    SEL_0_8_VOL = 4,
    #[doc = "5: `101`"]
    SEL_1_235_VOL = 5,
    #[doc = "6: `110`"]
    SEL_1_575_VOL = 6,
    #[doc = "7: `111`"]
    SEL_2_6_VOL = 7,
}
impl From<CMP_LA_VSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_LA_VSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_LA_VSEL` reader - Selects the voltage threshold for cmp_la comparator"]
pub struct CMP_LA_VSEL_R(crate::FieldReader<u8, CMP_LA_VSEL_A>);
impl CMP_LA_VSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_LA_VSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_LA_VSEL_A {
        match self.bits {
            0 => CMP_LA_VSEL_A::SEL_0_2_VOL,
            1 => CMP_LA_VSEL_A::SEL_0_4_VOL,
            2 => CMP_LA_VSEL_A::SEL_0_55_VOL,
            3 => CMP_LA_VSEL_A::SEL_0_655_VOL,
            4 => CMP_LA_VSEL_A::SEL_0_8_VOL,
            5 => CMP_LA_VSEL_A::SEL_1_235_VOL,
            6 => CMP_LA_VSEL_A::SEL_1_575_VOL,
            7 => CMP_LA_VSEL_A::SEL_2_6_VOL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL_0_2_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_2_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_0_2_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_4_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_4_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_0_4_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_55_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_55_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_0_55_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_655_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_655_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_0_655_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_0_8_VOL`"]
    #[inline(always)]
    pub fn is_sel_0_8_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_0_8_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_1_235_VOL`"]
    #[inline(always)]
    pub fn is_sel_1_235_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_1_235_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_1_575_VOL`"]
    #[inline(always)]
    pub fn is_sel_1_575_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_1_575_VOL
    }
    #[doc = "Checks if the value of the field is `SEL_2_6_VOL`"]
    #[inline(always)]
    pub fn is_sel_2_6_vol(&self) -> bool {
        **self == CMP_LA_VSEL_A::SEL_2_6_VOL
    }
}
impl core::ops::Deref for CMP_LA_VSEL_R {
    type Target = crate::FieldReader<u8, CMP_LA_VSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LA_VSEL` writer - Selects the voltage threshold for cmp_la comparator"]
pub struct CMP_LA_VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LA_VSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_LA_VSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_0_2_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_0_2_VOL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_0_4_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_0_4_VOL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_0_55_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_0_55_VOL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_0_655_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_0_655_VOL)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_0_8_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_0_8_VOL)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_1_235_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_1_235_VOL)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_1_575_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_1_575_VOL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_2_6_vol(self) -> &'a mut W {
        self.variant(CMP_LA_VSEL_A::SEL_2_6_VOL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `RD_CC1_DB_DIS` reader - Disable Dead Battery Rd termination on CC1"]
pub struct RD_CC1_DB_DIS_R(crate::FieldReader<bool, bool>);
impl RD_CC1_DB_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_CC1_DB_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_CC1_DB_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_CC1_DB_DIS` writer - Disable Dead Battery Rd termination on CC1"]
pub struct RD_CC1_DB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_CC1_DB_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RD_CC2_DB_DIS` reader - Disable Dead Battery Rd termination on CC2"]
pub struct RD_CC2_DB_DIS_R(crate::FieldReader<bool, bool>);
impl RD_CC2_DB_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_CC2_DB_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_CC2_DB_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_CC2_DB_DIS` writer - Disable Dead Battery Rd termination on CC2"]
pub struct RD_CC2_DB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_CC2_DB_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `RD_CC1_EN` reader - 0: Disable CC1 Trimmed Rd Termination 1: Enable CC1 Trimmed Rd Termination"]
pub struct RD_CC1_EN_R(crate::FieldReader<bool, bool>);
impl RD_CC1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_CC1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_CC1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_CC1_EN` writer - 0: Disable CC1 Trimmed Rd Termination 1: Enable CC1 Trimmed Rd Termination"]
pub struct RD_CC1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_CC1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RD_CC2_EN` reader - 0: Disable CC2 Trimmed Rd Termination 1: Enable CC2 Trimmed Rd Termination"]
pub struct RD_CC2_EN_R(crate::FieldReader<bool, bool>);
impl RD_CC2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_CC2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_CC2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_CC2_EN` writer - 0: Disable CC2 Trimmed Rd Termination 1: Enable CC2 Trimmed Rd Termination"]
pub struct RD_CC2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_CC2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RP_CC1_EN` reader - 0: Disable CC1 Pull-up Termination (Rp) 1: Enable CC1 Pull-up Termination"]
pub struct RP_CC1_EN_R(crate::FieldReader<bool, bool>);
impl RP_CC1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RP_CC1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RP_CC1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RP_CC1_EN` writer - 0: Disable CC1 Pull-up Termination (Rp) 1: Enable CC1 Pull-up Termination"]
pub struct RP_CC1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RP_CC1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RP_CC2_EN` reader - 0: Disable CC2 Pull-up Termination (Rp) 1: Enable CC2 Pull-up Termination"]
pub struct RP_CC2_EN_R(crate::FieldReader<bool, bool>);
impl RP_CC2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RP_CC2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RP_CC2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RP_CC2_EN` writer - 0: Disable CC2 Pull-up Termination (Rp) 1: Enable CC2 Pull-up Termination"]
pub struct RP_CC2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RP_CC2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Selects the Pull-up Termination current value 0, 2 - 80uA (Default current broadcast) 1 - 180uA (1.5A current broadcast) 3 - 330uA (3.0A current broadcast)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RP_MODE_A {
    #[doc = "0: `0`"]
    PU0_80_UA = 0,
    #[doc = "1: `1`"]
    PU1_1P5_A = 1,
    #[doc = "2: `10`"]
    PU2_80_UA = 2,
    #[doc = "3: `11`"]
    PU3_3A = 3,
}
impl From<RP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RP_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RP_MODE` reader - Selects the Pull-up Termination current value 0, 2 - 80uA (Default current broadcast) 1 - 180uA (1.5A current broadcast) 3 - 330uA (3.0A current broadcast)"]
pub struct RP_MODE_R(crate::FieldReader<u8, RP_MODE_A>);
impl RP_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RP_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_MODE_A {
        match self.bits {
            0 => RP_MODE_A::PU0_80_UA,
            1 => RP_MODE_A::PU1_1P5_A,
            2 => RP_MODE_A::PU2_80_UA,
            3 => RP_MODE_A::PU3_3A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PU0_80_UA`"]
    #[inline(always)]
    pub fn is_pu0_80_ua(&self) -> bool {
        **self == RP_MODE_A::PU0_80_UA
    }
    #[doc = "Checks if the value of the field is `PU1_1P5_A`"]
    #[inline(always)]
    pub fn is_pu1_1p5_a(&self) -> bool {
        **self == RP_MODE_A::PU1_1P5_A
    }
    #[doc = "Checks if the value of the field is `PU2_80_UA`"]
    #[inline(always)]
    pub fn is_pu2_80_ua(&self) -> bool {
        **self == RP_MODE_A::PU2_80_UA
    }
    #[doc = "Checks if the value of the field is `PU3_3A`"]
    #[inline(always)]
    pub fn is_pu3_3a(&self) -> bool {
        **self == RP_MODE_A::PU3_3A
    }
}
impl core::ops::Deref for RP_MODE_R {
    type Target = crate::FieldReader<u8, RP_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RP_MODE` writer - Selects the Pull-up Termination current value 0, 2 - 80uA (Default current broadcast) 1 - 180uA (1.5A current broadcast) 3 - 330uA (3.0A current broadcast)"]
pub struct RP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RP_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pu0_80_ua(self) -> &'a mut W {
        self.variant(RP_MODE_A::PU0_80_UA)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pu1_1p5_a(self) -> &'a mut W {
        self.variant(RP_MODE_A::PU1_1P5_A)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pu2_80_ua(self) -> &'a mut W {
        self.variant(RP_MODE_A::PU2_80_UA)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pu3_3a(self) -> &'a mut W {
        self.variant(RP_MODE_A::PU3_3A)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `EN_HYST` reader - Enables the hysteresis mode for the line activity comparator"]
pub struct EN_HYST_R(crate::FieldReader<bool, bool>);
impl EN_HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_HYST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_HYST` writer - Enables the hysteresis mode for the line activity comparator"]
pub struct EN_HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_HYST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `HYST_MODE` reader - Selects the amount of line activity comparator hysteresis 0: 50mV hystersis 1: 100mV hysteresis\""]
pub struct HYST_MODE_R(crate::FieldReader<bool, bool>);
impl HYST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HYST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST_MODE` writer - Selects the amount of line activity comparator hysteresis 0: 50mV hystersis 1: 100mV hysteresis\""]
pub struct HYST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DFP_EN` reader - Controls the reference voltage generator for DFP vs. UFP/Cable operation 0: UFP/Cable - voltage reference is 2.4V 1: DFP - voltage reference is 2.6V\" This register is a user configuration that must be set."]
pub struct DFP_EN_R(crate::FieldReader<bool, bool>);
impl DFP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFP_EN` writer - Controls the reference voltage generator for DFP vs. UFP/Cable operation 0: UFP/Cable - voltage reference is 2.4V 1: DFP - voltage reference is 2.6V\" This register is a user configuration that must be set."]
pub struct DFP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFP_EN_W<'a> {
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
#[doc = "Field `PWR_DISABLE` reader - Disables all active circuitry and DC paths DS_ATTACH_DET_EN is still active"]
pub struct PWR_DISABLE_R(crate::FieldReader<bool, bool>);
impl PWR_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_DISABLE` writer - Disables all active circuitry and DC paths DS_ATTACH_DET_EN is still active"]
pub struct PWR_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DISABLE_W<'a> {
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
    #[doc = "Bit 0 - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
    #[inline(always)]
    pub fn tx_en(&self) -> TX_EN_R {
        TX_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the Transceiver to receive data, Active High This bit should be set after CC Line active interrupt (wakeup) in DeepSleep. FW should set this bit at init and not change after across deep sleep and wake."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Firmware detects cable attach and specifies whether CC1 or CC2 is connected to the CC-line of the cable. 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cc_1v2(&self) -> CC_1V2_R {
        CC_1V2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CC line voltage Comparator enable"]
    #[inline(always)]
    pub fn cmp_en(&self) -> CMP_EN_R {
        CMP_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Connects cmp_dn comparator to CC1/CC2 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cmp_dn_cc1v2(&self) -> CMP_DN_CC1V2_R {
        CMP_DN_CC1V2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Selects the voltage threshold for cmp_dn comparator"]
    #[inline(always)]
    pub fn cmp_dn_vsel(&self) -> CMP_DN_VSEL_R {
        CMP_DN_VSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Connects cmp_up comparator to CC1/CC2 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cmp_up_cc1v2(&self) -> CMP_UP_CC1V2_R {
        CMP_UP_CC1V2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Selects the voltage threshold for cmp_up comparator"]
    #[inline(always)]
    pub fn cmp_up_vsel(&self) -> CMP_UP_VSEL_R {
        CMP_UP_VSEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Selects the cmp_up comparator offset:"]
    #[inline(always)]
    pub fn cmp_up_offset(&self) -> CMP_UP_OFFSET_R {
        CMP_UP_OFFSET_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Enables the offset generator for the cmp_up comparator 0 - no offset 1 - offset enabled, see CMP_UP_OFFSET register for value"]
    #[inline(always)]
    pub fn cmp_up_offset_en(&self) -> CMP_UP_OFFSET_EN_R {
        CMP_UP_OFFSET_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Connects cmp_la comparator to CC1/CC2 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cmp_la_cc1v2(&self) -> CMP_LA_CC1V2_R {
        CMP_LA_CC1V2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Selects the voltage threshold for cmp_la comparator"]
    #[inline(always)]
    pub fn cmp_la_vsel(&self) -> CMP_LA_VSEL_R {
        CMP_LA_VSEL_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Disable Dead Battery Rd termination on CC1"]
    #[inline(always)]
    pub fn rd_cc1_db_dis(&self) -> RD_CC1_DB_DIS_R {
        RD_CC1_DB_DIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Disable Dead Battery Rd termination on CC2"]
    #[inline(always)]
    pub fn rd_cc2_db_dis(&self) -> RD_CC2_DB_DIS_R {
        RD_CC2_DB_DIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 0: Disable CC1 Trimmed Rd Termination 1: Enable CC1 Trimmed Rd Termination"]
    #[inline(always)]
    pub fn rd_cc1_en(&self) -> RD_CC1_EN_R {
        RD_CC1_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 0: Disable CC2 Trimmed Rd Termination 1: Enable CC2 Trimmed Rd Termination"]
    #[inline(always)]
    pub fn rd_cc2_en(&self) -> RD_CC2_EN_R {
        RD_CC2_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 0: Disable CC1 Pull-up Termination (Rp) 1: Enable CC1 Pull-up Termination"]
    #[inline(always)]
    pub fn rp_cc1_en(&self) -> RP_CC1_EN_R {
        RP_CC1_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 0: Disable CC2 Pull-up Termination (Rp) 1: Enable CC2 Pull-up Termination"]
    #[inline(always)]
    pub fn rp_cc2_en(&self) -> RP_CC2_EN_R {
        RP_CC2_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - Selects the Pull-up Termination current value 0, 2 - 80uA (Default current broadcast) 1 - 180uA (1.5A current broadcast) 3 - 330uA (3.0A current broadcast)"]
    #[inline(always)]
    pub fn rp_mode(&self) -> RP_MODE_R {
        RP_MODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Enables the hysteresis mode for the line activity comparator"]
    #[inline(always)]
    pub fn en_hyst(&self) -> EN_HYST_R {
        EN_HYST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Selects the amount of line activity comparator hysteresis 0: 50mV hystersis 1: 100mV hysteresis\""]
    #[inline(always)]
    pub fn hyst_mode(&self) -> HYST_MODE_R {
        HYST_MODE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Controls the reference voltage generator for DFP vs. UFP/Cable operation 0: UFP/Cable - voltage reference is 2.4V 1: DFP - voltage reference is 2.6V\" This register is a user configuration that must be set."]
    #[inline(always)]
    pub fn dfp_en(&self) -> DFP_EN_R {
        DFP_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Disables all active circuitry and DC paths DS_ATTACH_DET_EN is still active"]
    #[inline(always)]
    pub fn pwr_disable(&self) -> PWR_DISABLE_R {
        PWR_DISABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FW can only use this bit when the DEBUG_CC_0.TX_CC_DRIVE_SRC is set to \"1\". 0: Disables the Transceiver to transmit data 1: Enables the Transceiver to transmit data"]
    #[inline(always)]
    pub fn tx_en(&mut self) -> TX_EN_W {
        TX_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enables the Transceiver to receive data, Active High This bit should be set after CC Line active interrupt (wakeup) in DeepSleep. FW should set this bit at init and not change after across deep sleep and wake."]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RX_EN_W {
        RX_EN_W { w: self }
    }
    #[doc = "Bit 2 - Firmware detects cable attach and specifies whether CC1 or CC2 is connected to the CC-line of the cable. 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cc_1v2(&mut self) -> CC_1V2_W {
        CC_1V2_W { w: self }
    }
    #[doc = "Bit 3 - CC line voltage Comparator enable"]
    #[inline(always)]
    pub fn cmp_en(&mut self) -> CMP_EN_W {
        CMP_EN_W { w: self }
    }
    #[doc = "Bit 4 - Connects cmp_dn comparator to CC1/CC2 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cmp_dn_cc1v2(&mut self) -> CMP_DN_CC1V2_W {
        CMP_DN_CC1V2_W { w: self }
    }
    #[doc = "Bits 5:7 - Selects the voltage threshold for cmp_dn comparator"]
    #[inline(always)]
    pub fn cmp_dn_vsel(&mut self) -> CMP_DN_VSEL_W {
        CMP_DN_VSEL_W { w: self }
    }
    #[doc = "Bit 8 - Connects cmp_up comparator to CC1/CC2 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cmp_up_cc1v2(&mut self) -> CMP_UP_CC1V2_W {
        CMP_UP_CC1V2_W { w: self }
    }
    #[doc = "Bits 9:11 - Selects the voltage threshold for cmp_up comparator"]
    #[inline(always)]
    pub fn cmp_up_vsel(&mut self) -> CMP_UP_VSEL_W {
        CMP_UP_VSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - Selects the cmp_up comparator offset:"]
    #[inline(always)]
    pub fn cmp_up_offset(&mut self) -> CMP_UP_OFFSET_W {
        CMP_UP_OFFSET_W { w: self }
    }
    #[doc = "Bit 15 - Enables the offset generator for the cmp_up comparator 0 - no offset 1 - offset enabled, see CMP_UP_OFFSET register for value"]
    #[inline(always)]
    pub fn cmp_up_offset_en(&mut self) -> CMP_UP_OFFSET_EN_W {
        CMP_UP_OFFSET_EN_W { w: self }
    }
    #[doc = "Bit 16 - Connects cmp_la comparator to CC1/CC2 0 - CC1 1 - CC2"]
    #[inline(always)]
    pub fn cmp_la_cc1v2(&mut self) -> CMP_LA_CC1V2_W {
        CMP_LA_CC1V2_W { w: self }
    }
    #[doc = "Bits 17:19 - Selects the voltage threshold for cmp_la comparator"]
    #[inline(always)]
    pub fn cmp_la_vsel(&mut self) -> CMP_LA_VSEL_W {
        CMP_LA_VSEL_W { w: self }
    }
    #[doc = "Bit 20 - Disable Dead Battery Rd termination on CC1"]
    #[inline(always)]
    pub fn rd_cc1_db_dis(&mut self) -> RD_CC1_DB_DIS_W {
        RD_CC1_DB_DIS_W { w: self }
    }
    #[doc = "Bit 21 - Disable Dead Battery Rd termination on CC2"]
    #[inline(always)]
    pub fn rd_cc2_db_dis(&mut self) -> RD_CC2_DB_DIS_W {
        RD_CC2_DB_DIS_W { w: self }
    }
    #[doc = "Bit 22 - 0: Disable CC1 Trimmed Rd Termination 1: Enable CC1 Trimmed Rd Termination"]
    #[inline(always)]
    pub fn rd_cc1_en(&mut self) -> RD_CC1_EN_W {
        RD_CC1_EN_W { w: self }
    }
    #[doc = "Bit 23 - 0: Disable CC2 Trimmed Rd Termination 1: Enable CC2 Trimmed Rd Termination"]
    #[inline(always)]
    pub fn rd_cc2_en(&mut self) -> RD_CC2_EN_W {
        RD_CC2_EN_W { w: self }
    }
    #[doc = "Bit 24 - 0: Disable CC1 Pull-up Termination (Rp) 1: Enable CC1 Pull-up Termination"]
    #[inline(always)]
    pub fn rp_cc1_en(&mut self) -> RP_CC1_EN_W {
        RP_CC1_EN_W { w: self }
    }
    #[doc = "Bit 25 - 0: Disable CC2 Pull-up Termination (Rp) 1: Enable CC2 Pull-up Termination"]
    #[inline(always)]
    pub fn rp_cc2_en(&mut self) -> RP_CC2_EN_W {
        RP_CC2_EN_W { w: self }
    }
    #[doc = "Bits 26:27 - Selects the Pull-up Termination current value 0, 2 - 80uA (Default current broadcast) 1 - 180uA (1.5A current broadcast) 3 - 330uA (3.0A current broadcast)"]
    #[inline(always)]
    pub fn rp_mode(&mut self) -> RP_MODE_W {
        RP_MODE_W { w: self }
    }
    #[doc = "Bit 28 - Enables the hysteresis mode for the line activity comparator"]
    #[inline(always)]
    pub fn en_hyst(&mut self) -> EN_HYST_W {
        EN_HYST_W { w: self }
    }
    #[doc = "Bit 29 - Selects the amount of line activity comparator hysteresis 0: 50mV hystersis 1: 100mV hysteresis\""]
    #[inline(always)]
    pub fn hyst_mode(&mut self) -> HYST_MODE_W {
        HYST_MODE_W { w: self }
    }
    #[doc = "Bit 30 - Controls the reference voltage generator for DFP vs. UFP/Cable operation 0: UFP/Cable - voltage reference is 2.4V 1: DFP - voltage reference is 2.6V\" This register is a user configuration that must be set."]
    #[inline(always)]
    pub fn dfp_en(&mut self) -> DFP_EN_W {
        DFP_EN_W { w: self }
    }
    #[doc = "Bit 31 - Disables all active circuitry and DC paths DS_ATTACH_DET_EN is still active"]
    #[inline(always)]
    pub fn pwr_disable(&mut self) -> PWR_DISABLE_W {
        PWR_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD C-connector Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_ctrl_0](index.html) module"]
pub struct CC_CTRL_0_SPEC;
impl crate::RegisterSpec for CC_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc_ctrl_0::R](R) reader structure"]
impl crate::Readable for CC_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc_ctrl_0::W](W) writer structure"]
impl crate::Writable for CC_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC_CTRL_0 to value 0xb004_0000"]
impl crate::Resettable for CC_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb004_0000
    }
}
