#[doc = "Register `CC_CTRL_1` reader"]
pub struct R(crate::R<CC_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC_CTRL_1` writer"]
pub struct W(crate::W<CC_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_CTRL_1_SPEC>;
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
impl From<crate::W<CC_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC_ADFT_EN` reader - Enables ADFT Mode"]
pub struct CC_ADFT_EN_R(crate::FieldReader<bool, bool>);
impl CC_ADFT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_ADFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_ADFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_ADFT_EN` writer - Enables ADFT Mode"]
pub struct CC_ADFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_ADFT_EN_W<'a> {
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
#[doc = "Field `CC_ADFT_CTRL` reader - Selects ADFT connection See s8usbpd BROS for decoding details"]
pub struct CC_ADFT_CTRL_R(crate::FieldReader<u8, u8>);
impl CC_ADFT_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC_ADFT_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_ADFT_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_ADFT_CTRL` writer - Selects ADFT connection See s8usbpd BROS for decoding details"]
pub struct CC_ADFT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_ADFT_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `RX_OFFSET_EN` reader - Enables the offset generator for the RX comparator 0 - no offset 1 - offset enabled, see RX_OFFSET register for value"]
pub struct RX_OFFSET_EN_R(crate::FieldReader<bool, bool>);
impl RX_OFFSET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OFFSET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_OFFSET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OFFSET_EN` writer - Enables the offset generator for the RX comparator 0 - no offset 1 - offset enabled, see RX_OFFSET register for value"]
pub struct RX_OFFSET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OFFSET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Selects the RX comparator offset:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_OFFSET_A {
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
impl From<RX_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_OFFSET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_OFFSET` reader - Selects the RX comparator offset:"]
pub struct RX_OFFSET_R(crate::FieldReader<u8, RX_OFFSET_A>);
impl RX_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_OFFSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OFFSET_A {
        match self.bits {
            0 => RX_OFFSET_A::SEL_MINUS_50_MV,
            1 => RX_OFFSET_A::SEL_MINUS_100_MV,
            2 => RX_OFFSET_A::SEL_MINUS_150_MV,
            3 => RX_OFFSET_A::SEL_MINUS_200_MV,
            4 => RX_OFFSET_A::SEL_PLUS_50_MV,
            5 => RX_OFFSET_A::SEL_PLUS_100_MV,
            6 => RX_OFFSET_A::SEL_PLUS_150_MV,
            7 => RX_OFFSET_A::SEL_PLUS_200_MV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_50_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_50_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_MINUS_50_MV
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_100_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_100_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_MINUS_100_MV
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_150_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_150_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_MINUS_150_MV
    }
    #[doc = "Checks if the value of the field is `SEL_MINUS_200_MV`"]
    #[inline(always)]
    pub fn is_sel_minus_200_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_MINUS_200_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_50_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_50_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_PLUS_50_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_100_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_100_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_PLUS_100_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_150_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_150_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_PLUS_150_MV
    }
    #[doc = "Checks if the value of the field is `SEL_PLUS_200_MV`"]
    #[inline(always)]
    pub fn is_sel_plus_200_mv(&self) -> bool {
        **self == RX_OFFSET_A::SEL_PLUS_200_MV
    }
}
impl core::ops::Deref for RX_OFFSET_R {
    type Target = crate::FieldReader<u8, RX_OFFSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OFFSET` writer - Selects the RX comparator offset:"]
pub struct RX_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_OFFSET_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_minus_50_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_MINUS_50_MV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_minus_100_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_MINUS_100_MV)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_minus_150_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_MINUS_150_MV)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_minus_200_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_MINUS_200_MV)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_plus_50_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_PLUS_50_MV)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_plus_100_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_PLUS_100_MV)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_plus_150_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_PLUS_150_MV)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_plus_200_mv(self) -> &'a mut W {
        self.variant(RX_OFFSET_A::SEL_PLUS_200_MV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `DS_ATTACH_DET_EN` reader - Enables the deepsleep attach detect pull-up resistor Set HI for a DFP waiting for attach"]
pub struct DS_ATTACH_DET_EN_R(crate::FieldReader<bool, bool>);
impl DS_ATTACH_DET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DS_ATTACH_DET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS_ATTACH_DET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS_ATTACH_DET_EN` writer - Enables the deepsleep attach detect pull-up resistor Set HI for a DFP waiting for attach"]
pub struct DS_ATTACH_DET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_ATTACH_DET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Transmit voltage select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VTX_SEL_A {
    #[doc = "0: `0`"]
    SEL_0_93_V = 0,
    #[doc = "1: `1`"]
    SEL_1_03_V = 1,
    #[doc = "2: `10`"]
    SEL_1_125_V = 2,
    #[doc = "3: `11`"]
    SEL_1_225_V = 3,
    #[doc = "4: `100`"]
    SEL_1_33_V = 4,
    #[doc = "5: `101`"]
    SEL_1_43_V = 5,
    #[doc = "6: `110`"]
    SEL_1_53_V = 6,
    #[doc = "7: `111`"]
    SEL_1_63_V = 7,
}
impl From<VTX_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VTX_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VTX_SEL` reader - Transmit voltage select"]
pub struct VTX_SEL_R(crate::FieldReader<u8, VTX_SEL_A>);
impl VTX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VTX_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VTX_SEL_A {
        match self.bits {
            0 => VTX_SEL_A::SEL_0_93_V,
            1 => VTX_SEL_A::SEL_1_03_V,
            2 => VTX_SEL_A::SEL_1_125_V,
            3 => VTX_SEL_A::SEL_1_225_V,
            4 => VTX_SEL_A::SEL_1_33_V,
            5 => VTX_SEL_A::SEL_1_43_V,
            6 => VTX_SEL_A::SEL_1_53_V,
            7 => VTX_SEL_A::SEL_1_63_V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL_0_93_V`"]
    #[inline(always)]
    pub fn is_sel_0_93_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_0_93_V
    }
    #[doc = "Checks if the value of the field is `SEL_1_03_V`"]
    #[inline(always)]
    pub fn is_sel_1_03_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_1_03_V
    }
    #[doc = "Checks if the value of the field is `SEL_1_125_V`"]
    #[inline(always)]
    pub fn is_sel_1_125_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_1_125_V
    }
    #[doc = "Checks if the value of the field is `SEL_1_225_V`"]
    #[inline(always)]
    pub fn is_sel_1_225_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_1_225_V
    }
    #[doc = "Checks if the value of the field is `SEL_1_33_V`"]
    #[inline(always)]
    pub fn is_sel_1_33_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_1_33_V
    }
    #[doc = "Checks if the value of the field is `SEL_1_43_V`"]
    #[inline(always)]
    pub fn is_sel_1_43_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_1_43_V
    }
    #[doc = "Checks if the value of the field is `SEL_1_53_V`"]
    #[inline(always)]
    pub fn is_sel_1_53_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_1_53_V
    }
    #[doc = "Checks if the value of the field is `SEL_1_63_V`"]
    #[inline(always)]
    pub fn is_sel_1_63_v(&self) -> bool {
        **self == VTX_SEL_A::SEL_1_63_V
    }
}
impl core::ops::Deref for VTX_SEL_R {
    type Target = crate::FieldReader<u8, VTX_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTX_SEL` writer - Transmit voltage select"]
pub struct VTX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VTX_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTX_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_0_93_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_0_93_V)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_1_03_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_1_03_V)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sel_1_125_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_1_125_V)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sel_1_225_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_1_225_V)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sel_1_33_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_1_33_V)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sel_1_43_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_1_43_V)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sel_1_53_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_1_53_V)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sel_1_63_v(self) -> &'a mut W {
        self.variant(VTX_SEL_A::SEL_1_63_V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `CC_ISO_N` reader - 0: All outputs are isolated to a known value 1: Normal operation"]
pub struct CC_ISO_N_R(crate::FieldReader<bool, bool>);
impl CC_ISO_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_ISO_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_ISO_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC_ISO_N` writer - 0: All outputs are isolated to a known value 1: Normal operation"]
pub struct CC_ISO_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_ISO_N_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enables ADFT Mode"]
    #[inline(always)]
    pub fn cc_adft_en(&self) -> CC_ADFT_EN_R {
        CC_ADFT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Selects ADFT connection See s8usbpd BROS for decoding details"]
    #[inline(always)]
    pub fn cc_adft_ctrl(&self) -> CC_ADFT_CTRL_R {
        CC_ADFT_CTRL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Enables the offset generator for the RX comparator 0 - no offset 1 - offset enabled, see RX_OFFSET register for value"]
    #[inline(always)]
    pub fn rx_offset_en(&self) -> RX_OFFSET_EN_R {
        RX_OFFSET_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - Selects the RX comparator offset:"]
    #[inline(always)]
    pub fn rx_offset(&self) -> RX_OFFSET_R {
        RX_OFFSET_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Enables the deepsleep attach detect pull-up resistor Set HI for a DFP waiting for attach"]
    #[inline(always)]
    pub fn ds_attach_det_en(&self) -> DS_ATTACH_DET_EN_R {
        DS_ATTACH_DET_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Transmit voltage select"]
    #[inline(always)]
    pub fn vtx_sel(&self) -> VTX_SEL_R {
        VTX_SEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - 0: All outputs are isolated to a known value 1: Normal operation"]
    #[inline(always)]
    pub fn cc_iso_n(&self) -> CC_ISO_N_R {
        CC_ISO_N_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables ADFT Mode"]
    #[inline(always)]
    pub fn cc_adft_en(&mut self) -> CC_ADFT_EN_W {
        CC_ADFT_EN_W { w: self }
    }
    #[doc = "Bits 1:4 - Selects ADFT connection See s8usbpd BROS for decoding details"]
    #[inline(always)]
    pub fn cc_adft_ctrl(&mut self) -> CC_ADFT_CTRL_W {
        CC_ADFT_CTRL_W { w: self }
    }
    #[doc = "Bit 6 - Enables the offset generator for the RX comparator 0 - no offset 1 - offset enabled, see RX_OFFSET register for value"]
    #[inline(always)]
    pub fn rx_offset_en(&mut self) -> RX_OFFSET_EN_W {
        RX_OFFSET_EN_W { w: self }
    }
    #[doc = "Bits 7:9 - Selects the RX comparator offset:"]
    #[inline(always)]
    pub fn rx_offset(&mut self) -> RX_OFFSET_W {
        RX_OFFSET_W { w: self }
    }
    #[doc = "Bit 10 - Enables the deepsleep attach detect pull-up resistor Set HI for a DFP waiting for attach"]
    #[inline(always)]
    pub fn ds_attach_det_en(&mut self) -> DS_ATTACH_DET_EN_W {
        DS_ATTACH_DET_EN_W { w: self }
    }
    #[doc = "Bits 11:13 - Transmit voltage select"]
    #[inline(always)]
    pub fn vtx_sel(&mut self) -> VTX_SEL_W {
        VTX_SEL_W { w: self }
    }
    #[doc = "Bit 14 - 0: All outputs are isolated to a known value 1: Normal operation"]
    #[inline(always)]
    pub fn cc_iso_n(&mut self) -> CC_ISO_N_W {
        CC_ISO_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD C-connector Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_ctrl_1](index.html) module"]
pub struct CC_CTRL_1_SPEC;
impl crate::RegisterSpec for CC_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc_ctrl_1::R](R) reader structure"]
impl crate::Readable for CC_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc_ctrl_1::W](W) writer structure"]
impl crate::Writable for CC_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC_CTRL_1 to value 0x5000"]
impl crate::Resettable for CC_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000
    }
}
