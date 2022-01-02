#[doc = "Register `UART_FLOW_CTRL` reader"]
pub struct R(crate::R<UART_FLOW_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FLOW_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FLOW_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FLOW_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_FLOW_CTRL` writer"]
pub struct W(crate::W<UART_FLOW_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FLOW_CTRL_SPEC>;
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
impl From<crate::W<UART_FLOW_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FLOW_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal \"uart_rts_out\" is activated. By setting this field to \"0\", flow control is effectively SW disabled (may be useful for debug purposes)."]
pub struct TRIGGER_LEVEL_R(crate::FieldReader<u8, u8>);
impl TRIGGER_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGGER_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal \"uart_rts_out\" is activated. By setting this field to \"0\", flow control is effectively SW disabled (may be useful for debug purposes)."]
pub struct TRIGGER_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RTS_POLARITY` reader - Polarity of the RTS output signal \"uart_rts_out\": '0': RTS is low/'0' active; \"uart_rts_out\" is '0' when active and \"uart_rts_out\" is '1' when inactive. '1': RTS is high/'1' active; \"uart_rts_out\" is '1' when active and \"uart_rts_out\" is '0' when inactive. During IP reset (Hibernate system power mode), \"uart_rts_out\" is '1'. This represents an inactive state assuming a low/'0' active polarity."]
pub struct RTS_POLARITY_R(crate::FieldReader<bool, bool>);
impl RTS_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTS_POLARITY` writer - Polarity of the RTS output signal \"uart_rts_out\": '0': RTS is low/'0' active; \"uart_rts_out\" is '0' when active and \"uart_rts_out\" is '1' when inactive. '1': RTS is high/'1' active; \"uart_rts_out\" is '1' when active and \"uart_rts_out\" is '0' when inactive. During IP reset (Hibernate system power mode), \"uart_rts_out\" is '1'. This represents an inactive state assuming a low/'0' active polarity."]
pub struct RTS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_POLARITY_W<'a> {
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
#[doc = "Field `CTS_POLARITY` reader - Polarity of the CTS input signal \"uart_cts_in\": '0': CTS is low/'0' active; \"uart_cts_in\" is '0' when active and \"uart_cts_in\" is '1' when inactive. '1': CTS is high/'1' active; \"uart_cts_in\" is '1' when active and \"uart_cts_in\" is '0' when inactive."]
pub struct CTS_POLARITY_R(crate::FieldReader<bool, bool>);
impl CTS_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_POLARITY` writer - Polarity of the CTS input signal \"uart_cts_in\": '0': CTS is low/'0' active; \"uart_cts_in\" is '0' when active and \"uart_cts_in\" is '1' when inactive. '1': CTS is high/'1' active; \"uart_cts_in\" is '1' when active and \"uart_cts_in\" is '0' when inactive."]
pub struct CTS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_POLARITY_W<'a> {
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
#[doc = "Field `CTS_ENABLED` reader - Enable use of CTS input signal \"uart_cts_in\" by the UART transmitter: '0': Disabled. The UART transmitter ignores \"uart_cts_in\", and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses \"uart_cts_in\" to qualify the transmission of data. It transmits when \"uart_cts_in\" is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', \"uart_cts_in\" is connected to \"uart_rts_out\" in the IP (both signals are subjected to signal polarity changes are indicated by RTS_POLARITY and CTS_POLARITY)."]
pub struct CTS_ENABLED_R(crate::FieldReader<bool, bool>);
impl CTS_ENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_ENABLED` writer - Enable use of CTS input signal \"uart_cts_in\" by the UART transmitter: '0': Disabled. The UART transmitter ignores \"uart_cts_in\", and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses \"uart_cts_in\" to qualify the transmission of data. It transmits when \"uart_cts_in\" is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', \"uart_cts_in\" is connected to \"uart_rts_out\" in the IP (both signals are subjected to signal polarity changes are indicated by RTS_POLARITY and CTS_POLARITY)."]
pub struct CTS_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_ENABLED_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal \"uart_rts_out\" is activated. By setting this field to \"0\", flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal \"uart_rts_out\": '0': RTS is low/'0' active; \"uart_rts_out\" is '0' when active and \"uart_rts_out\" is '1' when inactive. '1': RTS is high/'1' active; \"uart_rts_out\" is '1' when active and \"uart_rts_out\" is '0' when inactive. During IP reset (Hibernate system power mode), \"uart_rts_out\" is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    pub fn rts_polarity(&self) -> RTS_POLARITY_R {
        RTS_POLARITY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal \"uart_cts_in\": '0': CTS is low/'0' active; \"uart_cts_in\" is '0' when active and \"uart_cts_in\" is '1' when inactive. '1': CTS is high/'1' active; \"uart_cts_in\" is '1' when active and \"uart_cts_in\" is '0' when inactive."]
    #[inline(always)]
    pub fn cts_polarity(&self) -> CTS_POLARITY_R {
        CTS_POLARITY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal \"uart_cts_in\" by the UART transmitter: '0': Disabled. The UART transmitter ignores \"uart_cts_in\", and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses \"uart_cts_in\" to qualify the transmission of data. It transmits when \"uart_cts_in\" is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', \"uart_cts_in\" is connected to \"uart_rts_out\" in the IP (both signals are subjected to signal polarity changes are indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub fn cts_enabled(&self) -> CTS_ENABLED_R {
        CTS_ENABLED_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal \"uart_rts_out\" is activated. By setting this field to \"0\", flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W {
        TRIGGER_LEVEL_W { w: self }
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal \"uart_rts_out\": '0': RTS is low/'0' active; \"uart_rts_out\" is '0' when active and \"uart_rts_out\" is '1' when inactive. '1': RTS is high/'1' active; \"uart_rts_out\" is '1' when active and \"uart_rts_out\" is '0' when inactive. During IP reset (Hibernate system power mode), \"uart_rts_out\" is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    pub fn rts_polarity(&mut self) -> RTS_POLARITY_W {
        RTS_POLARITY_W { w: self }
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal \"uart_cts_in\": '0': CTS is low/'0' active; \"uart_cts_in\" is '0' when active and \"uart_cts_in\" is '1' when inactive. '1': CTS is high/'1' active; \"uart_cts_in\" is '1' when active and \"uart_cts_in\" is '0' when inactive."]
    #[inline(always)]
    pub fn cts_polarity(&mut self) -> CTS_POLARITY_W {
        CTS_POLARITY_W { w: self }
    }
    #[doc = "Bit 25 - Enable use of CTS input signal \"uart_cts_in\" by the UART transmitter: '0': Disabled. The UART transmitter ignores \"uart_cts_in\", and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses \"uart_cts_in\" to qualify the transmission of data. It transmits when \"uart_cts_in\" is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', \"uart_cts_in\" is connected to \"uart_rts_out\" in the IP (both signals are subjected to signal polarity changes are indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub fn cts_enabled(&mut self) -> CTS_ENABLED_W {
        CTS_ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART flow control register UART flow control is a design time configuration parameter, which make the presence of this MMIO register conditional to the configuration. The \"uart_rts_out\" and \"uart_cts_in\" are always present on the IP interface. If flow control is configured out, \"uart_rts_out\" is NOT connected, and \"uart_cts_in\" should be connected to '0'.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_flow_ctrl](index.html) module"]
pub struct UART_FLOW_CTRL_SPEC;
impl crate::RegisterSpec for UART_FLOW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_flow_ctrl::R](R) reader structure"]
impl crate::Readable for UART_FLOW_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_flow_ctrl::W](W) writer structure"]
impl crate::Writable for UART_FLOW_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_FLOW_CTRL to value 0"]
impl crate::Resettable for UART_FLOW_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
