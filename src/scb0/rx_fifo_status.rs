#[doc = "Register `RX_FIFO_STATUS` reader"]
pub struct R(crate::R<RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - "]
pub struct USED_R(crate::FieldReader<u8, u8>);
impl USED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR_VALID` reader - "]
pub struct SR_VALID_R(crate::FieldReader<bool, bool>);
impl SR_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SR_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_PTR` reader - "]
pub struct RD_PTR_R(crate::FieldReader<u8, u8>);
impl RD_PTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PTR` reader - "]
pub struct WR_PTR_R(crate::FieldReader<u8, u8>);
impl WR_PTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sr_valid(&self) -> SR_VALID_R {
        SR_VALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_status](index.html) module"]
pub struct RX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for RX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_status::R](R) reader structure"]
impl crate::Readable for RX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_STATUS to value 0"]
impl crate::Resettable for RX_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
