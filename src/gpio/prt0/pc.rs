#[doc = "Register `PC` reader"]
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC` writer"]
pub struct W(crate::W<PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_SPEC>;
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
impl From<crate::W<PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The GPIO drive mode for IO pad 0. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the HSIOM is properly configured (HSIOM_PRT_SELx) before turning the IO on here to avoid producing glitches on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DM0_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    INPUT = 1,
    #[doc = "2: `10`"]
    _0_PU = 2,
    #[doc = "3: `11`"]
    PD_1 = 3,
    #[doc = "4: `100`"]
    _0_Z = 4,
    #[doc = "5: `101`"]
    Z_1 = 5,
    #[doc = "6: `110`"]
    _0_1 = 6,
    #[doc = "7: `111`"]
    PD_PU = 7,
}
impl From<DM0_A> for u8 {
    #[inline(always)]
    fn from(variant: DM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DM0` reader - The GPIO drive mode for IO pad 0. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the HSIOM is properly configured (HSIOM_PRT_SELx) before turning the IO on here to avoid producing glitches on the bus."]
pub struct DM0_R(crate::FieldReader<u8, DM0_A>);
impl DM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM0_A {
        match self.bits {
            0 => DM0_A::OFF,
            1 => DM0_A::INPUT,
            2 => DM0_A::_0_PU,
            3 => DM0_A::PD_1,
            4 => DM0_A::_0_Z,
            5 => DM0_A::Z_1,
            6 => DM0_A::_0_1,
            7 => DM0_A::PD_PU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == DM0_A::OFF
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == DM0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `_0_PU`"]
    #[inline(always)]
    pub fn is_0_pu(&self) -> bool {
        **self == DM0_A::_0_PU
    }
    #[doc = "Checks if the value of the field is `PD_1`"]
    #[inline(always)]
    pub fn is_pd_1(&self) -> bool {
        **self == DM0_A::PD_1
    }
    #[doc = "Checks if the value of the field is `_0_Z`"]
    #[inline(always)]
    pub fn is_0_z(&self) -> bool {
        **self == DM0_A::_0_Z
    }
    #[doc = "Checks if the value of the field is `Z_1`"]
    #[inline(always)]
    pub fn is_z_1(&self) -> bool {
        **self == DM0_A::Z_1
    }
    #[doc = "Checks if the value of the field is `_0_1`"]
    #[inline(always)]
    pub fn is_0_1(&self) -> bool {
        **self == DM0_A::_0_1
    }
    #[doc = "Checks if the value of the field is `PD_PU`"]
    #[inline(always)]
    pub fn is_pd_pu(&self) -> bool {
        **self == DM0_A::PD_PU
    }
}
impl core::ops::Deref for DM0_R {
    type Target = crate::FieldReader<u8, DM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM0` writer - The GPIO drive mode for IO pad 0. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the HSIOM is properly configured (HSIOM_PRT_SELx) before turning the IO on here to avoid producing glitches on the bus."]
pub struct DM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DM0_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DM0_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _0_pu(self) -> &'a mut W {
        self.variant(DM0_A::_0_PU)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pd_1(self) -> &'a mut W {
        self.variant(DM0_A::PD_1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _0_z(self) -> &'a mut W {
        self.variant(DM0_A::_0_Z)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn z_1(self) -> &'a mut W {
        self.variant(DM0_A::Z_1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _0_1(self) -> &'a mut W {
        self.variant(DM0_A::_0_1)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn pd_pu(self) -> &'a mut W {
        self.variant(DM0_A::PD_PU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DM1` reader - The GPIO drive mode for IO pad 1."]
pub struct DM1_R(crate::FieldReader<u8, u8>);
impl DM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM1` writer - The GPIO drive mode for IO pad 1."]
pub struct DM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `PORT_VTRIP_SEL` reader - The GPIO cells include a VTRIP_SEL signal to alter the input buffer voltage. Note: this bit is ignored for SIO ports, the VTRIP_SEL settings in the SIO register are used instead (a separate VTRIP_SEL is provided for each pin pair). 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer."]
pub struct PORT_VTRIP_SEL_R(crate::FieldReader<bool, bool>);
impl PORT_VTRIP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORT_VTRIP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_VTRIP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_VTRIP_SEL` writer - The GPIO cells include a VTRIP_SEL signal to alter the input buffer voltage. Note: this bit is ignored for SIO ports, the VTRIP_SEL settings in the SIO register are used instead (a separate VTRIP_SEL is provided for each pin pair). 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer."]
pub struct PORT_VTRIP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_VTRIP_SEL_W<'a> {
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
#[doc = "Field `PORT_SLOW` reader - This field controls the output edge rate of all pins on the port: '0': fast. '1': slow."]
pub struct PORT_SLOW_R(crate::FieldReader<bool, bool>);
impl PORT_SLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORT_SLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_SLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_SLOW` writer - This field controls the output edge rate of all pins on the port: '0': fast. '1': slow."]
pub struct PORT_SLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_SLOW_W<'a> {
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
#[doc = "Field `PORT_HYST_TRIM` reader - This field is used to improve the hysteresis (to 10% of vddio) of the selectable trip point input buffer. The voltage reference comes from the VREFGEN block and is only available when using the VREFGEN block: '0': <= 2.2 V input signaling Voltage. '1': > 2.2 V input signaling Voltage."]
pub struct PORT_HYST_TRIM_R(crate::FieldReader<bool, bool>);
impl PORT_HYST_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORT_HYST_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_HYST_TRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_HYST_TRIM` writer - This field is used to improve the hysteresis (to 10% of vddio) of the selectable trip point input buffer. The voltage reference comes from the VREFGEN block and is only available when using the VREFGEN block: '0': <= 2.2 V input signaling Voltage. '1': > 2.2 V input signaling Voltage."]
pub struct PORT_HYST_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_HYST_TRIM_W<'a> {
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
#[doc = "Slew control. Only used in the O_Z drive mode (mode 4: strong pull down, open drain): This field is intended for I2C functionality. See BROS 001-70428 for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT_SLEW_CTL_A {
    #[doc = "0: `0`"]
    PORT_SLEW_CTL_0 = 0,
    #[doc = "1: `1`"]
    PORT_SLEW_CTL_1 = 1,
    #[doc = "2: `10`"]
    PORT_SLEW_CTL_2 = 2,
    #[doc = "3: `11`"]
    PORT_SLEW_CTL_3 = 3,
}
impl From<PORT_SLEW_CTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT_SLEW_CTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PORT_SLEW_CTL` reader - Slew control. Only used in the O_Z drive mode (mode 4: strong pull down, open drain): This field is intended for I2C functionality. See BROS 001-70428 for more details."]
pub struct PORT_SLEW_CTL_R(crate::FieldReader<u8, PORT_SLEW_CTL_A>);
impl PORT_SLEW_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PORT_SLEW_CTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_SLEW_CTL_A {
        match self.bits {
            0 => PORT_SLEW_CTL_A::PORT_SLEW_CTL_0,
            1 => PORT_SLEW_CTL_A::PORT_SLEW_CTL_1,
            2 => PORT_SLEW_CTL_A::PORT_SLEW_CTL_2,
            3 => PORT_SLEW_CTL_A::PORT_SLEW_CTL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PORT_SLEW_CTL_0`"]
    #[inline(always)]
    pub fn is_port_slew_ctl_0(&self) -> bool {
        **self == PORT_SLEW_CTL_A::PORT_SLEW_CTL_0
    }
    #[doc = "Checks if the value of the field is `PORT_SLEW_CTL_1`"]
    #[inline(always)]
    pub fn is_port_slew_ctl_1(&self) -> bool {
        **self == PORT_SLEW_CTL_A::PORT_SLEW_CTL_1
    }
    #[doc = "Checks if the value of the field is `PORT_SLEW_CTL_2`"]
    #[inline(always)]
    pub fn is_port_slew_ctl_2(&self) -> bool {
        **self == PORT_SLEW_CTL_A::PORT_SLEW_CTL_2
    }
    #[doc = "Checks if the value of the field is `PORT_SLEW_CTL_3`"]
    #[inline(always)]
    pub fn is_port_slew_ctl_3(&self) -> bool {
        **self == PORT_SLEW_CTL_A::PORT_SLEW_CTL_3
    }
}
impl core::ops::Deref for PORT_SLEW_CTL_R {
    type Target = crate::FieldReader<u8, PORT_SLEW_CTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_SLEW_CTL` writer - Slew control. Only used in the O_Z drive mode (mode 4: strong pull down, open drain): This field is intended for I2C functionality. See BROS 001-70428 for more details."]
pub struct PORT_SLEW_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_SLEW_CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORT_SLEW_CTL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn port_slew_ctl_0(self) -> &'a mut W {
        self.variant(PORT_SLEW_CTL_A::PORT_SLEW_CTL_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn port_slew_ctl_1(self) -> &'a mut W {
        self.variant(PORT_SLEW_CTL_A::PORT_SLEW_CTL_1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn port_slew_ctl_2(self) -> &'a mut W {
        self.variant(PORT_SLEW_CTL_A::PORT_SLEW_CTL_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn port_slew_ctl_3(self) -> &'a mut W {
        self.variant(PORT_SLEW_CTL_A::PORT_SLEW_CTL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `PORT_IB_MODE_SEL` reader - This field selects the input buffer reference. The size (1 or 2 bits) and functionality is dependent on the IO cell. For GPIOv2 IO cells, bit PORT_IB_MODE_SEL\\[1\\]
is not used (GPIOv2 IO cell replaces GPIO IO cell): \"0\"/\"2\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\"/\"3\": vcchib. For GPIO_OVTv2 and SIOv2 IO cells: \"0\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\": vcchib. \"2\": OVT. \"3\": Reference (possibly from reference generator cell). For SIO IO cell, this field is present but not used as the SIO IO cell does not provide input buffer mode select functionality (SIOv2 IO cell will replace SIO IO cell, as soon as it is available)."]
pub struct PORT_IB_MODE_SEL_R(crate::FieldReader<u8, u8>);
impl PORT_IB_MODE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PORT_IB_MODE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_IB_MODE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_IB_MODE_SEL` writer - This field selects the input buffer reference. The size (1 or 2 bits) and functionality is dependent on the IO cell. For GPIOv2 IO cells, bit PORT_IB_MODE_SEL\\[1\\]
is not used (GPIOv2 IO cell replaces GPIO IO cell): \"0\"/\"2\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\"/\"3\": vcchib. For GPIO_OVTv2 and SIOv2 IO cells: \"0\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\": vcchib. \"2\": OVT. \"3\": Reference (possibly from reference generator cell). For SIO IO cell, this field is present but not used as the SIO IO cell does not provide input buffer mode select functionality (SIOv2 IO cell will replace SIO IO cell, as soon as it is available)."]
pub struct PORT_IB_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_IB_MODE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pad 0. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the HSIOM is properly configured (HSIOM_PRT_SELx) before turning the IO on here to avoid producing glitches on the bus."]
    #[inline(always)]
    pub fn dm0(&self) -> DM0_R {
        DM0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - The GPIO drive mode for IO pad 1."]
    #[inline(always)]
    pub fn dm1(&self) -> DM1_R {
        DM1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 24 - The GPIO cells include a VTRIP_SEL signal to alter the input buffer voltage. Note: this bit is ignored for SIO ports, the VTRIP_SEL settings in the SIO register are used instead (a separate VTRIP_SEL is provided for each pin pair). 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer."]
    #[inline(always)]
    pub fn port_vtrip_sel(&self) -> PORT_VTRIP_SEL_R {
        PORT_VTRIP_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This field controls the output edge rate of all pins on the port: '0': fast. '1': slow."]
    #[inline(always)]
    pub fn port_slow(&self) -> PORT_SLOW_R {
        PORT_SLOW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - This field is used to improve the hysteresis (to 10% of vddio) of the selectable trip point input buffer. The voltage reference comes from the VREFGEN block and is only available when using the VREFGEN block: '0': <= 2.2 V input signaling Voltage. '1': > 2.2 V input signaling Voltage."]
    #[inline(always)]
    pub fn port_hyst_trim(&self) -> PORT_HYST_TRIM_R {
        PORT_HYST_TRIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Slew control. Only used in the O_Z drive mode (mode 4: strong pull down, open drain): This field is intended for I2C functionality. See BROS 001-70428 for more details."]
    #[inline(always)]
    pub fn port_slew_ctl(&self) -> PORT_SLEW_CTL_R {
        PORT_SLEW_CTL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - This field selects the input buffer reference. The size (1 or 2 bits) and functionality is dependent on the IO cell. For GPIOv2 IO cells, bit PORT_IB_MODE_SEL\\[1\\]
is not used (GPIOv2 IO cell replaces GPIO IO cell): \"0\"/\"2\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\"/\"3\": vcchib. For GPIO_OVTv2 and SIOv2 IO cells: \"0\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\": vcchib. \"2\": OVT. \"3\": Reference (possibly from reference generator cell). For SIO IO cell, this field is present but not used as the SIO IO cell does not provide input buffer mode select functionality (SIOv2 IO cell will replace SIO IO cell, as soon as it is available)."]
    #[inline(always)]
    pub fn port_ib_mode_sel(&self) -> PORT_IB_MODE_SEL_R {
        PORT_IB_MODE_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pad 0. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the HSIOM is properly configured (HSIOM_PRT_SELx) before turning the IO on here to avoid producing glitches on the bus."]
    #[inline(always)]
    pub fn dm0(&mut self) -> DM0_W {
        DM0_W { w: self }
    }
    #[doc = "Bits 3:5 - The GPIO drive mode for IO pad 1."]
    #[inline(always)]
    pub fn dm1(&mut self) -> DM1_W {
        DM1_W { w: self }
    }
    #[doc = "Bit 24 - The GPIO cells include a VTRIP_SEL signal to alter the input buffer voltage. Note: this bit is ignored for SIO ports, the VTRIP_SEL settings in the SIO register are used instead (a separate VTRIP_SEL is provided for each pin pair). 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer."]
    #[inline(always)]
    pub fn port_vtrip_sel(&mut self) -> PORT_VTRIP_SEL_W {
        PORT_VTRIP_SEL_W { w: self }
    }
    #[doc = "Bit 25 - This field controls the output edge rate of all pins on the port: '0': fast. '1': slow."]
    #[inline(always)]
    pub fn port_slow(&mut self) -> PORT_SLOW_W {
        PORT_SLOW_W { w: self }
    }
    #[doc = "Bit 27 - This field is used to improve the hysteresis (to 10% of vddio) of the selectable trip point input buffer. The voltage reference comes from the VREFGEN block and is only available when using the VREFGEN block: '0': <= 2.2 V input signaling Voltage. '1': > 2.2 V input signaling Voltage."]
    #[inline(always)]
    pub fn port_hyst_trim(&mut self) -> PORT_HYST_TRIM_W {
        PORT_HYST_TRIM_W { w: self }
    }
    #[doc = "Bits 28:29 - Slew control. Only used in the O_Z drive mode (mode 4: strong pull down, open drain): This field is intended for I2C functionality. See BROS 001-70428 for more details."]
    #[inline(always)]
    pub fn port_slew_ctl(&mut self) -> PORT_SLEW_CTL_W {
        PORT_SLEW_CTL_W { w: self }
    }
    #[doc = "Bits 30:31 - This field selects the input buffer reference. The size (1 or 2 bits) and functionality is dependent on the IO cell. For GPIOv2 IO cells, bit PORT_IB_MODE_SEL\\[1\\]
is not used (GPIOv2 IO cell replaces GPIO IO cell): \"0\"/\"2\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\"/\"3\": vcchib. For GPIO_OVTv2 and SIOv2 IO cells: \"0\": CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') \"1\": vcchib. \"2\": OVT. \"3\": Reference (possibly from reference generator cell). For SIO IO cell, this field is present but not used as the SIO IO cell does not provide input buffer mode select functionality (SIOv2 IO cell will replace SIO IO cell, as soon as it is available)."]
    #[inline(always)]
    pub fn port_ib_mode_sel(&mut self) -> PORT_IB_MODE_SEL_W {
        PORT_IB_MODE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register Configures the output drive and input buffer state for each pin, and the slew rate and input threshold selection for the whole port. One register is provided per port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](index.html) module"]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc::R](R) reader structure"]
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc::W](W) writer structure"]
impl crate::Writable for PC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
