#[doc = "Register `CMD_RESP_CTRL` reader"]
pub struct R(crate::R<CMD_RESP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RESP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RESP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RESP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_RESP_CTRL` writer"]
pub struct W(crate::W<CMD_RESP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_RESP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CMD_RESP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_RESP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE_RD_ADDR` reader - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub struct BASE_RD_ADDR_R(crate::FieldReader<u8, u8>);
impl BASE_RD_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BASE_RD_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE_RD_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASE_RD_ADDR` writer - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub struct BASE_RD_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_RD_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `BASE_WR_ADDR` reader - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub struct BASE_WR_ADDR_R(crate::FieldReader<u8, u8>);
impl BASE_WR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BASE_WR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE_WR_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASE_WR_ADDR` writer - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub struct BASE_WR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_WR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&self) -> BASE_RD_ADDR_R {
        BASE_RD_ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&self) -> BASE_WR_ADDR_R {
        BASE_WR_ADDR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&mut self) -> BASE_RD_ADDR_W {
        BASE_RD_ADDR_W { w: self }
    }
    #[doc = "Bits 16:20 - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&mut self) -> BASE_WR_ADDR_W {
        BASE_WR_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command/response control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_resp_ctrl](index.html) module"]
pub struct CMD_RESP_CTRL_SPEC;
impl crate::RegisterSpec for CMD_RESP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_resp_ctrl::R](R) reader structure"]
impl crate::Readable for CMD_RESP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_resp_ctrl::W](W) writer structure"]
impl crate::Writable for CMD_RESP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD_RESP_CTRL to value 0"]
impl crate::Resettable for CMD_RESP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
