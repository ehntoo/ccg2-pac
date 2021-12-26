#[doc = "Register `UART_CTRL` reader"]
pub struct R(crate::R<UART_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_CTRL` writer"]
pub struct W(crate::W<UART_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CTRL_SPEC>;
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
impl From<crate::W<UART_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOPBACK` reader - "]
pub struct LOOPBACK_R(crate::FieldReader<bool, bool>);
impl LOOPBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPBACK` writer - "]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    UART_STD = 0,
    #[doc = "1: `1`"]
    UART_SMARTCARD = 1,
    #[doc = "2: `10`"]
    UART_IRDA = 2,
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
            0 => Some(MODE_A::UART_STD),
            1 => Some(MODE_A::UART_SMARTCARD),
            2 => Some(MODE_A::UART_IRDA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART_STD`"]
    #[inline(always)]
    pub fn is_uart_std(&self) -> bool {
        **self == MODE_A::UART_STD
    }
    #[doc = "Checks if the value of the field is `UART_SMARTCARD`"]
    #[inline(always)]
    pub fn is_uart_smartcard(&self) -> bool {
        **self == MODE_A::UART_SMARTCARD
    }
    #[doc = "Checks if the value of the field is `UART_IRDA`"]
    #[inline(always)]
    pub fn is_uart_irda(&self) -> bool {
        **self == MODE_A::UART_IRDA
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
    pub fn uart_std(self) -> &'a mut W {
        self.variant(MODE_A::UART_STD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn uart_smartcard(self) -> &'a mut W {
        self.variant(MODE_A::UART_SMARTCARD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart_irda(self) -> &'a mut W {
        self.variant(MODE_A::UART_IRDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bits 24:25"]
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ctrl](index.html) module"]
pub struct UART_CTRL_SPEC;
impl crate::RegisterSpec for UART_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ctrl::R](R) reader structure"]
impl crate::Readable for UART_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ctrl::W](W) writer structure"]
impl crate::Writable for UART_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_CTRL to value 0x0300_0000"]
impl crate::Resettable for UART_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}
