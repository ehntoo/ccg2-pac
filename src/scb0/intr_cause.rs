#[doc = "Register `INTR_CAUSE` reader"]
pub struct R(crate::R<INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M` reader - Master interrupt active (\"interrupt_master\"): INTR_M_MASKED != 0."]
pub struct M_R(crate::FieldReader<bool, bool>);
impl M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S` reader - Slave interrupt active (\"interrupt_slave\"): INTR_S_MASKED != 0."]
pub struct S_R(crate::FieldReader<bool, bool>);
impl S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX` reader - Transmitter interrupt active (\"interrupt_tx\"): INTR_TX_MASKED != 0."]
pub struct TX_R(crate::FieldReader<bool, bool>);
impl TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX` reader - Receiver interrupt active (\"interrupt_rx\"): INTR_RX_MASKED != 0."]
pub struct RX_R(crate::FieldReader<bool, bool>);
impl RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_EC` reader - Externally clock I2C interrupt active (\"interrupt_i2c_ec\"): INTR_I2C_EC_MASKED != 0."]
pub struct I2C_EC_R(crate::FieldReader<bool, bool>);
impl I2C_EC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_EC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_EC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_EC` reader - Externally clocked SPI interrupt active (\"interrupt_spi_ec\"): INTR_SPI_EC_MASKED != 0."]
pub struct SPI_EC_R(crate::FieldReader<bool, bool>);
impl SPI_EC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_EC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_EC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Master interrupt active (\"interrupt_master\"): INTR_M_MASKED != 0."]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave interrupt active (\"interrupt_slave\"): INTR_S_MASKED != 0."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter interrupt active (\"interrupt_tx\"): INTR_TX_MASKED != 0."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver interrupt active (\"interrupt_rx\"): INTR_RX_MASKED != 0."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Externally clock I2C interrupt active (\"interrupt_i2c_ec\"): INTR_I2C_EC_MASKED != 0."]
    #[inline(always)]
    pub fn i2c_ec(&self) -> I2C_EC_R {
        I2C_EC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Externally clocked SPI interrupt active (\"interrupt_spi_ec\"): INTR_SPI_EC_MASKED != 0."]
    #[inline(always)]
    pub fn spi_ec(&self) -> SPI_EC_R {
        SPI_EC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Active clocked interrupt signal register Enables software to determine the source of the combined interrupt output signals \"interrupt_ic\", \"interrupt_ec\" and \"interrupt\".\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
pub struct INTR_CAUSE_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for INTR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
