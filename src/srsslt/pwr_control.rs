#[doc = "Register `PWR_CONTROL` reader"]
pub struct R(crate::R<PWR_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CONTROL` writer"]
pub struct W(crate::W<PWR_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CONTROL_SPEC>;
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
impl From<crate::W<PWR_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POWER_MODE_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    ACTIVE = 1,
    #[doc = "2: `10`"]
    SLEEP = 2,
    #[doc = "3: `11`"]
    DEEP_SLEEP = 3,
}
impl From<POWER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: POWER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POWER_MODE` reader - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
pub struct POWER_MODE_R(crate::FieldReader<u8, POWER_MODE_A>);
impl POWER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POWER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POWER_MODE_A> {
        match self.bits {
            0 => Some(POWER_MODE_A::RESET),
            1 => Some(POWER_MODE_A::ACTIVE),
            2 => Some(POWER_MODE_A::SLEEP),
            3 => Some(POWER_MODE_A::DEEP_SLEEP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == POWER_MODE_A::RESET
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == POWER_MODE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        **self == POWER_MODE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        **self == POWER_MODE_A::DEEP_SLEEP
    }
}
impl core::ops::Deref for POWER_MODE_R {
    type Target = crate::FieldReader<u8, POWER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_SESSION_A {
    #[doc = "0: `0`"]
    NO_SESSION = 0,
    #[doc = "1: `1`"]
    SESSION_ACTIVE = 1,
}
impl From<DEBUG_SESSION_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_SESSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUG_SESSION` reader - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
pub struct DEBUG_SESSION_R(crate::FieldReader<bool, DEBUG_SESSION_A>);
impl DEBUG_SESSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEBUG_SESSION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_SESSION_A {
        match self.bits {
            false => DEBUG_SESSION_A::NO_SESSION,
            true => DEBUG_SESSION_A::SESSION_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SESSION`"]
    #[inline(always)]
    pub fn is_no_session(&self) -> bool {
        **self == DEBUG_SESSION_A::NO_SESSION
    }
    #[doc = "Checks if the value of the field is `SESSION_ACTIVE`"]
    #[inline(always)]
    pub fn is_session_active(&self) -> bool {
        **self == DEBUG_SESSION_A::SESSION_ACTIVE
    }
}
impl core::ops::Deref for DEBUG_SESSION_R {
    type Target = crate::FieldReader<bool, DEBUG_SESSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_READY` reader - Indicates whether the low power mode regulator is ready to enter DEEPSLEEP mode. 0: If DEEPSLEEP mode is requested, device will enter SLEEP mode. When low power regulators are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP works as described."]
pub struct LPM_READY_R(crate::FieldReader<bool, bool>);
impl LPM_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER_TEMP_EN` reader - Enables the die over temperature sensor. Must be enabled when using the TEMP_HIGH interrupt."]
pub struct OVER_TEMP_EN_R(crate::FieldReader<bool, bool>);
impl OVER_TEMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVER_TEMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVER_TEMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER_TEMP_EN` writer - Enables the die over temperature sensor. Must be enabled when using the TEMP_HIGH interrupt."]
pub struct OVER_TEMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_TEMP_EN_W<'a> {
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
#[doc = "Field `OVER_TEMP_THRESH` reader - Over-temperature threshold. 0: TEMP_HIGH condition occurs between 120C and 125C. 1: TEMP_HIGH condition occurs between 60C and 75C (used for testing)."]
pub struct OVER_TEMP_THRESH_R(crate::FieldReader<bool, bool>);
impl OVER_TEMP_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVER_TEMP_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVER_TEMP_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER_TEMP_THRESH` writer - Over-temperature threshold. 0: TEMP_HIGH condition occurs between 120C and 125C. 1: TEMP_HIGH condition occurs between 60C and 75C (used for testing)."]
pub struct OVER_TEMP_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_TEMP_THRESH_W<'a> {
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
#[doc = "Field `SPARE` reader - Spare AHB readback bits that are hooked to PWR_PWRSYS_TRIM1.SPARE_TRIM\\[1:0\\]
through spare logic equivalent to bitwise inversion. Engineering only."]
pub struct SPARE_R(crate::FieldReader<u8, u8>);
impl SPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_VCCD` reader - Always write 0 except as noted below. PSoC4-S0 and Streetfighter CapSense products may set this bit if Vccd is provided externally (on Vccd pin). Setting this bit turns off the active regulator and will lead to system reset (BOD) unless both Vddd and Vccd pins are supplied externally. This register bit only resets for XRES, POR, or a detected BOD."]
pub struct EXT_VCCD_R(crate::FieldReader<bool, bool>);
impl EXT_VCCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_VCCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_VCCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_VCCD` writer - Always write 0 except as noted below. PSoC4-S0 and Streetfighter CapSense products may set this bit if Vccd is provided externally (on Vccd pin). Setting this bit turns off the active regulator and will lead to system reset (BOD) unless both Vddd and Vccd pins are supplied externally. This register bit only resets for XRES, POR, or a detected BOD."]
pub struct EXT_VCCD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_VCCD_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub fn debug_session(&self) -> DEBUG_SESSION_R {
        DEBUG_SESSION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the low power mode regulator is ready to enter DEEPSLEEP mode. 0: If DEEPSLEEP mode is requested, device will enter SLEEP mode. When low power regulators are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP works as described."]
    #[inline(always)]
    pub fn lpm_ready(&self) -> LPM_READY_R {
        LPM_READY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the die over temperature sensor. Must be enabled when using the TEMP_HIGH interrupt."]
    #[inline(always)]
    pub fn over_temp_en(&self) -> OVER_TEMP_EN_R {
        OVER_TEMP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Over-temperature threshold. 0: TEMP_HIGH condition occurs between 120C and 125C. 1: TEMP_HIGH condition occurs between 60C and 75C (used for testing)."]
    #[inline(always)]
    pub fn over_temp_thresh(&self) -> OVER_TEMP_THRESH_R {
        OVER_TEMP_THRESH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Spare AHB readback bits that are hooked to PWR_PWRSYS_TRIM1.SPARE_TRIM\\[1:0\\]
through spare logic equivalent to bitwise inversion. Engineering only."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Always write 0 except as noted below. PSoC4-S0 and Streetfighter CapSense products may set this bit if Vccd is provided externally (on Vccd pin). Setting this bit turns off the active regulator and will lead to system reset (BOD) unless both Vddd and Vccd pins are supplied externally. This register bit only resets for XRES, POR, or a detected BOD."]
    #[inline(always)]
    pub fn ext_vccd(&self) -> EXT_VCCD_R {
        EXT_VCCD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enables the die over temperature sensor. Must be enabled when using the TEMP_HIGH interrupt."]
    #[inline(always)]
    pub fn over_temp_en(&mut self) -> OVER_TEMP_EN_W {
        OVER_TEMP_EN_W { w: self }
    }
    #[doc = "Bit 17 - Over-temperature threshold. 0: TEMP_HIGH condition occurs between 120C and 125C. 1: TEMP_HIGH condition occurs between 60C and 75C (used for testing)."]
    #[inline(always)]
    pub fn over_temp_thresh(&mut self) -> OVER_TEMP_THRESH_W {
        OVER_TEMP_THRESH_W { w: self }
    }
    #[doc = "Bit 23 - Always write 0 except as noted below. PSoC4-S0 and Streetfighter CapSense products may set this bit if Vccd is provided externally (on Vccd pin). Setting this bit turns off the active regulator and will lead to system reset (BOD) unless both Vddd and Vccd pins are supplied externally. This register bit only resets for XRES, POR, or a detected BOD."]
    #[inline(always)]
    pub fn ext_vccd(&mut self) -> EXT_VCCD_W {
        EXT_VCCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Control Controls the device power mode options and allows observation of current state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_control](index.html) module"]
pub struct PWR_CONTROL_SPEC;
impl crate::RegisterSpec for PWR_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_control::R](R) reader structure"]
impl crate::Readable for PWR_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_control::W](W) writer structure"]
impl crate::Writable for PWR_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CONTROL to value 0"]
impl crate::Resettable for PWR_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
