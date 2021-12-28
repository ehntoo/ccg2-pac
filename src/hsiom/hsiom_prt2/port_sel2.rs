#[doc = "Register `PORT_SEL2` reader"]
pub struct R(crate::R<PORT_SEL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_SEL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_SEL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_SEL2` writer"]
pub struct W(crate::W<PORT_SEL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SEL2_SPEC>;
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
impl From<crate::W<PORT_SEL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_SEL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO0_SEL_A {
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO0_SEL` reader - "]
pub struct IO0_SEL_R(crate::FieldReader<u8, IO0_SEL_A>);
impl IO0_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO0_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO0_SEL_A> {
        match self.bits {
            0 => Some(IO0_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO0_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO0_SEL_R {
    type Target = crate::FieldReader<u8, IO0_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO0_SEL` writer - "]
pub struct IO0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO0_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO1_SEL_A {
    #[doc = "8: `1000`"]
    TCPWM5_LINE = 8,
    #[doc = "9: `1001`"]
    TCPWM5_TR_COMPARE_MATCH = 9,
    #[doc = "10: `1010`"]
    SCB0_UART_RX = 10,
    #[doc = "11: `1011`"]
    TCPWM5_TR_OVERFLOW = 11,
    #[doc = "12: `1100`"]
    USBPD0_CMP_OUT = 12,
    #[doc = "13: `1101`"]
    SCB0_SPI_CLK = 13,
    #[doc = "15: `1111`"]
    SCB0_I2C_SCL = 15,
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO1_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO1_SEL` reader - "]
pub struct IO1_SEL_R(crate::FieldReader<u8, IO1_SEL_A>);
impl IO1_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO1_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO1_SEL_A> {
        match self.bits {
            8 => Some(IO1_SEL_A::TCPWM5_LINE),
            9 => Some(IO1_SEL_A::TCPWM5_TR_COMPARE_MATCH),
            10 => Some(IO1_SEL_A::SCB0_UART_RX),
            11 => Some(IO1_SEL_A::TCPWM5_TR_OVERFLOW),
            12 => Some(IO1_SEL_A::USBPD0_CMP_OUT),
            13 => Some(IO1_SEL_A::SCB0_SPI_CLK),
            15 => Some(IO1_SEL_A::SCB0_I2C_SCL),
            0 => Some(IO1_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCPWM5_LINE`"]
    #[inline(always)]
    pub fn is_tcpwm5_line(&self) -> bool {
        **self == IO1_SEL_A::TCPWM5_LINE
    }
    #[doc = "Checks if the value of the field is `TCPWM5_TR_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_tcpwm5_tr_compare_match(&self) -> bool {
        **self == IO1_SEL_A::TCPWM5_TR_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `SCB0_UART_RX`"]
    #[inline(always)]
    pub fn is_scb0_uart_rx(&self) -> bool {
        **self == IO1_SEL_A::SCB0_UART_RX
    }
    #[doc = "Checks if the value of the field is `TCPWM5_TR_OVERFLOW`"]
    #[inline(always)]
    pub fn is_tcpwm5_tr_overflow(&self) -> bool {
        **self == IO1_SEL_A::TCPWM5_TR_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `USBPD0_CMP_OUT`"]
    #[inline(always)]
    pub fn is_usbpd0_cmp_out(&self) -> bool {
        **self == IO1_SEL_A::USBPD0_CMP_OUT
    }
    #[doc = "Checks if the value of the field is `SCB0_SPI_CLK`"]
    #[inline(always)]
    pub fn is_scb0_spi_clk(&self) -> bool {
        **self == IO1_SEL_A::SCB0_SPI_CLK
    }
    #[doc = "Checks if the value of the field is `SCB0_I2C_SCL`"]
    #[inline(always)]
    pub fn is_scb0_i2c_scl(&self) -> bool {
        **self == IO1_SEL_A::SCB0_I2C_SCL
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO1_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO1_SEL_R {
    type Target = crate::FieldReader<u8, IO1_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO1_SEL` writer - "]
pub struct IO1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO1_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn tcpwm5_line(self) -> &'a mut W {
        self.variant(IO1_SEL_A::TCPWM5_LINE)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn tcpwm5_tr_compare_match(self) -> &'a mut W {
        self.variant(IO1_SEL_A::TCPWM5_TR_COMPARE_MATCH)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn scb0_uart_rx(self) -> &'a mut W {
        self.variant(IO1_SEL_A::SCB0_UART_RX)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn tcpwm5_tr_overflow(self) -> &'a mut W {
        self.variant(IO1_SEL_A::TCPWM5_TR_OVERFLOW)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn usbpd0_cmp_out(self) -> &'a mut W {
        self.variant(IO1_SEL_A::USBPD0_CMP_OUT)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn scb0_spi_clk(self) -> &'a mut W {
        self.variant(IO1_SEL_A::SCB0_SPI_CLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn scb0_i2c_scl(self) -> &'a mut W {
        self.variant(IO1_SEL_A::SCB0_I2C_SCL)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO1_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO2_SEL_A {
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO2_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO2_SEL` reader - "]
pub struct IO2_SEL_R(crate::FieldReader<u8, IO2_SEL_A>);
impl IO2_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO2_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO2_SEL_A> {
        match self.bits {
            0 => Some(IO2_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO2_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO2_SEL_R {
    type Target = crate::FieldReader<u8, IO2_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO2_SEL` writer - "]
pub struct IO2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO2_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO2_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO3_SEL_A {
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO3_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO3_SEL` reader - "]
pub struct IO3_SEL_R(crate::FieldReader<u8, IO3_SEL_A>);
impl IO3_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO3_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO3_SEL_A> {
        match self.bits {
            0 => Some(IO3_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO3_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO3_SEL_R {
    type Target = crate::FieldReader<u8, IO3_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO3_SEL` writer - "]
pub struct IO3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO3_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO3_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn io0_sel(&self) -> IO0_SEL_R {
        IO0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn io1_sel(&self) -> IO1_SEL_R {
        IO1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn io2_sel(&self) -> IO2_SEL_R {
        IO2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn io3_sel(&self) -> IO3_SEL_R {
        IO3_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn io0_sel(&mut self) -> IO0_SEL_W {
        IO0_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn io1_sel(&mut self) -> IO1_SEL_W {
        IO1_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn io2_sel(&mut self) -> IO2_SEL_W {
        IO2_SEL_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn io3_sel(&mut self) -> IO3_SEL_W {
        IO3_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel2](index.html) module"]
pub struct PORT_SEL2_SPEC;
impl crate::RegisterSpec for PORT_SEL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_sel2::R](R) reader structure"]
impl crate::Readable for PORT_SEL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_sel2::W](W) writer structure"]
impl crate::Writable for PORT_SEL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_SEL2 to value 0"]
impl crate::Resettable for PORT_SEL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
