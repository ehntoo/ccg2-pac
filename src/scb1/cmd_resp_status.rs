#[doc = "Register `CMD_RESP_STATUS` reader"]
pub struct R(crate::R<CMD_RESP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RESP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RESP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RESP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURR_RD_ADDR` reader - I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
pub struct CURR_RD_ADDR_R(crate::FieldReader<u8, u8>);
impl CURR_RD_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CURR_RD_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURR_RD_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURR_WR_ADDR` reader - I2C/SPI write current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
pub struct CURR_WR_ADDR_R(crate::FieldReader<u8, u8>);
impl CURR_WR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CURR_WR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURR_WR_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_RESP_EC_BUS_BUSY` reader - Indicates whether there is an ongoing bus transfer to the IP. '0': no ongoing bus transfer. '1': ongoing bus transferr. For SPI, the field is '1' when the slave is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
pub struct CMD_RESP_EC_BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl CMD_RESP_EC_BUS_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_RESP_EC_BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_RESP_EC_BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_RESP_EC_BUSY` reader - Indicates whether the CURR_RD_ADDR and CURR_WR_ADDR fields in this register are reliable (CMD_RESP_EC_BUSY is '0') or not reliable (CMD_RESP_EC_BUSY is '1'). Note: - When there is no bus transfer, CMD_RESP_EC_BUSY is '0'. - When there is a bus transfer, CMD_RESP_EC_BUSY is '0', when the CURR_RD_ADDR and CURR_WR_ADDR are not updated by the HW. - When there is a bus transfer, CMD_RESP_EC_BUSY is '1', when the CURR_RD_ADDR or CURR_WR_ADDR are updated by the HW. Note that this update lasts a single serial interface clock cycle."]
pub struct CMD_RESP_EC_BUSY_R(crate::FieldReader<bool, bool>);
impl CMD_RESP_EC_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_RESP_EC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_RESP_EC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn curr_rd_addr(&self) -> CURR_RD_ADDR_R {
        CURR_RD_ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - I2C/SPI write current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn curr_wr_addr(&self) -> CURR_WR_ADDR_R {
        CURR_WR_ADDR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Indicates whether there is an ongoing bus transfer to the IP. '0': no ongoing bus transfer. '1': ongoing bus transferr. For SPI, the field is '1' when the slave is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
    #[inline(always)]
    pub fn cmd_resp_ec_bus_busy(&self) -> CMD_RESP_EC_BUS_BUSY_R {
        CMD_RESP_EC_BUS_BUSY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Indicates whether the CURR_RD_ADDR and CURR_WR_ADDR fields in this register are reliable (CMD_RESP_EC_BUSY is '0') or not reliable (CMD_RESP_EC_BUSY is '1'). Note: - When there is no bus transfer, CMD_RESP_EC_BUSY is '0'. - When there is a bus transfer, CMD_RESP_EC_BUSY is '0', when the CURR_RD_ADDR and CURR_WR_ADDR are not updated by the HW. - When there is a bus transfer, CMD_RESP_EC_BUSY is '1', when the CURR_RD_ADDR or CURR_WR_ADDR are updated by the HW. Note that this update lasts a single serial interface clock cycle."]
    #[inline(always)]
    pub fn cmd_resp_ec_busy(&self) -> CMD_RESP_EC_BUSY_R {
        CMD_RESP_EC_BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Command/response status register. The register fields reflect register states without a default/reset value (CURR_RD_ADDR and CURR_WR_ADDR) or reflect an external bus state. Therefore, these registers are undefined after the IP is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_resp_status](index.html) module"]
pub struct CMD_RESP_STATUS_SPEC;
impl crate::RegisterSpec for CMD_RESP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_resp_status::R](R) reader structure"]
impl crate::Readable for CMD_RESP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMD_RESP_STATUS to value 0"]
impl crate::Resettable for CMD_RESP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
