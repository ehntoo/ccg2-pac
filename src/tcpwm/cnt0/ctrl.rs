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
#[doc = "Field `AUTO_RELOAD_CC` reader - "]
pub struct AUTO_RELOAD_CC_R(crate::FieldReader<bool, bool>);
impl AUTO_RELOAD_CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_RELOAD_CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_RELOAD_CC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_RELOAD_CC` writer - "]
pub struct AUTO_RELOAD_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_CC_W<'a> {
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
#[doc = "Field `AUTO_RELOAD_PERIOD` reader - "]
pub struct AUTO_RELOAD_PERIOD_R(crate::FieldReader<bool, bool>);
impl AUTO_RELOAD_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_RELOAD_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_RELOAD_PERIOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_RELOAD_PERIOD` writer - "]
pub struct AUTO_RELOAD_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_PERIOD_W<'a> {
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
#[doc = "Field `PWM_SYNC_KILL` reader - "]
pub struct PWM_SYNC_KILL_R(crate::FieldReader<bool, bool>);
impl PWM_SYNC_KILL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWM_SYNC_KILL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_SYNC_KILL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM_SYNC_KILL` writer - "]
pub struct PWM_SYNC_KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SYNC_KILL_W<'a> {
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
#[doc = "Field `PWM_STOP_ON_KILL` reader - "]
pub struct PWM_STOP_ON_KILL_R(crate::FieldReader<bool, bool>);
impl PWM_STOP_ON_KILL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWM_STOP_ON_KILL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_STOP_ON_KILL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM_STOP_ON_KILL` writer - "]
pub struct PWM_STOP_ON_KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STOP_ON_KILL_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GENERIC_A {
    #[doc = "0: `0`"]
    DIVBY1 = 0,
    #[doc = "1: `1`"]
    DIVBY2 = 1,
    #[doc = "2: `10`"]
    DIVBY4 = 2,
    #[doc = "3: `11`"]
    DIVBY8 = 3,
    #[doc = "4: `100`"]
    DIVBY16 = 4,
    #[doc = "5: `101`"]
    DIVBY32 = 5,
    #[doc = "6: `110`"]
    DIVBY64 = 6,
    #[doc = "7: `111`"]
    DIVBY128 = 7,
}
impl From<GENERIC_A> for u8 {
    #[inline(always)]
    fn from(variant: GENERIC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GENERIC` reader - "]
pub struct GENERIC_R(crate::FieldReader<u8, GENERIC_A>);
impl GENERIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERIC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENERIC_A> {
        match self.bits {
            0 => Some(GENERIC_A::DIVBY1),
            1 => Some(GENERIC_A::DIVBY2),
            2 => Some(GENERIC_A::DIVBY4),
            3 => Some(GENERIC_A::DIVBY8),
            4 => Some(GENERIC_A::DIVBY16),
            5 => Some(GENERIC_A::DIVBY32),
            6 => Some(GENERIC_A::DIVBY64),
            7 => Some(GENERIC_A::DIVBY128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIVBY1`"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        **self == GENERIC_A::DIVBY1
    }
    #[doc = "Checks if the value of the field is `DIVBY2`"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        **self == GENERIC_A::DIVBY2
    }
    #[doc = "Checks if the value of the field is `DIVBY4`"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        **self == GENERIC_A::DIVBY4
    }
    #[doc = "Checks if the value of the field is `DIVBY8`"]
    #[inline(always)]
    pub fn is_divby8(&self) -> bool {
        **self == GENERIC_A::DIVBY8
    }
    #[doc = "Checks if the value of the field is `DIVBY16`"]
    #[inline(always)]
    pub fn is_divby16(&self) -> bool {
        **self == GENERIC_A::DIVBY16
    }
    #[doc = "Checks if the value of the field is `DIVBY32`"]
    #[inline(always)]
    pub fn is_divby32(&self) -> bool {
        **self == GENERIC_A::DIVBY32
    }
    #[doc = "Checks if the value of the field is `DIVBY64`"]
    #[inline(always)]
    pub fn is_divby64(&self) -> bool {
        **self == GENERIC_A::DIVBY64
    }
    #[doc = "Checks if the value of the field is `DIVBY128`"]
    #[inline(always)]
    pub fn is_divby128(&self) -> bool {
        **self == GENERIC_A::DIVBY128
    }
}
impl core::ops::Deref for GENERIC_R {
    type Target = crate::FieldReader<u8, GENERIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENERIC` writer - "]
pub struct GENERIC_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GENERIC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn divby8(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY8)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn divby16(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY16)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn divby32(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY32)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn divby64(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY64)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn divby128(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UP_DOWN_MODE_A {
    #[doc = "0: `0`"]
    COUNT_UP = 0,
    #[doc = "1: `1`"]
    COUNT_DOWN = 1,
    #[doc = "2: `10`"]
    COUNT_UPDN1 = 2,
    #[doc = "3: `11`"]
    COUNT_UPDN2 = 3,
}
impl From<UP_DOWN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UP_DOWN_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UP_DOWN_MODE` reader - "]
pub struct UP_DOWN_MODE_R(crate::FieldReader<u8, UP_DOWN_MODE_A>);
impl UP_DOWN_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UP_DOWN_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_DOWN_MODE_A {
        match self.bits {
            0 => UP_DOWN_MODE_A::COUNT_UP,
            1 => UP_DOWN_MODE_A::COUNT_DOWN,
            2 => UP_DOWN_MODE_A::COUNT_UPDN1,
            3 => UP_DOWN_MODE_A::COUNT_UPDN2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT_UP`"]
    #[inline(always)]
    pub fn is_count_up(&self) -> bool {
        **self == UP_DOWN_MODE_A::COUNT_UP
    }
    #[doc = "Checks if the value of the field is `COUNT_DOWN`"]
    #[inline(always)]
    pub fn is_count_down(&self) -> bool {
        **self == UP_DOWN_MODE_A::COUNT_DOWN
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN1`"]
    #[inline(always)]
    pub fn is_count_updn1(&self) -> bool {
        **self == UP_DOWN_MODE_A::COUNT_UPDN1
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN2`"]
    #[inline(always)]
    pub fn is_count_updn2(&self) -> bool {
        **self == UP_DOWN_MODE_A::COUNT_UPDN2
    }
}
impl core::ops::Deref for UP_DOWN_MODE_R {
    type Target = crate::FieldReader<u8, UP_DOWN_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UP_DOWN_MODE` writer - "]
pub struct UP_DOWN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_DOWN_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UP_DOWN_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn count_up(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn count_down(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_DOWN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn count_updn1(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn count_updn2(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ONE_SHOT` reader - "]
pub struct ONE_SHOT_R(crate::FieldReader<bool, bool>);
impl ONE_SHOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONE_SHOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONE_SHOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONE_SHOT` writer - "]
pub struct ONE_SHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_SHOT_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QUADRATURE_MODE_A {
    #[doc = "0: `0`"]
    X1 = 0,
    #[doc = "1: `1`"]
    X2 = 1,
    #[doc = "2: `10`"]
    X4 = 2,
}
impl From<QUADRATURE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: QUADRATURE_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QUADRATURE_MODE` reader - "]
pub struct QUADRATURE_MODE_R(crate::FieldReader<u8, QUADRATURE_MODE_A>);
impl QUADRATURE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QUADRATURE_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<QUADRATURE_MODE_A> {
        match self.bits {
            0 => Some(QUADRATURE_MODE_A::X1),
            1 => Some(QUADRATURE_MODE_A::X2),
            2 => Some(QUADRATURE_MODE_A::X4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        **self == QUADRATURE_MODE_A::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        **self == QUADRATURE_MODE_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        **self == QUADRATURE_MODE_A::X4
    }
}
impl core::ops::Deref for QUADRATURE_MODE_R {
    type Target = crate::FieldReader<u8, QUADRATURE_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUADRATURE_MODE` writer - "]
pub struct QUADRATURE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADRATURE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADRATURE_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    TIMER = 0,
    #[doc = "2: `10`"]
    CAPTURE = 2,
    #[doc = "3: `11`"]
    QUAD = 3,
    #[doc = "4: `100`"]
    PWM = 4,
    #[doc = "5: `101`"]
    PWM_DT = 5,
    #[doc = "6: `110`"]
    PWM_PR = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - "]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::TIMER),
            2 => Some(MODE_A::CAPTURE),
            3 => Some(MODE_A::QUAD),
            4 => Some(MODE_A::PWM),
            5 => Some(MODE_A::PWM_DT),
            6 => Some(MODE_A::PWM_PR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        **self == MODE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        **self == MODE_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        **self == MODE_A::QUAD
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        **self == MODE_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_DT`"]
    #[inline(always)]
    pub fn is_pwm_dt(&self) -> bool {
        **self == MODE_A::PWM_DT
    }
    #[doc = "Checks if the value of the field is `PWM_PR`"]
    #[inline(always)]
    pub fn is_pwm_pr(&self) -> bool {
        **self == MODE_A::PWM_PR
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - "]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(MODE_A::TIMER)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(MODE_A::CAPTURE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(MODE_A::QUAD)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE_A::PWM)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm_dt(self) -> &'a mut W {
        self.variant(MODE_A::PWM_DT)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pwm_pr(self) -> &'a mut W {
        self.variant(MODE_A::PWM_PR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn auto_reload_cc(&self) -> AUTO_RELOAD_CC_R {
        AUTO_RELOAD_CC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn auto_reload_period(&self) -> AUTO_RELOAD_PERIOD_R {
        AUTO_RELOAD_PERIOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_sync_kill(&self) -> PWM_SYNC_KILL_R {
        PWM_SYNC_KILL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&self) -> PWM_STOP_ON_KILL_R {
        PWM_STOP_ON_KILL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn generic(&self) -> GENERIC_R {
        GENERIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn up_down_mode(&self) -> UP_DOWN_MODE_R {
        UP_DOWN_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn one_shot(&self) -> ONE_SHOT_R {
        ONE_SHOT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn quadrature_mode(&self) -> QUADRATURE_MODE_R {
        QUADRATURE_MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn auto_reload_cc(&mut self) -> AUTO_RELOAD_CC_W {
        AUTO_RELOAD_CC_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn auto_reload_period(&mut self) -> AUTO_RELOAD_PERIOD_W {
        AUTO_RELOAD_PERIOD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_sync_kill(&mut self) -> PWM_SYNC_KILL_W {
        PWM_SYNC_KILL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&mut self) -> PWM_STOP_ON_KILL_W {
        PWM_STOP_ON_KILL_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn generic(&mut self) -> GENERIC_W {
        GENERIC_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn up_down_mode(&mut self) -> UP_DOWN_MODE_W {
        UP_DOWN_MODE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn one_shot(&mut self) -> ONE_SHOT_W {
        ONE_SHOT_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn quadrature_mode(&mut self) -> QUADRATURE_MODE_W {
        QUADRATURE_MODE_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
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
