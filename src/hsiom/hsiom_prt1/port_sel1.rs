#[doc = "Register `PORT_SEL1` reader"]
pub struct R(crate::R<PORT_SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_SEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_SEL1` writer"]
pub struct W(crate::W<PORT_SEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SEL1_SPEC>;
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
impl From<crate::W<PORT_SEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_SEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO0_SEL_A {
    #[doc = "8: `1000`"]
    TCPWM0_LINE = 8,
    #[doc = "9: `1001`"]
    TCPWM0_TR_COMPARE_MATCH = 9,
    #[doc = "10: `1010`"]
    SCB1_UART_TX = 10,
    #[doc = "11: `1011`"]
    TCPWM0_TR_OVERFLOW = 11,
    #[doc = "12: `1100`"]
    USBPD0_TX_DATA_EN = 12,
    #[doc = "14: `1110`"]
    SCB1_SPI_CLK = 14,
    #[doc = "15: `1111`"]
    SCB1_I2C_SCL = 15,
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
            8 => Some(IO0_SEL_A::TCPWM0_LINE),
            9 => Some(IO0_SEL_A::TCPWM0_TR_COMPARE_MATCH),
            10 => Some(IO0_SEL_A::SCB1_UART_TX),
            11 => Some(IO0_SEL_A::TCPWM0_TR_OVERFLOW),
            12 => Some(IO0_SEL_A::USBPD0_TX_DATA_EN),
            14 => Some(IO0_SEL_A::SCB1_SPI_CLK),
            15 => Some(IO0_SEL_A::SCB1_I2C_SCL),
            0 => Some(IO0_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCPWM0_LINE`"]
    #[inline(always)]
    pub fn is_tcpwm0_line(&self) -> bool {
        **self == IO0_SEL_A::TCPWM0_LINE
    }
    #[doc = "Checks if the value of the field is `TCPWM0_TR_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_tcpwm0_tr_compare_match(&self) -> bool {
        **self == IO0_SEL_A::TCPWM0_TR_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `SCB1_UART_TX`"]
    #[inline(always)]
    pub fn is_scb1_uart_tx(&self) -> bool {
        **self == IO0_SEL_A::SCB1_UART_TX
    }
    #[doc = "Checks if the value of the field is `TCPWM0_TR_OVERFLOW`"]
    #[inline(always)]
    pub fn is_tcpwm0_tr_overflow(&self) -> bool {
        **self == IO0_SEL_A::TCPWM0_TR_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `USBPD0_TX_DATA_EN`"]
    #[inline(always)]
    pub fn is_usbpd0_tx_data_en(&self) -> bool {
        **self == IO0_SEL_A::USBPD0_TX_DATA_EN
    }
    #[doc = "Checks if the value of the field is `SCB1_SPI_CLK`"]
    #[inline(always)]
    pub fn is_scb1_spi_clk(&self) -> bool {
        **self == IO0_SEL_A::SCB1_SPI_CLK
    }
    #[doc = "Checks if the value of the field is `SCB1_I2C_SCL`"]
    #[inline(always)]
    pub fn is_scb1_i2c_scl(&self) -> bool {
        **self == IO0_SEL_A::SCB1_I2C_SCL
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
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn tcpwm0_line(self) -> &'a mut W {
        self.variant(IO0_SEL_A::TCPWM0_LINE)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn tcpwm0_tr_compare_match(self) -> &'a mut W {
        self.variant(IO0_SEL_A::TCPWM0_TR_COMPARE_MATCH)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn scb1_uart_tx(self) -> &'a mut W {
        self.variant(IO0_SEL_A::SCB1_UART_TX)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn tcpwm0_tr_overflow(self) -> &'a mut W {
        self.variant(IO0_SEL_A::TCPWM0_TR_OVERFLOW)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn usbpd0_tx_data_en(self) -> &'a mut W {
        self.variant(IO0_SEL_A::USBPD0_TX_DATA_EN)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn scb1_spi_clk(self) -> &'a mut W {
        self.variant(IO0_SEL_A::SCB1_SPI_CLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn scb1_i2c_scl(self) -> &'a mut W {
        self.variant(IO0_SEL_A::SCB1_I2C_SCL)
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
    TCPWM1_LINE = 8,
    #[doc = "9: `1001`"]
    TCPWM1_TR_COMPARE_MATCH = 9,
    #[doc = "10: `1010`"]
    SCB1_UART_CTS = 10,
    #[doc = "11: `1011`"]
    TCPWM1_TR_OVERFLOW = 11,
    #[doc = "12: `1100`"]
    CPUSS0_SWD_DATA = 12,
    #[doc = "14: `1110`"]
    SCB1_SPI_MOSI = 14,
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
            8 => Some(IO1_SEL_A::TCPWM1_LINE),
            9 => Some(IO1_SEL_A::TCPWM1_TR_COMPARE_MATCH),
            10 => Some(IO1_SEL_A::SCB1_UART_CTS),
            11 => Some(IO1_SEL_A::TCPWM1_TR_OVERFLOW),
            12 => Some(IO1_SEL_A::CPUSS0_SWD_DATA),
            14 => Some(IO1_SEL_A::SCB1_SPI_MOSI),
            0 => Some(IO1_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCPWM1_LINE`"]
    #[inline(always)]
    pub fn is_tcpwm1_line(&self) -> bool {
        **self == IO1_SEL_A::TCPWM1_LINE
    }
    #[doc = "Checks if the value of the field is `TCPWM1_TR_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_tcpwm1_tr_compare_match(&self) -> bool {
        **self == IO1_SEL_A::TCPWM1_TR_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `SCB1_UART_CTS`"]
    #[inline(always)]
    pub fn is_scb1_uart_cts(&self) -> bool {
        **self == IO1_SEL_A::SCB1_UART_CTS
    }
    #[doc = "Checks if the value of the field is `TCPWM1_TR_OVERFLOW`"]
    #[inline(always)]
    pub fn is_tcpwm1_tr_overflow(&self) -> bool {
        **self == IO1_SEL_A::TCPWM1_TR_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `CPUSS0_SWD_DATA`"]
    #[inline(always)]
    pub fn is_cpuss0_swd_data(&self) -> bool {
        **self == IO1_SEL_A::CPUSS0_SWD_DATA
    }
    #[doc = "Checks if the value of the field is `SCB1_SPI_MOSI`"]
    #[inline(always)]
    pub fn is_scb1_spi_mosi(&self) -> bool {
        **self == IO1_SEL_A::SCB1_SPI_MOSI
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
    pub fn tcpwm1_line(self) -> &'a mut W {
        self.variant(IO1_SEL_A::TCPWM1_LINE)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn tcpwm1_tr_compare_match(self) -> &'a mut W {
        self.variant(IO1_SEL_A::TCPWM1_TR_COMPARE_MATCH)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn scb1_uart_cts(self) -> &'a mut W {
        self.variant(IO1_SEL_A::SCB1_UART_CTS)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn tcpwm1_tr_overflow(self) -> &'a mut W {
        self.variant(IO1_SEL_A::TCPWM1_TR_OVERFLOW)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn cpuss0_swd_data(self) -> &'a mut W {
        self.variant(IO1_SEL_A::CPUSS0_SWD_DATA)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn scb1_spi_mosi(self) -> &'a mut W {
        self.variant(IO1_SEL_A::SCB1_SPI_MOSI)
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
    #[doc = "8: `1000`"]
    TCPWM2_LINE = 8,
    #[doc = "9: `1001`"]
    TCPWM2_TR_COMPARE_MATCH = 9,
    #[doc = "11: `1011`"]
    TCPWM2_TR_OVERFLOW = 11,
    #[doc = "12: `1100`"]
    CPUSS0_SWD_CLK = 12,
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
            8 => Some(IO2_SEL_A::TCPWM2_LINE),
            9 => Some(IO2_SEL_A::TCPWM2_TR_COMPARE_MATCH),
            11 => Some(IO2_SEL_A::TCPWM2_TR_OVERFLOW),
            12 => Some(IO2_SEL_A::CPUSS0_SWD_CLK),
            0 => Some(IO2_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCPWM2_LINE`"]
    #[inline(always)]
    pub fn is_tcpwm2_line(&self) -> bool {
        **self == IO2_SEL_A::TCPWM2_LINE
    }
    #[doc = "Checks if the value of the field is `TCPWM2_TR_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_tcpwm2_tr_compare_match(&self) -> bool {
        **self == IO2_SEL_A::TCPWM2_TR_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `TCPWM2_TR_OVERFLOW`"]
    #[inline(always)]
    pub fn is_tcpwm2_tr_overflow(&self) -> bool {
        **self == IO2_SEL_A::TCPWM2_TR_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `CPUSS0_SWD_CLK`"]
    #[inline(always)]
    pub fn is_cpuss0_swd_clk(&self) -> bool {
        **self == IO2_SEL_A::CPUSS0_SWD_CLK
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
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn tcpwm2_line(self) -> &'a mut W {
        self.variant(IO2_SEL_A::TCPWM2_LINE)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn tcpwm2_tr_compare_match(self) -> &'a mut W {
        self.variant(IO2_SEL_A::TCPWM2_TR_COMPARE_MATCH)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn tcpwm2_tr_overflow(self) -> &'a mut W {
        self.variant(IO2_SEL_A::TCPWM2_TR_OVERFLOW)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn cpuss0_swd_clk(self) -> &'a mut W {
        self.variant(IO2_SEL_A::CPUSS0_SWD_CLK)
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
    #[doc = "8: `1000`"]
    SRSS0_EXT_CLK = 8,
    #[doc = "10: `1010`"]
    SCB1_UART_RX = 10,
    #[doc = "13: `1101`"]
    SCB0_SPI_SELECT0 = 13,
    #[doc = "14: `1110`"]
    SCB1_SPI_MISO = 14,
    #[doc = "15: `1111`"]
    SCB1_I2C_SDA = 15,
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
            8 => Some(IO3_SEL_A::SRSS0_EXT_CLK),
            10 => Some(IO3_SEL_A::SCB1_UART_RX),
            13 => Some(IO3_SEL_A::SCB0_SPI_SELECT0),
            14 => Some(IO3_SEL_A::SCB1_SPI_MISO),
            15 => Some(IO3_SEL_A::SCB1_I2C_SDA),
            0 => Some(IO3_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SRSS0_EXT_CLK`"]
    #[inline(always)]
    pub fn is_srss0_ext_clk(&self) -> bool {
        **self == IO3_SEL_A::SRSS0_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `SCB1_UART_RX`"]
    #[inline(always)]
    pub fn is_scb1_uart_rx(&self) -> bool {
        **self == IO3_SEL_A::SCB1_UART_RX
    }
    #[doc = "Checks if the value of the field is `SCB0_SPI_SELECT0`"]
    #[inline(always)]
    pub fn is_scb0_spi_select0(&self) -> bool {
        **self == IO3_SEL_A::SCB0_SPI_SELECT0
    }
    #[doc = "Checks if the value of the field is `SCB1_SPI_MISO`"]
    #[inline(always)]
    pub fn is_scb1_spi_miso(&self) -> bool {
        **self == IO3_SEL_A::SCB1_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `SCB1_I2C_SDA`"]
    #[inline(always)]
    pub fn is_scb1_i2c_sda(&self) -> bool {
        **self == IO3_SEL_A::SCB1_I2C_SDA
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
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn srss0_ext_clk(self) -> &'a mut W {
        self.variant(IO3_SEL_A::SRSS0_EXT_CLK)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn scb1_uart_rx(self) -> &'a mut W {
        self.variant(IO3_SEL_A::SCB1_UART_RX)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn scb0_spi_select0(self) -> &'a mut W {
        self.variant(IO3_SEL_A::SCB0_SPI_SELECT0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn scb1_spi_miso(self) -> &'a mut W {
        self.variant(IO3_SEL_A::SCB1_SPI_MISO)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn scb1_i2c_sda(self) -> &'a mut W {
        self.variant(IO3_SEL_A::SCB1_I2C_SDA)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO4_SEL_A {
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO4_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO4_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO4_SEL` reader - "]
pub struct IO4_SEL_R(crate::FieldReader<u8, IO4_SEL_A>);
impl IO4_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO4_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO4_SEL_A> {
        match self.bits {
            0 => Some(IO4_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO4_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO4_SEL_R {
    type Target = crate::FieldReader<u8, IO4_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO4_SEL` writer - "]
pub struct IO4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO4_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO4_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO4_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO5_SEL_A {
    #[doc = "8: `1000`"]
    TCPWM3_LINE = 8,
    #[doc = "9: `1001`"]
    TCPWM3_TR_COMPARE_MATCH = 9,
    #[doc = "10: `1010`"]
    SCB1_UART_RTS = 10,
    #[doc = "11: `1011`"]
    TCPWM3_TR_OVERFLOW = 11,
    #[doc = "12: `1100`"]
    USBPD0_CMP_OUT = 12,
    #[doc = "13: `1101`"]
    SCB0_SPI_MISO = 13,
    #[doc = "14: `1110`"]
    SCB1_SPI_SELECT0 = 14,
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO5_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO5_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO5_SEL` reader - "]
pub struct IO5_SEL_R(crate::FieldReader<u8, IO5_SEL_A>);
impl IO5_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO5_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO5_SEL_A> {
        match self.bits {
            8 => Some(IO5_SEL_A::TCPWM3_LINE),
            9 => Some(IO5_SEL_A::TCPWM3_TR_COMPARE_MATCH),
            10 => Some(IO5_SEL_A::SCB1_UART_RTS),
            11 => Some(IO5_SEL_A::TCPWM3_TR_OVERFLOW),
            12 => Some(IO5_SEL_A::USBPD0_CMP_OUT),
            13 => Some(IO5_SEL_A::SCB0_SPI_MISO),
            14 => Some(IO5_SEL_A::SCB1_SPI_SELECT0),
            0 => Some(IO5_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCPWM3_LINE`"]
    #[inline(always)]
    pub fn is_tcpwm3_line(&self) -> bool {
        **self == IO5_SEL_A::TCPWM3_LINE
    }
    #[doc = "Checks if the value of the field is `TCPWM3_TR_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_tcpwm3_tr_compare_match(&self) -> bool {
        **self == IO5_SEL_A::TCPWM3_TR_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `SCB1_UART_RTS`"]
    #[inline(always)]
    pub fn is_scb1_uart_rts(&self) -> bool {
        **self == IO5_SEL_A::SCB1_UART_RTS
    }
    #[doc = "Checks if the value of the field is `TCPWM3_TR_OVERFLOW`"]
    #[inline(always)]
    pub fn is_tcpwm3_tr_overflow(&self) -> bool {
        **self == IO5_SEL_A::TCPWM3_TR_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `USBPD0_CMP_OUT`"]
    #[inline(always)]
    pub fn is_usbpd0_cmp_out(&self) -> bool {
        **self == IO5_SEL_A::USBPD0_CMP_OUT
    }
    #[doc = "Checks if the value of the field is `SCB0_SPI_MISO`"]
    #[inline(always)]
    pub fn is_scb0_spi_miso(&self) -> bool {
        **self == IO5_SEL_A::SCB0_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `SCB1_SPI_SELECT0`"]
    #[inline(always)]
    pub fn is_scb1_spi_select0(&self) -> bool {
        **self == IO5_SEL_A::SCB1_SPI_SELECT0
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO5_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO5_SEL_R {
    type Target = crate::FieldReader<u8, IO5_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO5_SEL` writer - "]
pub struct IO5_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO5_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO5_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn tcpwm3_line(self) -> &'a mut W {
        self.variant(IO5_SEL_A::TCPWM3_LINE)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn tcpwm3_tr_compare_match(self) -> &'a mut W {
        self.variant(IO5_SEL_A::TCPWM3_TR_COMPARE_MATCH)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn scb1_uart_rts(self) -> &'a mut W {
        self.variant(IO5_SEL_A::SCB1_UART_RTS)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn tcpwm3_tr_overflow(self) -> &'a mut W {
        self.variant(IO5_SEL_A::TCPWM3_TR_OVERFLOW)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn usbpd0_cmp_out(self) -> &'a mut W {
        self.variant(IO5_SEL_A::USBPD0_CMP_OUT)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn scb0_spi_miso(self) -> &'a mut W {
        self.variant(IO5_SEL_A::SCB0_SPI_MISO)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn scb1_spi_select0(self) -> &'a mut W {
        self.variant(IO5_SEL_A::SCB1_SPI_SELECT0)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO5_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO6_SEL_A {
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO6_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO6_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO6_SEL` reader - "]
pub struct IO6_SEL_R(crate::FieldReader<u8, IO6_SEL_A>);
impl IO6_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO6_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO6_SEL_A> {
        match self.bits {
            0 => Some(IO6_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO6_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO6_SEL_R {
    type Target = crate::FieldReader<u8, IO6_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO6_SEL` writer - "]
pub struct IO6_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO6_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO6_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO6_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO7_SEL_A {
    #[doc = "8: `1000`"]
    TCPWM4_LINE = 8,
    #[doc = "9: `1001`"]
    TCPWM4_TR_COMPARE_MATCH = 9,
    #[doc = "10: `1010`"]
    SCB0_UART_TX = 10,
    #[doc = "11: `1011`"]
    TCPWM4_TR_OVERFLOW = 11,
    #[doc = "12: `1100`"]
    USBPD0_TX_DATA = 12,
    #[doc = "13: `1101`"]
    SCB0_SPI_MOSI = 13,
    #[doc = "15: `1111`"]
    SCB0_I2C_SDA = 15,
    #[doc = "0: `0`"]
    GPIO = 0,
}
impl From<IO7_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO7_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO7_SEL` reader - "]
pub struct IO7_SEL_R(crate::FieldReader<u8, IO7_SEL_A>);
impl IO7_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO7_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IO7_SEL_A> {
        match self.bits {
            8 => Some(IO7_SEL_A::TCPWM4_LINE),
            9 => Some(IO7_SEL_A::TCPWM4_TR_COMPARE_MATCH),
            10 => Some(IO7_SEL_A::SCB0_UART_TX),
            11 => Some(IO7_SEL_A::TCPWM4_TR_OVERFLOW),
            12 => Some(IO7_SEL_A::USBPD0_TX_DATA),
            13 => Some(IO7_SEL_A::SCB0_SPI_MOSI),
            15 => Some(IO7_SEL_A::SCB0_I2C_SDA),
            0 => Some(IO7_SEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCPWM4_LINE`"]
    #[inline(always)]
    pub fn is_tcpwm4_line(&self) -> bool {
        **self == IO7_SEL_A::TCPWM4_LINE
    }
    #[doc = "Checks if the value of the field is `TCPWM4_TR_COMPARE_MATCH`"]
    #[inline(always)]
    pub fn is_tcpwm4_tr_compare_match(&self) -> bool {
        **self == IO7_SEL_A::TCPWM4_TR_COMPARE_MATCH
    }
    #[doc = "Checks if the value of the field is `SCB0_UART_TX`"]
    #[inline(always)]
    pub fn is_scb0_uart_tx(&self) -> bool {
        **self == IO7_SEL_A::SCB0_UART_TX
    }
    #[doc = "Checks if the value of the field is `TCPWM4_TR_OVERFLOW`"]
    #[inline(always)]
    pub fn is_tcpwm4_tr_overflow(&self) -> bool {
        **self == IO7_SEL_A::TCPWM4_TR_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `USBPD0_TX_DATA`"]
    #[inline(always)]
    pub fn is_usbpd0_tx_data(&self) -> bool {
        **self == IO7_SEL_A::USBPD0_TX_DATA
    }
    #[doc = "Checks if the value of the field is `SCB0_SPI_MOSI`"]
    #[inline(always)]
    pub fn is_scb0_spi_mosi(&self) -> bool {
        **self == IO7_SEL_A::SCB0_SPI_MOSI
    }
    #[doc = "Checks if the value of the field is `SCB0_I2C_SDA`"]
    #[inline(always)]
    pub fn is_scb0_i2c_sda(&self) -> bool {
        **self == IO7_SEL_A::SCB0_I2C_SDA
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO7_SEL_A::GPIO
    }
}
impl core::ops::Deref for IO7_SEL_R {
    type Target = crate::FieldReader<u8, IO7_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO7_SEL` writer - "]
pub struct IO7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO7_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn tcpwm4_line(self) -> &'a mut W {
        self.variant(IO7_SEL_A::TCPWM4_LINE)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn tcpwm4_tr_compare_match(self) -> &'a mut W {
        self.variant(IO7_SEL_A::TCPWM4_TR_COMPARE_MATCH)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn scb0_uart_tx(self) -> &'a mut W {
        self.variant(IO7_SEL_A::SCB0_UART_TX)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn tcpwm4_tr_overflow(self) -> &'a mut W {
        self.variant(IO7_SEL_A::TCPWM4_TR_OVERFLOW)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn usbpd0_tx_data(self) -> &'a mut W {
        self.variant(IO7_SEL_A::USBPD0_TX_DATA)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn scb0_spi_mosi(self) -> &'a mut W {
        self.variant(IO7_SEL_A::SCB0_SPI_MOSI)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn scb0_i2c_sda(self) -> &'a mut W {
        self.variant(IO7_SEL_A::SCB0_I2C_SDA)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO7_SEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
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
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn io4_sel(&self) -> IO4_SEL_R {
        IO4_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn io5_sel(&self) -> IO5_SEL_R {
        IO5_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn io6_sel(&self) -> IO6_SEL_R {
        IO6_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn io7_sel(&self) -> IO7_SEL_R {
        IO7_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn io4_sel(&mut self) -> IO4_SEL_W {
        IO4_SEL_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn io5_sel(&mut self) -> IO5_SEL_W {
        IO5_SEL_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn io6_sel(&mut self) -> IO6_SEL_W {
        IO6_SEL_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn io7_sel(&mut self) -> IO7_SEL_W {
        IO7_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel1](index.html) module"]
pub struct PORT_SEL1_SPEC;
impl crate::RegisterSpec for PORT_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_sel1::R](R) reader structure"]
impl crate::Readable for PORT_SEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_sel1::W](W) writer structure"]
impl crate::Writable for PORT_SEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_SEL1 to value 0"]
impl crate::Resettable for PORT_SEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
