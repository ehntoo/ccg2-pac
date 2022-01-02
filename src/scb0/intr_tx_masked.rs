#[doc = "Register `INTR_TX_MASKED` reader"]
pub struct R(crate::R<INTR_TX_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_TX_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_TX_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_TX_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRIGGER` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `NOT_FULL` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `EMPTY` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `OVERFLOW` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `BLOCKED` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `UART_NACK` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `UART_DONE` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `UART_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
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
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn not_full(&self) -> NOT_FULL_R {
        NOT_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UART_NACK_R {
        UART_NACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_done(&self) -> UART_DONE_R {
        UART_DONE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UART_ARB_LOST_R {
        UART_ARB_LOST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Transmitter interrupt masked request register When read, this register reflects a bitwise and between the interrupt request and mask registers. This register allows SW to read the status of all mask enabled interrupt causes with a single load operation, rather than two load operations: one for the interrupt causes and one for the masks. This simplifies Firmware development. The associated interrupt is active ('1'), when INTR_TX_MASKED != 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_tx_masked](index.html) module"]
pub struct INTR_TX_MASKED_SPEC;
impl crate::RegisterSpec for INTR_TX_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_tx_masked::R](R) reader structure"]
impl crate::Readable for INTR_TX_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_TX_MASKED to value 0"]
impl crate::Resettable for INTR_TX_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
