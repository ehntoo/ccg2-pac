#[doc = "Register `INTR_M_MASKED` reader"]
pub struct R(crate::R<INTR_M_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_M_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_M_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_M_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - "]
pub struct I2C_ARB_LOST_R(crate::FieldReader<bool, bool>);
impl I2C_ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_NACK` reader - "]
pub struct I2C_NACK_R(crate::FieldReader<bool, bool>);
impl I2C_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ACK` reader - "]
pub struct I2C_ACK_R(crate::FieldReader<bool, bool>);
impl I2C_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_STOP` reader - "]
pub struct I2C_STOP_R(crate::FieldReader<bool, bool>);
impl I2C_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_BUS_ERROR` reader - "]
pub struct I2C_BUS_ERROR_R(crate::FieldReader<bool, bool>);
impl I2C_BUS_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_BUS_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_BUS_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_DONE` reader - "]
pub struct SPI_DONE_R(crate::FieldReader<bool, bool>);
impl SPI_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_done(&self) -> SPI_DONE_R {
        SPI_DONE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_m_masked](index.html) module"]
pub struct INTR_M_MASKED_SPEC;
impl crate::RegisterSpec for INTR_M_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_m_masked::R](R) reader structure"]
impl crate::Readable for INTR_M_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_M_MASKED to value 0"]
impl crate::Resettable for INTR_M_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
