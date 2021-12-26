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
#[doc = "Field `CURR_RD_ADDR` reader - "]
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
#[doc = "Field `CURR_WR_ADDR` reader - "]
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
#[doc = "Field `CMD_RESP_EC_BUS_BUSY` reader - "]
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
#[doc = "Field `CMD_RESP_EC_BUSY` reader - "]
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
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn curr_rd_addr(&self) -> CURR_RD_ADDR_R {
        CURR_RD_ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn curr_wr_addr(&self) -> CURR_WR_ADDR_R {
        CURR_WR_ADDR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmd_resp_ec_bus_busy(&self) -> CMD_RESP_EC_BUS_BUSY_R {
        CMD_RESP_EC_BUS_BUSY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmd_resp_ec_busy(&self) -> CMD_RESP_EC_BUSY_R {
        CMD_RESP_EC_BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_resp_status](index.html) module"]
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
