#[doc = "Register `DFT_IO_TEST` reader"]
pub struct R(crate::R<DFT_IO_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFT_IO_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFT_IO_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFT_IO_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFT_IO_TEST` writer"]
pub struct W(crate::W<DFT_IO_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFT_IO_TEST_SPEC>;
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
impl From<crate::W<DFT_IO_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFT_IO_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DfT IO SELF TEST mode:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFT_IO_TEST_MODE_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    TEST_ADFT = 1,
    #[doc = "2: `10`"]
    TEST_ANA = 2,
    #[doc = "3: `11`"]
    TEST_GEN = 3,
}
impl From<DFT_IO_TEST_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DFT_IO_TEST_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DFT_IO_TEST_MODE` reader - DfT IO SELF TEST mode:"]
pub struct DFT_IO_TEST_MODE_R(crate::FieldReader<u8, DFT_IO_TEST_MODE_A>);
impl DFT_IO_TEST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFT_IO_TEST_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFT_IO_TEST_MODE_A {
        match self.bits {
            0 => DFT_IO_TEST_MODE_A::OFF,
            1 => DFT_IO_TEST_MODE_A::TEST_ADFT,
            2 => DFT_IO_TEST_MODE_A::TEST_ANA,
            3 => DFT_IO_TEST_MODE_A::TEST_GEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == DFT_IO_TEST_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `TEST_ADFT`"]
    #[inline(always)]
    pub fn is_test_adft(&self) -> bool {
        **self == DFT_IO_TEST_MODE_A::TEST_ADFT
    }
    #[doc = "Checks if the value of the field is `TEST_ANA`"]
    #[inline(always)]
    pub fn is_test_ana(&self) -> bool {
        **self == DFT_IO_TEST_MODE_A::TEST_ANA
    }
    #[doc = "Checks if the value of the field is `TEST_GEN`"]
    #[inline(always)]
    pub fn is_test_gen(&self) -> bool {
        **self == DFT_IO_TEST_MODE_A::TEST_GEN
    }
}
impl core::ops::Deref for DFT_IO_TEST_MODE_R {
    type Target = crate::FieldReader<u8, DFT_IO_TEST_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_IO_TEST_MODE` writer - DfT IO SELF TEST mode:"]
pub struct DFT_IO_TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_IO_TEST_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFT_IO_TEST_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DFT_IO_TEST_MODE_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn test_adft(self) -> &'a mut W {
        self.variant(DFT_IO_TEST_MODE_A::TEST_ADFT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn test_ana(self) -> &'a mut W {
        self.variant(DFT_IO_TEST_MODE_A::TEST_ANA)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn test_gen(self) -> &'a mut W {
        self.variant(DFT_IO_TEST_MODE_A::TEST_GEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DFT_HLD_OVR_0` reader - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_HLD_OVR_0_R(crate::FieldReader<bool, bool>);
impl DFT_HLD_OVR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_HLD_OVR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_HLD_OVR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_HLD_OVR_0` writer - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_HLD_OVR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_HLD_OVR_0_W<'a> {
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
#[doc = "Field `DFT_OE_N_0` reader - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_OE_N_0_R(crate::FieldReader<bool, bool>);
impl DFT_OE_N_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_OE_N_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_OE_N_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_OE_N_0` writer - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_OE_N_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_OE_N_0_W<'a> {
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
#[doc = "Field `DFT_ANALOG_EN_0` reader - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANALOG_EN_0_R(crate::FieldReader<bool, bool>);
impl DFT_ANALOG_EN_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANALOG_EN_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANALOG_EN_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANALOG_EN_0` writer - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANALOG_EN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANALOG_EN_0_W<'a> {
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
#[doc = "Field `DFT_ANA_SEL_0` reader - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_SEL_0_R(crate::FieldReader<bool, bool>);
impl DFT_ANA_SEL_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANA_SEL_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANA_SEL_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANA_SEL_0` writer - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_SEL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANA_SEL_0_W<'a> {
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
#[doc = "Field `DFT_ANA_POL_0` reader - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_POL_0_R(crate::FieldReader<bool, bool>);
impl DFT_ANA_POL_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANA_POL_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANA_POL_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANA_POL_0` writer - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_POL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANA_POL_0_W<'a> {
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
#[doc = "Field `DFT_HLD_OVR_1` reader - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_HLD_OVR_1_R(crate::FieldReader<bool, bool>);
impl DFT_HLD_OVR_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_HLD_OVR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_HLD_OVR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_HLD_OVR_1` writer - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_HLD_OVR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_HLD_OVR_1_W<'a> {
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
#[doc = "Field `DFT_OE_N_1` reader - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_OE_N_1_R(crate::FieldReader<bool, bool>);
impl DFT_OE_N_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_OE_N_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_OE_N_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_OE_N_1` writer - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_OE_N_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_OE_N_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DFT_ANALOG_EN_1` reader - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANALOG_EN_1_R(crate::FieldReader<bool, bool>);
impl DFT_ANALOG_EN_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANALOG_EN_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANALOG_EN_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANALOG_EN_1` writer - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANALOG_EN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANALOG_EN_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `DFT_ANA_SEL_1` reader - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_SEL_1_R(crate::FieldReader<bool, bool>);
impl DFT_ANA_SEL_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANA_SEL_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANA_SEL_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANA_SEL_1` writer - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_SEL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANA_SEL_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `DFT_ANA_POL_1` reader - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_POL_1_R(crate::FieldReader<bool, bool>);
impl DFT_ANA_POL_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANA_POL_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANA_POL_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANA_POL_1` writer - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
pub struct DFT_ANA_POL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANA_POL_1_W<'a> {
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
#[doc = "Field `DFT_HLD_OVR_2` reader - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of all IO cells other than ADFT-0/1 . TEST_ANA: Connects to �hld_ovr� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �hld_ovr� of all IO cells."]
pub struct DFT_HLD_OVR_2_R(crate::FieldReader<bool, bool>);
impl DFT_HLD_OVR_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_HLD_OVR_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_HLD_OVR_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_HLD_OVR_2` writer - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of all IO cells other than ADFT-0/1 . TEST_ANA: Connects to �hld_ovr� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �hld_ovr� of all IO cells."]
pub struct DFT_HLD_OVR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_HLD_OVR_2_W<'a> {
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
#[doc = "Field `DFT_OE_N_2` reader - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �oe_n� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �oe_n� of all IO cells"]
pub struct DFT_OE_N_2_R(crate::FieldReader<bool, bool>);
impl DFT_OE_N_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_OE_N_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_OE_N_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_OE_N_2` writer - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �oe_n� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �oe_n� of all IO cells"]
pub struct DFT_OE_N_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_OE_N_2_W<'a> {
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
#[doc = "Field `DFT_ANALOG_EN_2` reader - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of all IO cells other than ADFT-0/1. TEST_ANA: DFT_ANALOG_EN_2 && DM\\[0\\]
connects to �analog_en� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_en� of all IO cells"]
pub struct DFT_ANALOG_EN_2_R(crate::FieldReader<bool, bool>);
impl DFT_ANALOG_EN_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANALOG_EN_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANALOG_EN_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANALOG_EN_2` writer - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of all IO cells other than ADFT-0/1. TEST_ANA: DFT_ANALOG_EN_2 && DM\\[0\\]
connects to �analog_en� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_en� of all IO cells"]
pub struct DFT_ANALOG_EN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANALOG_EN_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DFT_ANA_SEL_2` reader - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_sel� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_sel� of all IO cells."]
pub struct DFT_ANA_SEL_2_R(crate::FieldReader<bool, bool>);
impl DFT_ANA_SEL_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANA_SEL_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANA_SEL_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANA_SEL_2` writer - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_sel� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_sel� of all IO cells."]
pub struct DFT_ANA_SEL_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANA_SEL_2_W<'a> {
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
#[doc = "Field `DFT_ANA_POL_2` reader - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_pol� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_pol� of all IO cells."]
pub struct DFT_ANA_POL_2_R(crate::FieldReader<bool, bool>);
impl DFT_ANA_POL_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_ANA_POL_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_ANA_POL_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_ANA_POL_2` writer - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_pol� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_pol� of all IO cells."]
pub struct DFT_ANA_POL_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_ANA_POL_2_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - DfT IO SELF TEST mode:"]
    #[inline(always)]
    pub fn dft_io_test_mode(&self) -> DFT_IO_TEST_MODE_R {
        DFT_IO_TEST_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_hld_ovr_0(&self) -> DFT_HLD_OVR_0_R {
        DFT_HLD_OVR_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_oe_n_0(&self) -> DFT_OE_N_0_R {
        DFT_OE_N_0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_analog_en_0(&self) -> DFT_ANALOG_EN_0_R {
        DFT_ANALOG_EN_0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_sel_0(&self) -> DFT_ANA_SEL_0_R {
        DFT_ANA_SEL_0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_pol_0(&self) -> DFT_ANA_POL_0_R {
        DFT_ANA_POL_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_hld_ovr_1(&self) -> DFT_HLD_OVR_1_R {
        DFT_HLD_OVR_1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_oe_n_1(&self) -> DFT_OE_N_1_R {
        DFT_OE_N_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_analog_en_1(&self) -> DFT_ANALOG_EN_1_R {
        DFT_ANALOG_EN_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_sel_1(&self) -> DFT_ANA_SEL_1_R {
        DFT_ANA_SEL_1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_pol_1(&self) -> DFT_ANA_POL_1_R {
        DFT_ANA_POL_1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of all IO cells other than ADFT-0/1 . TEST_ANA: Connects to �hld_ovr� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �hld_ovr� of all IO cells."]
    #[inline(always)]
    pub fn dft_hld_ovr_2(&self) -> DFT_HLD_OVR_2_R {
        DFT_HLD_OVR_2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �oe_n� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �oe_n� of all IO cells"]
    #[inline(always)]
    pub fn dft_oe_n_2(&self) -> DFT_OE_N_2_R {
        DFT_OE_N_2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of all IO cells other than ADFT-0/1. TEST_ANA: DFT_ANALOG_EN_2 && DM\\[0\\]
connects to �analog_en� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_en� of all IO cells"]
    #[inline(always)]
    pub fn dft_analog_en_2(&self) -> DFT_ANALOG_EN_2_R {
        DFT_ANALOG_EN_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_sel� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_sel� of all IO cells."]
    #[inline(always)]
    pub fn dft_ana_sel_2(&self) -> DFT_ANA_SEL_2_R {
        DFT_ANA_SEL_2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_pol� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_pol� of all IO cells."]
    #[inline(always)]
    pub fn dft_ana_pol_2(&self) -> DFT_ANA_POL_2_R {
        DFT_ANA_POL_2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DfT IO SELF TEST mode:"]
    #[inline(always)]
    pub fn dft_io_test_mode(&mut self) -> DFT_IO_TEST_MODE_W {
        DFT_IO_TEST_MODE_W { w: self }
    }
    #[doc = "Bit 8 - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_hld_ovr_0(&mut self) -> DFT_HLD_OVR_0_W {
        DFT_HLD_OVR_0_W { w: self }
    }
    #[doc = "Bit 9 - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_oe_n_0(&mut self) -> DFT_OE_N_0_W {
        DFT_OE_N_0_W { w: self }
    }
    #[doc = "Bit 10 - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_analog_en_0(&mut self) -> DFT_ANALOG_EN_0_W {
        DFT_ANALOG_EN_0_W { w: self }
    }
    #[doc = "Bit 11 - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_sel_0(&mut self) -> DFT_ANA_SEL_0_W {
        DFT_ANA_SEL_0_W { w: self }
    }
    #[doc = "Bit 12 - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-0 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_0 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_pol_0(&mut self) -> DFT_ANA_POL_0_W {
        DFT_ANA_POL_0_W { w: self }
    }
    #[doc = "Bit 16 - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �hld_ovr� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_hld_ovr_1(&mut self) -> DFT_HLD_OVR_1_W {
        DFT_HLD_OVR_1_W { w: self }
    }
    #[doc = "Bit 17 - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �oe_n� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_oe_n_1(&mut self) -> DFT_OE_N_1_W {
        DFT_OE_N_1_W { w: self }
    }
    #[doc = "Bit 18 - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_en� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_analog_en_1(&mut self) -> DFT_ANALOG_EN_1_W {
        DFT_ANALOG_EN_1_W { w: self }
    }
    #[doc = "Bit 19 - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_sel� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_sel_1(&mut self) -> DFT_ANA_SEL_1_W {
        DFT_ANA_SEL_1_W { w: self }
    }
    #[doc = "Bit 20 - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of the ADFT-1 assigned IO cell. TEST_ANA: Connects to �analog_pol� of the IO_TEST_1 assigned IO cell. TEST_GEN: not used."]
    #[inline(always)]
    pub fn dft_ana_pol_1(&mut self) -> DFT_ANA_POL_1_W {
        DFT_ANA_POL_1_W { w: self }
    }
    #[doc = "Bit 24 - �hld_ovr� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �hld_ovr� of all IO cells other than ADFT-0/1 . TEST_ANA: Connects to �hld_ovr� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �hld_ovr� of all IO cells."]
    #[inline(always)]
    pub fn dft_hld_ovr_2(&mut self) -> DFT_HLD_OVR_2_W {
        DFT_HLD_OVR_2_W { w: self }
    }
    #[doc = "Bit 25 - �oe_n� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �oe_n� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �oe_n� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �oe_n� of all IO cells"]
    #[inline(always)]
    pub fn dft_oe_n_2(&mut self) -> DFT_OE_N_2_W {
        DFT_OE_N_2_W { w: self }
    }
    #[doc = "Bit 26 - �analog_en� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_en� of all IO cells other than ADFT-0/1. TEST_ANA: DFT_ANALOG_EN_2 && DM\\[0\\]
connects to �analog_en� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_en� of all IO cells"]
    #[inline(always)]
    pub fn dft_analog_en_2(&mut self) -> DFT_ANALOG_EN_2_W {
        DFT_ANALOG_EN_2_W { w: self }
    }
    #[doc = "Bit 27 - �analog_sel� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_sel� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_sel� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_sel� of all IO cells."]
    #[inline(always)]
    pub fn dft_ana_sel_2(&mut self) -> DFT_ANA_SEL_2_W {
        DFT_ANA_SEL_2_W { w: self }
    }
    #[doc = "Bit 28 - �analog_pol� DfT control for IO cells depending on DFT_IO_TEST_MODE as given below. TEST_ADFT: Connects to �analog_pol� of all IO cells other than ADFT-0/1. TEST_ANA: Connects to �analog_pol� of all IO cells other than IO_TEST_0/1. TEST_GEN: Connects to �analog_pol� of all IO cells."]
    #[inline(always)]
    pub fn dft_ana_pol_2(&mut self) -> DFT_ANA_POL_2_W {
        DFT_ANA_POL_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO SELF TEST control register for DfT purposes only This register is used to significantly reduce the test time for IO cells. It also avoids the need to develop a large amount of chip specific functional test vectors. With ATPG it is not possible to get full stuck-at fault coverage for some IO cell inputs (�hld_ovr, oe_n, analog_en, analog_sel, analog_pol�). This register gives direct controlabilty on those inputs of all IO cells by bypassing the functional paths. That allows generic (not chip specific) ROOS code (also SWD IO cells are included) to get DfT fault-coverage for these signals. This register is used in conjunction with the GPIO.PC.dm\\[2:0\\], GPIO.DR.out and SRSS.CORE.PWR_STOP.FREEZE for control and results are observed through GPIO.PS.data. Only one IO cell on the whole chip gets IO_TEST_0 and only one gets IO_TEST_1 and default values of IO_TEST_0 is '0' and IO_TEST_1 is '1'. Also only one IO cell on the whole chip gets asssigned ADFT-0 and only one gets asssigned ADFT-1. All four IO_TEST_0/1 and ADFT-0/1 pins are assigned in the product pin spreadsheet.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dft_io_test](index.html) module"]
pub struct DFT_IO_TEST_SPEC;
impl crate::RegisterSpec for DFT_IO_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dft_io_test::R](R) reader structure"]
impl crate::Readable for DFT_IO_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dft_io_test::W](W) writer structure"]
impl crate::Writable for DFT_IO_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFT_IO_TEST to value 0x0202_0200"]
impl crate::Resettable for DFT_IO_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0202_0200
    }
}
