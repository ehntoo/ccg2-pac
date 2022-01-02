#[doc = "Register `INTR_S_MASKED` reader"]
pub struct R(crate::R<INTR_S_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_S_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_S_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_S_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `I2C_NACK` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `I2C_ACK` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `I2C_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub struct I2C_WRITE_STOP_R(crate::FieldReader<bool, bool>);
impl I2C_WRITE_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_WRITE_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_WRITE_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_STOP` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `I2C_START` reader - Logical and of corresponding request and mask bits."]
pub struct I2C_START_R(crate::FieldReader<bool, bool>);
impl I2C_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ADDR_MATCH` reader - Logical and of corresponding request and mask bits."]
pub struct I2C_ADDR_MATCH_R(crate::FieldReader<bool, bool>);
impl I2C_ADDR_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ADDR_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ADDR_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_GENERAL` reader - Logical and of corresponding request and mask bits."]
pub struct I2C_GENERAL_R(crate::FieldReader<bool, bool>);
impl I2C_GENERAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_GENERAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_GENERAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
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
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub struct SPI_EZ_WRITE_STOP_R(crate::FieldReader<bool, bool>);
impl SPI_EZ_WRITE_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_EZ_WRITE_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_EZ_WRITE_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_EZ_STOP` reader - Logical and of corresponding request and mask bits."]
pub struct SPI_EZ_STOP_R(crate::FieldReader<bool, bool>);
impl SPI_EZ_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_EZ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_EZ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub struct SPI_BUS_ERROR_R(crate::FieldReader<bool, bool>);
impl SPI_BUS_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_BUS_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_BUS_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SPI_EZ_WRITE_STOP_R {
        SPI_EZ_WRITE_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SPI_EZ_STOP_R {
        SPI_EZ_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SPI_BUS_ERROR_R {
        SPI_BUS_ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Slave interrupt masked request register When read, this register reflects a bitwise and between the interrupt request and mask registers. This register allows SW to read the status of all mask enabled interrupt causes with a single load operation, rather than two load operations: one for the interrupt causes and one for the masks. This simplifies Firmware development. The associated interrupt is active ('1'), when INTR_S_MASKED != 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_s_masked](index.html) module"]
pub struct INTR_S_MASKED_SPEC;
impl crate::RegisterSpec for INTR_S_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_s_masked::R](R) reader structure"]
impl crate::Readable for INTR_S_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_S_MASKED to value 0"]
impl crate::Resettable for INTR_S_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
