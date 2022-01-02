#[doc = "Register `RX_DEFAULT_SOP_GOODCRC_CTRL` reader"]
pub struct R(crate::R<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_DEFAULT_SOP_GOODCRC_CTRL` writer"]
pub struct W(crate::W<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>;
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
impl From<crate::W<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC>) -> Self {
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
#[doc = "RX Default SOP GoodCRC Control There are two purposes for this register: 1. Transmit Path: When hardware is done transmitting a packet, it will start the RX_CRC_TIMER. The CRC timer should stop on following conditions: 1. Good CRC Received: With Matching Message ID, Matching Header Sop type. 2. Hard Reset on Receive side 3. Soft Reset on Receive Side: 4. Any other message legal in the current firmware state: Condition 1, condition 2: These two conditions are automatically taken care by hardware and CRC timer is stopped. Condition 3, condition 4: Firmware needs to take care of. Firmware can program what legal messages it is expecting in a particular state and hardware will stop its counter. E.G: If firmware wants the transmit logic to stop its CRC counter and not retry the packet on reception of these following messages: � Soft Reset Control Message: Message Type 1101 � Get Source Cap Control Message: Message Type 0111 � Vendor Defined Data Message: Message Type 1111 Then in that case firmware will program RX_DEFAULT_SOP_GOODCRC_CTRL to: 8000_2080 (Bit 7th, bit 13th and bit 31st) Hardware will stop the timers on reception of these packets and will also automatically send GoodCRC message to these messages if the proper auto bit is set in RX_SOP_GOOD_CRC_EN_CTRL. Other messages received will be logged in RX_Memory but Good CRC will not be returned or timer will not be stopped. 2. Receive Path: Based on one hot encoding of RX_DEFAULT_SOP_GOODCRC_CTRL mapped to message type field in the incoming header, Good CRC will be returned automatically if the correct RX_SOP_GOOD_CRC_EN_CTRL bit is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_default_sop_goodcrc_ctrl](index.html) module"]
pub struct RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC;
impl crate::RegisterSpec for RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_default_sop_goodcrc_ctrl::R](R) reader structure"]
impl crate::Readable for RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_default_sop_goodcrc_ctrl::W](W) writer structure"]
impl crate::Writable for RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_DEFAULT_SOP_GOODCRC_CTRL to value 0"]
impl crate::Resettable for RX_DEFAULT_SOP_GOODCRC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
