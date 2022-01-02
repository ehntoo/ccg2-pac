#[doc = "Register `ADC_CTRL` reader"]
pub struct R(crate::R<ADC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CTRL` writer"]
pub struct W(crate::W<ADC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CTRL_SPEC>;
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
impl From<crate::W<ADC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_CNTRL` reader - Control bits for 8-bit DAC. DAC_CNTRL register is used only if CPU wants to implement the SAR algorithm in FW."]
pub struct DAC_CNTRL_R(crate::FieldReader<u8, u8>);
impl DAC_CNTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_CNTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CNTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CNTRL` writer - Control bits for 8-bit DAC. DAC_CNTRL register is used only if CPU wants to implement the SAR algorithm in FW."]
pub struct DAC_CNTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CNTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DFT_MUXSEL` reader - ADC DFT Control: 0: Normal operation 1: DAC output voltage"]
pub struct DFT_MUXSEL_R(crate::FieldReader<bool, bool>);
impl DFT_MUXSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_MUXSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFT_MUXSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_MUXSEL` writer - ADC DFT Control: 0: Normal operation 1: DAC output voltage"]
pub struct DFT_MUXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_MUXSEL_W<'a> {
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
#[doc = "Field `ADC_ISO_N` reader - This is for when high voltage supply for a port is not present. This bit should be set when the high voltage is present, in order to ensure that the outputs are set to know values. 0: All outputs are isolated to a known value 1: Normal operation"]
pub struct ADC_ISO_N_R(crate::FieldReader<bool, bool>);
impl ADC_ISO_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_ISO_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ISO_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ISO_N` writer - This is for when high voltage supply for a port is not present. This bit should be set when the high voltage is present, in order to ensure that the outputs are set to know values. 0: All outputs are isolated to a known value 1: Normal operation"]
pub struct ADC_ISO_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ISO_N_W<'a> {
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
#[doc = "Field `CMP_OUT` reader - Comparator Output. If voltage on ID pin is less than DAC voltage, then cmp_out is HIGH."]
pub struct CMP_OUT_R(crate::FieldReader<bool, bool>);
impl CMP_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input Voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VSEL_A {
    #[doc = "0: `0`"]
    ANA_IN1_AMUX_A = 0,
    #[doc = "1: `1`"]
    ANA_IN2_AMUX_A = 1,
    #[doc = "2: `10`"]
    BAND_GAP = 2,
    #[doc = "3: `11`"]
    BJT = 3,
}
impl From<VSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VSEL` reader - Input Voltage select"]
pub struct VSEL_R(crate::FieldReader<u8, VSEL_A>);
impl VSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSEL_A {
        match self.bits {
            0 => VSEL_A::ANA_IN1_AMUX_A,
            1 => VSEL_A::ANA_IN2_AMUX_A,
            2 => VSEL_A::BAND_GAP,
            3 => VSEL_A::BJT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANA_IN1_AMUX_A`"]
    #[inline(always)]
    pub fn is_ana_in1_amux_a(&self) -> bool {
        **self == VSEL_A::ANA_IN1_AMUX_A
    }
    #[doc = "Checks if the value of the field is `ANA_IN2_AMUX_A`"]
    #[inline(always)]
    pub fn is_ana_in2_amux_a(&self) -> bool {
        **self == VSEL_A::ANA_IN2_AMUX_A
    }
    #[doc = "Checks if the value of the field is `BAND_GAP`"]
    #[inline(always)]
    pub fn is_band_gap(&self) -> bool {
        **self == VSEL_A::BAND_GAP
    }
    #[doc = "Checks if the value of the field is `BJT`"]
    #[inline(always)]
    pub fn is_bjt(&self) -> bool {
        **self == VSEL_A::BJT
    }
}
impl core::ops::Deref for VSEL_R {
    type Target = crate::FieldReader<u8, VSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSEL` writer - Input Voltage select"]
pub struct VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ana_in1_amux_a(self) -> &'a mut W {
        self.variant(VSEL_A::ANA_IN1_AMUX_A)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ana_in2_amux_a(self) -> &'a mut W {
        self.variant(VSEL_A::ANA_IN2_AMUX_A)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn band_gap(self) -> &'a mut W {
        self.variant(VSEL_A::BAND_GAP)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn bjt(self) -> &'a mut W {
        self.variant(VSEL_A::BJT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `PD_LV` reader - ADC Power down control, active high."]
pub struct PD_LV_R(crate::FieldReader<bool, bool>);
impl PD_LV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_LV` writer - ADC Power down control, active high."]
pub struct PD_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_LV_W<'a> {
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
    #[doc = "Bits 0:7 - Control bits for 8-bit DAC. DAC_CNTRL register is used only if CPU wants to implement the SAR algorithm in FW."]
    #[inline(always)]
    pub fn dac_cntrl(&self) -> DAC_CNTRL_R {
        DAC_CNTRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - ADC DFT Control: 0: Normal operation 1: DAC output voltage"]
    #[inline(always)]
    pub fn dft_muxsel(&self) -> DFT_MUXSEL_R {
        DFT_MUXSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This is for when high voltage supply for a port is not present. This bit should be set when the high voltage is present, in order to ensure that the outputs are set to know values. 0: All outputs are isolated to a known value 1: Normal operation"]
    #[inline(always)]
    pub fn adc_iso_n(&self) -> ADC_ISO_N_R {
        ADC_ISO_N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator Output. If voltage on ID pin is less than DAC voltage, then cmp_out is HIGH."]
    #[inline(always)]
    pub fn cmp_out(&self) -> CMP_OUT_R {
        CMP_OUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Input Voltage select"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 31 - ADC Power down control, active high."]
    #[inline(always)]
    pub fn pd_lv(&self) -> PD_LV_R {
        PD_LV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Control bits for 8-bit DAC. DAC_CNTRL register is used only if CPU wants to implement the SAR algorithm in FW."]
    #[inline(always)]
    pub fn dac_cntrl(&mut self) -> DAC_CNTRL_W {
        DAC_CNTRL_W { w: self }
    }
    #[doc = "Bit 8 - ADC DFT Control: 0: Normal operation 1: DAC output voltage"]
    #[inline(always)]
    pub fn dft_muxsel(&mut self) -> DFT_MUXSEL_W {
        DFT_MUXSEL_W { w: self }
    }
    #[doc = "Bit 9 - This is for when high voltage supply for a port is not present. This bit should be set when the high voltage is present, in order to ensure that the outputs are set to know values. 0: All outputs are isolated to a known value 1: Normal operation"]
    #[inline(always)]
    pub fn adc_iso_n(&mut self) -> ADC_ISO_N_W {
        ADC_ISO_N_W { w: self }
    }
    #[doc = "Bits 17:18 - Input Voltage select"]
    #[inline(always)]
    pub fn vsel(&mut self) -> VSEL_W {
        VSEL_W { w: self }
    }
    #[doc = "Bit 31 - ADC Power down control, active high."]
    #[inline(always)]
    pub fn pd_lv(&mut self) -> PD_LV_W {
        PD_LV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S8USBPD DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ctrl](index.html) module"]
pub struct ADC_CTRL_SPEC;
impl crate::RegisterSpec for ADC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ctrl::R](R) reader structure"]
impl crate::Readable for ADC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ctrl::W](W) writer structure"]
impl crate::Writable for ADC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CTRL to value 0x8000_0200"]
impl crate::Resettable for ADC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0200
    }
}
