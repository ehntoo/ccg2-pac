#[doc = "Register `UART_RX_STATUS` reader"]
pub struct R(crate::R<UART_RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BR_COUNTER` reader - "]
pub struct BR_COUNTER_R(crate::FieldReader<u16, u16>);
impl BR_COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BR_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BR_COUNTER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn br_counter(&self) -> BR_COUNTER_R {
        BR_COUNTER_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rx_status](index.html) module"]
pub struct UART_RX_STATUS_SPEC;
impl crate::RegisterSpec for UART_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rx_status::R](R) reader structure"]
impl crate::Readable for UART_RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_RX_STATUS to value 0"]
impl crate::Resettable for UART_RX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
