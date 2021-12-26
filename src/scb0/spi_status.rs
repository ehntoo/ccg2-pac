#[doc = "Register `SPI_STATUS` reader"]
pub struct R(crate::R<SPI_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUS_BUSY` reader - "]
pub struct BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl BUS_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_EC_BUSY` reader - "]
pub struct SPI_EC_BUSY_R(crate::FieldReader<bool, bool>);
impl SPI_EC_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_EC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_EC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURR_EZ_ADDR` reader - "]
pub struct CURR_EZ_ADDR_R(crate::FieldReader<u8, u8>);
impl CURR_EZ_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CURR_EZ_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURR_EZ_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASE_EZ_ADDR` reader - "]
pub struct BASE_EZ_ADDR_R(crate::FieldReader<u8, u8>);
impl BASE_EZ_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BASE_EZ_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE_EZ_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_ec_busy(&self) -> SPI_EC_BUSY_R {
        SPI_EC_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CURR_EZ_ADDR_R {
        CURR_EZ_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BASE_EZ_ADDR_R {
        BASE_EZ_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_status](index.html) module"]
pub struct SPI_STATUS_SPEC;
impl crate::RegisterSpec for SPI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_status::R](R) reader structure"]
impl crate::Readable for SPI_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_STATUS to value 0"]
impl crate::Resettable for SPI_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
