#[doc = "Register `INTR_RX_MASKED` reader"]
pub struct R(crate::R<INTR_RX_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RX_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RX_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RX_MASKED_SPEC>) -> Self {
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
#[doc = "Field `NOT_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub struct NOT_EMPTY_R(crate::FieldReader<bool, bool>);
impl NOT_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOT_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL` reader - Logical and of corresponding request and mask bits."]
pub struct FULL_R(crate::FieldReader<bool, bool>);
impl FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_R {
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
#[doc = "Field `FRAME_ERROR` reader - Logical and of corresponding request and mask bits."]
pub struct FRAME_ERROR_R(crate::FieldReader<bool, bool>);
impl FRAME_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ERROR` reader - Logical and of corresponding request and mask bits."]
pub struct PARITY_ERROR_R(crate::FieldReader<bool, bool>);
impl PARITY_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD_DETECT` reader - Logical and of corresponding request and mask bits."]
pub struct BAUD_DETECT_R(crate::FieldReader<bool, bool>);
impl BAUD_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BAUD_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK_DETECT` reader - Logical and of corresponding request and mask bits."]
pub struct BREAK_DETECT_R(crate::FieldReader<bool, bool>);
impl BREAK_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BREAK_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAK_DETECT_R {
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
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 0x01) != 0)
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
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Receiver interrupt masked request register When read, this register reflects a bitwise and between the interrupt request and mask registers. This register allows SW to read the status of all mask enabled interrupt causes with a single load operation, rather than two load operations: one for the interrupt causes and one for the masks. This simplifies Firmware development. The associated interrupt is active ('1'), when INTR_RX_MASKED != 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_rx_masked](index.html) module"]
pub struct INTR_RX_MASKED_SPEC;
impl crate::RegisterSpec for INTR_RX_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_rx_masked::R](R) reader structure"]
impl crate::Readable for INTR_RX_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_RX_MASKED to value 0"]
impl crate::Resettable for INTR_RX_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
