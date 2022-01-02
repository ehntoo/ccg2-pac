#[doc = "Register `INTR_TX` reader"]
pub struct R(crate::R<INTR_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_TX` writer"]
pub struct W(crate::W<INTR_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_TX_SPEC>;
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
impl From<crate::W<INTR_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` reader - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL. Only used in FIFO mode."]
pub struct TRIGGER_R(crate::FieldReader<bool, bool>);
impl TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER` writer - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL. Only used in FIFO mode."]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
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
#[doc = "Field `NOT_FULL` reader - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
pub struct NOT_FULL_R(crate::FieldReader<bool, bool>);
impl NOT_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOT_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOT_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOT_FULL` writer - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
pub struct NOT_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOT_FULL_W<'a> {
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
#[doc = "Field `EMPTY` reader - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
pub struct EMPTY_R(crate::FieldReader<bool, bool>);
impl EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTY` writer - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
pub struct EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `OVERFLOW` reader - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
pub struct OVERFLOW_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW` writer - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UNDERFLOW` reader - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub struct UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl UNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERFLOW` writer - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub struct UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_W<'a> {
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
#[doc = "Field `BLOCKED` reader - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub struct BLOCKED_R(crate::FieldReader<bool, bool>);
impl BLOCKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCKED` writer - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub struct BLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `UART_NACK` reader - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
pub struct UART_NACK_R(crate::FieldReader<bool, bool>);
impl UART_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_NACK` writer - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
pub struct UART_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_NACK_W<'a> {
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
#[doc = "Field `UART_DONE` reader - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO; i.e. EMPTY is '1'. Set to '1', when event is detected. Write with '1' to clear bit."]
pub struct UART_DONE_R(crate::FieldReader<bool, bool>);
impl UART_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_DONE` writer - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO; i.e. EMPTY is '1'. Set to '1', when event is detected. Write with '1' to clear bit."]
pub struct UART_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DONE_W<'a> {
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
#[doc = "Field `UART_ARB_LOST` reader - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
pub struct UART_ARB_LOST_R(crate::FieldReader<bool, bool>);
impl UART_ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_ARB_LOST` writer - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
pub struct UART_ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ARB_LOST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_full(&self) -> NOT_FULL_R {
        NOT_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UART_NACK_R {
        UART_NACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO; i.e. EMPTY is '1'. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_done(&self) -> UART_DONE_R {
        UART_DONE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UART_ARB_LOST_R {
        UART_ARB_LOST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 1 - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_full(&mut self) -> NOT_FULL_W {
        NOT_FULL_W { w: self }
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W {
        EMPTY_W { w: self }
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 7 - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&mut self) -> BLOCKED_W {
        BLOCKED_W { w: self }
    }
    #[doc = "Bit 8 - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_nack(&mut self) -> UART_NACK_W {
        UART_NACK_W { w: self }
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO; i.e. EMPTY is '1'. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_done(&mut self) -> UART_DONE_W {
        UART_DONE_W { w: self }
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is usefull when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_arb_lost(&mut self) -> UART_ARB_LOST_W {
        UART_ARB_LOST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter interrupt request register. The register fields are not retained In DeepSleep power mode: HW clears the interrupt causes to '0', when coming out of DeepSleep power mode. In addition, HW clears the interrupt causes to '0', when the IP is disabled. As a result, the interrupt causes are only available in Active/Sleep power modes; they are generated by internally clocked logic (this logic operates on a clock that is only available in Active/Sleep power modes).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_tx](index.html) module"]
pub struct INTR_TX_SPEC;
impl crate::RegisterSpec for INTR_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_tx::R](R) reader structure"]
impl crate::Readable for INTR_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_tx::W](W) writer structure"]
impl crate::Writable for INTR_TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_TX to value 0"]
impl crate::Resettable for INTR_TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
