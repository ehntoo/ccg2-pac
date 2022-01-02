#[doc = "Register `I2C_STATUS` reader"]
pub struct R(crate::R<I2C_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUS_BUSY` reader - I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the IP is disabled, BUS_BUSY is '0'. After enabling the IP, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
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
#[doc = "Field `I2C_EC_BUSY` reader - Inidicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
pub struct I2C_EC_BUSY_R(crate::FieldReader<bool, bool>);
impl I2C_EC_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_EC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_EC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_READ` reader - I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0''."]
pub struct S_READ_R(crate::FieldReader<bool, bool>);
impl S_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_READ` reader - I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
pub struct M_READ_R(crate::FieldReader<bool, bool>);
impl M_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURR_EZ_ADDR` reader - I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
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
#[doc = "Field `BASE_EZ_ADDR` reader - I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
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
    #[doc = "Bit 0 - I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the IP is disabled, BUS_BUSY is '0'. After enabling the IP, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Inidicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
    #[inline(always)]
    pub fn i2c_ec_busy(&self) -> I2C_EC_BUSY_R {
        I2C_EC_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn s_read(&self) -> S_READ_R {
        S_READ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn m_read(&self) -> M_READ_R {
        M_READ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CURR_EZ_ADDR_R {
        CURR_EZ_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BASE_EZ_ADDR_R {
        BASE_EZ_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "I2C status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_status](index.html) module"]
pub struct I2C_STATUS_SPEC;
impl crate::RegisterSpec for I2C_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_status::R](R) reader structure"]
impl crate::Readable for I2C_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_STATUS to value 0"]
impl crate::Resettable for I2C_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
