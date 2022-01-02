#[doc = "Register `RX_GOODCRC_MSG` reader"]
pub struct R(crate::R<RX_GOODCRC_MSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_GOODCRC_MSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_GOODCRC_MSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_GOODCRC_MSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HEADER` reader - The INTR.RCV_GOODCRC_MSG_COMPLETE interrupt indicates the 2-Byte header for GoodCRC message is received and stored in this registers. GOODCRC_MSG_SOP_TYPE_DETECTED contains the SOP type for the GoodCRC MSG. At the start of every packet, INTR.RCV_GOODCRC_MSG_COMPLETE status is evaluated, if its reset, then only a new packet will be written else new packet will be dropped."]
pub struct HEADER_R(crate::FieldReader<u16, u16>);
impl HEADER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HEADER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HEADER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - The INTR.RCV_GOODCRC_MSG_COMPLETE interrupt indicates the 2-Byte header for GoodCRC message is received and stored in this registers. GOODCRC_MSG_SOP_TYPE_DETECTED contains the SOP type for the GoodCRC MSG. At the start of every packet, INTR.RCV_GOODCRC_MSG_COMPLETE status is evaluated, if its reset, then only a new packet will be written else new packet will be dropped."]
    #[inline(always)]
    pub fn header(&self) -> HEADER_R {
        HEADER_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "The 2-Byte Header of the received GoodCRC Message\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_goodcrc_msg](index.html) module"]
pub struct RX_GOODCRC_MSG_SPEC;
impl crate::RegisterSpec for RX_GOODCRC_MSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_goodcrc_msg::R](R) reader structure"]
impl crate::Readable for RX_GOODCRC_MSG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_GOODCRC_MSG to value 0"]
impl crate::Resettable for RX_GOODCRC_MSG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
