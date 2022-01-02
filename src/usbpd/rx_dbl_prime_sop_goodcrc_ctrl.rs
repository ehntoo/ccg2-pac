#[doc = "Register `RX_DBL_PRIME_SOP_GOODCRC_CTRL` reader"]
pub struct R(crate::R<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_DBL_PRIME_SOP_GOODCRC_CTRL` writer"]
pub struct W(crate::W<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>;
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
impl From<crate::W<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_GOOD_CRC_PER_CTRL_MSG_TYP` reader - The Message type of a control message is a 4-bit field. Every bit of this register mapps to the control message types. CPU can used this register to select for which one of the the message types of a control message should hardware send a GOODCRC message."]
pub struct SEND_GOOD_CRC_PER_CTRL_MSG_TYP_R(crate::FieldReader<u16, u16>);
impl SEND_GOOD_CRC_PER_CTRL_MSG_TYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SEND_GOOD_CRC_PER_CTRL_MSG_TYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_GOOD_CRC_PER_CTRL_MSG_TYP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_GOOD_CRC_PER_CTRL_MSG_TYP` writer - The Message type of a control message is a 4-bit field. Every bit of this register mapps to the control message types. CPU can used this register to select for which one of the the message types of a control message should hardware send a GOODCRC message."]
pub struct SEND_GOOD_CRC_PER_CTRL_MSG_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_GOOD_CRC_PER_CTRL_MSG_TYP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `SEND_GOOD_CRC_PER_DATA_MSG_TYP` reader - The Message type of a data message is a 4-bit field. Every bit of this register mapps to the data message types. CPU can used this register to select for which one of the the message types of a data message should hardware send a GOODCRC message."]
pub struct SEND_GOOD_CRC_PER_DATA_MSG_TYP_R(crate::FieldReader<u16, u16>);
impl SEND_GOOD_CRC_PER_DATA_MSG_TYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SEND_GOOD_CRC_PER_DATA_MSG_TYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_GOOD_CRC_PER_DATA_MSG_TYP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_GOOD_CRC_PER_DATA_MSG_TYP` writer - The Message type of a data message is a 4-bit field. Every bit of this register mapps to the data message types. CPU can used this register to select for which one of the the message types of a data message should hardware send a GOODCRC message."]
pub struct SEND_GOOD_CRC_PER_DATA_MSG_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_GOOD_CRC_PER_DATA_MSG_TYP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The Message type of a control message is a 4-bit field. Every bit of this register mapps to the control message types. CPU can used this register to select for which one of the the message types of a control message should hardware send a GOODCRC message."]
    #[inline(always)]
    pub fn send_good_crc_per_ctrl_msg_typ(&self) -> SEND_GOOD_CRC_PER_CTRL_MSG_TYP_R {
        SEND_GOOD_CRC_PER_CTRL_MSG_TYP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The Message type of a data message is a 4-bit field. Every bit of this register mapps to the data message types. CPU can used this register to select for which one of the the message types of a data message should hardware send a GOODCRC message."]
    #[inline(always)]
    pub fn send_good_crc_per_data_msg_typ(&self) -> SEND_GOOD_CRC_PER_DATA_MSG_TYP_R {
        SEND_GOOD_CRC_PER_DATA_MSG_TYP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The Message type of a control message is a 4-bit field. Every bit of this register mapps to the control message types. CPU can used this register to select for which one of the the message types of a control message should hardware send a GOODCRC message."]
    #[inline(always)]
    pub fn send_good_crc_per_ctrl_msg_typ(&mut self) -> SEND_GOOD_CRC_PER_CTRL_MSG_TYP_W {
        SEND_GOOD_CRC_PER_CTRL_MSG_TYP_W { w: self }
    }
    #[doc = "Bits 16:31 - The Message type of a data message is a 4-bit field. Every bit of this register mapps to the data message types. CPU can used this register to select for which one of the the message types of a data message should hardware send a GOODCRC message."]
    #[inline(always)]
    pub fn send_good_crc_per_data_msg_typ(&mut self) -> SEND_GOOD_CRC_PER_DATA_MSG_TYP_W {
        SEND_GOOD_CRC_PER_DATA_MSG_TYP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX DBL Prime SOP GoodCRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_dbl_prime_sop_goodcrc_ctrl](index.html) module"]
pub struct RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC;
impl crate::RegisterSpec for RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_dbl_prime_sop_goodcrc_ctrl::R](R) reader structure"]
impl crate::Readable for RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_dbl_prime_sop_goodcrc_ctrl::W](W) writer structure"]
impl crate::Writable for RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_DBL_PRIME_SOP_GOODCRC_CTRL to value 0"]
impl crate::Resettable for RX_DBL_PRIME_SOP_GOODCRC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
