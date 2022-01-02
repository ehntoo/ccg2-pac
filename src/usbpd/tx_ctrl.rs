#[doc = "Register `TX_CTRL` reader"]
pub struct R(crate::R<TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTRL` writer"]
pub struct W(crate::W<TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTRL_SPEC>;
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
impl From<crate::W<TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GOODCRC_MSG_BITS` reader - For SOP Only. This register constains the Transmit GoodCRC Message Header except the MessageID which Is handled by Hardware. \\[11:9\\]
Message ID (This is handled by HardWare)"]
pub struct GOODCRC_MSG_BITS_R(crate::FieldReader<u16, u16>);
impl GOODCRC_MSG_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GOODCRC_MSG_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOODCRC_MSG_BITS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOODCRC_MSG_BITS` writer - For SOP Only. This register constains the Transmit GoodCRC Message Header except the MessageID which Is handled by Hardware. \\[11:9\\]
Message ID (This is handled by HardWare)"]
pub struct GOODCRC_MSG_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> GOODCRC_MSG_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `EN_TX_BIST_CM2` reader - Setting the EN_TX_BIST_CM2 to \"1\" will start the transmision of Bist Carrier Mode 2 pattern. FW must manually set TX_CTRL.TX_REG_EN to \"1\" before setting this register(EN_TX_BIST_CM2) The TX_GO register is not required to be set for this mode."]
pub struct EN_TX_BIST_CM2_R(crate::FieldReader<bool, bool>);
impl EN_TX_BIST_CM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_TX_BIST_CM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_TX_BIST_CM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_TX_BIST_CM2` writer - Setting the EN_TX_BIST_CM2 to \"1\" will start the transmision of Bist Carrier Mode 2 pattern. FW must manually set TX_CTRL.TX_REG_EN to \"1\" before setting this register(EN_TX_BIST_CM2) The TX_GO register is not required to be set for this mode."]
pub struct EN_TX_BIST_CM2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_TX_BIST_CM2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TX_GO` reader - TX_GO causes a packet to be sent. FW can send GoodCrcMsg by storing it in the TX SRAM and use TX_GO to send it. Writing a 1 to this bit to cause the message stored in the SRAM Memory to be sent. Hardware clears this bit once the command is accepted and processing has begun. If TX_GO is set and there is a ongoing receive packet, the TX packet wont be sent and the COLLISION_TYPE1 interrupt will be set. In this case, hardware clears the TX_GO. Before setting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
pub struct TX_GO_R(crate::FieldReader<bool, bool>);
impl TX_GO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_GO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_GO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_GO` writer - TX_GO causes a packet to be sent. FW can send GoodCrcMsg by storing it in the TX SRAM and use TX_GO to send it. Writing a 1 to this bit to cause the message stored in the SRAM Memory to be sent. Hardware clears this bit once the command is accepted and processing has begun. If TX_GO is set and there is a ongoing receive packet, the TX packet wont be sent and the COLLISION_TYPE1 interrupt will be set. In this case, hardware clears the TX_GO. Before setting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
pub struct TX_GO_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_GO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `TX_SEND_RST` reader - Send a Reset over the link. Write a 1 to this bit to cause the transmitter to send a Hard Reset or Cable Reset(TX_HARD_CABLE_ORDER_SET register) over the link. Hardware clears this bit once the command is accepted and processing has begun. If TX_SEND_RST is set and there is a ongoing receive packet, the Reset Sequqnce wont be sent and the COLLISION_TYPE4 interrupt will be set. In this case, hardware clears the TX_SEND_RST. Before settting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
pub struct TX_SEND_RST_R(crate::FieldReader<bool, bool>);
impl TX_SEND_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SEND_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SEND_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SEND_RST` writer - Send a Reset over the link. Write a 1 to this bit to cause the transmitter to send a Hard Reset or Cable Reset(TX_HARD_CABLE_ORDER_SET register) over the link. Hardware clears this bit once the command is accepted and processing has begun. If TX_SEND_RST is set and there is a ongoing receive packet, the Reset Sequqnce wont be sent and the COLLISION_TYPE4 interrupt will be set. In this case, hardware clears the TX_SEND_RST. Before settting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
pub struct TX_SEND_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SEND_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TX_RETRY_ENABLE` reader - Enable transmit retry. Hardware clears this bit once the command is accepted and processing has begun. CPU should increment the retry counter in firmware once TX_PACKET_DONE interrupt is detected by CPU. CPU should set the Retry_Enable bit again if another retry is expected. (CPU should have approximately 750 usec to set this bit). The following operation is recommneded to FW: � FW maintains the retry counter � FW writes a packet in TX_Memory. � FW checks its retry counter and if its >0 , sets the retry enable bit. � FW sets the TX_GO. � HW sends the packet and starts CRC_Timer if enabled. � On Expiry of CRC_timer, HW retries the packet if retry enable bit was set and then HW auto clears that bit. � HW will start the CRC_timer again. � FW in parallel would have received the CRC_TIMER expiry interrupt� FW will decrement its retry counter and if retry counter is still >0, then it will set the retry enable again ( F/W will have approx 1 msec to do this after getting previous CRC timer expiry interrupt)"]
pub struct TX_RETRY_ENABLE_R(crate::FieldReader<bool, bool>);
impl TX_RETRY_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_RETRY_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RETRY_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RETRY_ENABLE` writer - Enable transmit retry. Hardware clears this bit once the command is accepted and processing has begun. CPU should increment the retry counter in firmware once TX_PACKET_DONE interrupt is detected by CPU. CPU should set the Retry_Enable bit again if another retry is expected. (CPU should have approximately 750 usec to set this bit). The following operation is recommneded to FW: � FW maintains the retry counter � FW writes a packet in TX_Memory. � FW checks its retry counter and if its >0 , sets the retry enable bit. � FW sets the TX_GO. � HW sends the packet and starts CRC_Timer if enabled. � On Expiry of CRC_timer, HW retries the packet if retry enable bit was set and then HW auto clears that bit. � HW will start the CRC_timer again. � FW in parallel would have received the CRC_TIMER expiry interrupt� FW will decrement its retry counter and if retry counter is still >0, then it will set the retry enable again ( F/W will have approx 1 msec to do this after getting previous CRC timer expiry interrupt)"]
pub struct TX_RETRY_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RETRY_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TX_REG_EN` reader - Enable the transmitter regulator"]
pub struct TX_REG_EN_R(crate::FieldReader<bool, bool>);
impl TX_REG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_REG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_REG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_REG_EN` writer - Enable the transmitter regulator"]
pub struct TX_REG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REG_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TX_REG_CFG` reader - 0: Hardware controlling of TX regulator Enable is disabled. CPU can fully control the TX regulator enable by using TX_REG_EN. 1: Hardware controlling the TX regulator Enable is enabled. In this case, CPU can only set the regulator enable to one by setting TX_REG_EN"]
pub struct TX_REG_CFG_R(crate::FieldReader<bool, bool>);
impl TX_REG_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_REG_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_REG_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_REG_CFG` writer - 0: Hardware controlling of TX regulator Enable is disabled. CPU can fully control the TX regulator enable by using TX_REG_EN. 1: Hardware controlling the TX regulator Enable is enabled. In this case, CPU can only set the regulator enable to one by setting TX_REG_EN"]
pub struct TX_REG_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REG_CFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TX_REG_TIMER` reader - The time needed to enable the TX regulator before transmission. The counter runs on CLK_TX_HALF(600Khz)"]
pub struct TX_REG_TIMER_R(crate::FieldReader<u8, u8>);
impl TX_REG_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_REG_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_REG_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_REG_TIMER` writer - The time needed to enable the TX regulator before transmission. The counter runs on CLK_TX_HALF(600Khz)"]
pub struct TX_REG_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REG_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - For SOP Only. This register constains the Transmit GoodCRC Message Header except the MessageID which Is handled by Hardware. \\[11:9\\]
Message ID (This is handled by HardWare)"]
    #[inline(always)]
    pub fn goodcrc_msg_bits(&self) -> GOODCRC_MSG_BITS_R {
        GOODCRC_MSG_BITS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Setting the EN_TX_BIST_CM2 to \"1\" will start the transmision of Bist Carrier Mode 2 pattern. FW must manually set TX_CTRL.TX_REG_EN to \"1\" before setting this register(EN_TX_BIST_CM2) The TX_GO register is not required to be set for this mode."]
    #[inline(always)]
    pub fn en_tx_bist_cm2(&self) -> EN_TX_BIST_CM2_R {
        EN_TX_BIST_CM2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TX_GO causes a packet to be sent. FW can send GoodCrcMsg by storing it in the TX SRAM and use TX_GO to send it. Writing a 1 to this bit to cause the message stored in the SRAM Memory to be sent. Hardware clears this bit once the command is accepted and processing has begun. If TX_GO is set and there is a ongoing receive packet, the TX packet wont be sent and the COLLISION_TYPE1 interrupt will be set. In this case, hardware clears the TX_GO. Before setting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
    #[inline(always)]
    pub fn tx_go(&self) -> TX_GO_R {
        TX_GO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Send a Reset over the link. Write a 1 to this bit to cause the transmitter to send a Hard Reset or Cable Reset(TX_HARD_CABLE_ORDER_SET register) over the link. Hardware clears this bit once the command is accepted and processing has begun. If TX_SEND_RST is set and there is a ongoing receive packet, the Reset Sequqnce wont be sent and the COLLISION_TYPE4 interrupt will be set. In this case, hardware clears the TX_SEND_RST. Before settting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
    #[inline(always)]
    pub fn tx_send_rst(&self) -> TX_SEND_RST_R {
        TX_SEND_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable transmit retry. Hardware clears this bit once the command is accepted and processing has begun. CPU should increment the retry counter in firmware once TX_PACKET_DONE interrupt is detected by CPU. CPU should set the Retry_Enable bit again if another retry is expected. (CPU should have approximately 750 usec to set this bit). The following operation is recommneded to FW: � FW maintains the retry counter � FW writes a packet in TX_Memory. � FW checks its retry counter and if its >0 , sets the retry enable bit. � FW sets the TX_GO. � HW sends the packet and starts CRC_Timer if enabled. � On Expiry of CRC_timer, HW retries the packet if retry enable bit was set and then HW auto clears that bit. � HW will start the CRC_timer again. � FW in parallel would have received the CRC_TIMER expiry interrupt� FW will decrement its retry counter and if retry counter is still >0, then it will set the retry enable again ( F/W will have approx 1 msec to do this after getting previous CRC timer expiry interrupt)"]
    #[inline(always)]
    pub fn tx_retry_enable(&self) -> TX_RETRY_ENABLE_R {
        TX_RETRY_ENABLE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable the transmitter regulator"]
    #[inline(always)]
    pub fn tx_reg_en(&self) -> TX_REG_EN_R {
        TX_REG_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 0: Hardware controlling of TX regulator Enable is disabled. CPU can fully control the TX regulator enable by using TX_REG_EN. 1: Hardware controlling the TX regulator Enable is enabled. In this case, CPU can only set the regulator enable to one by setting TX_REG_EN"]
    #[inline(always)]
    pub fn tx_reg_cfg(&self) -> TX_REG_CFG_R {
        TX_REG_CFG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - The time needed to enable the TX regulator before transmission. The counter runs on CLK_TX_HALF(600Khz)"]
    #[inline(always)]
    pub fn tx_reg_timer(&self) -> TX_REG_TIMER_R {
        TX_REG_TIMER_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - For SOP Only. This register constains the Transmit GoodCRC Message Header except the MessageID which Is handled by Hardware. \\[11:9\\]
Message ID (This is handled by HardWare)"]
    #[inline(always)]
    pub fn goodcrc_msg_bits(&mut self) -> GOODCRC_MSG_BITS_W {
        GOODCRC_MSG_BITS_W { w: self }
    }
    #[doc = "Bit 16 - Setting the EN_TX_BIST_CM2 to \"1\" will start the transmision of Bist Carrier Mode 2 pattern. FW must manually set TX_CTRL.TX_REG_EN to \"1\" before setting this register(EN_TX_BIST_CM2) The TX_GO register is not required to be set for this mode."]
    #[inline(always)]
    pub fn en_tx_bist_cm2(&mut self) -> EN_TX_BIST_CM2_W {
        EN_TX_BIST_CM2_W { w: self }
    }
    #[doc = "Bit 17 - TX_GO causes a packet to be sent. FW can send GoodCrcMsg by storing it in the TX SRAM and use TX_GO to send it. Writing a 1 to this bit to cause the message stored in the SRAM Memory to be sent. Hardware clears this bit once the command is accepted and processing has begun. If TX_GO is set and there is a ongoing receive packet, the TX packet wont be sent and the COLLISION_TYPE1 interrupt will be set. In this case, hardware clears the TX_GO. Before setting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
    #[inline(always)]
    pub fn tx_go(&mut self) -> TX_GO_W {
        TX_GO_W { w: self }
    }
    #[doc = "Bit 18 - Send a Reset over the link. Write a 1 to this bit to cause the transmitter to send a Hard Reset or Cable Reset(TX_HARD_CABLE_ORDER_SET register) over the link. Hardware clears this bit once the command is accepted and processing has begun. If TX_SEND_RST is set and there is a ongoing receive packet, the Reset Sequqnce wont be sent and the COLLISION_TYPE4 interrupt will be set. In this case, hardware clears the TX_SEND_RST. Before settting this FW should check: INTR0->RX_GOOD_PKT && STATUS->DATA_VALID == 0"]
    #[inline(always)]
    pub fn tx_send_rst(&mut self) -> TX_SEND_RST_W {
        TX_SEND_RST_W { w: self }
    }
    #[doc = "Bit 19 - Enable transmit retry. Hardware clears this bit once the command is accepted and processing has begun. CPU should increment the retry counter in firmware once TX_PACKET_DONE interrupt is detected by CPU. CPU should set the Retry_Enable bit again if another retry is expected. (CPU should have approximately 750 usec to set this bit). The following operation is recommneded to FW: � FW maintains the retry counter � FW writes a packet in TX_Memory. � FW checks its retry counter and if its >0 , sets the retry enable bit. � FW sets the TX_GO. � HW sends the packet and starts CRC_Timer if enabled. � On Expiry of CRC_timer, HW retries the packet if retry enable bit was set and then HW auto clears that bit. � HW will start the CRC_timer again. � FW in parallel would have received the CRC_TIMER expiry interrupt� FW will decrement its retry counter and if retry counter is still >0, then it will set the retry enable again ( F/W will have approx 1 msec to do this after getting previous CRC timer expiry interrupt)"]
    #[inline(always)]
    pub fn tx_retry_enable(&mut self) -> TX_RETRY_ENABLE_W {
        TX_RETRY_ENABLE_W { w: self }
    }
    #[doc = "Bit 20 - Enable the transmitter regulator"]
    #[inline(always)]
    pub fn tx_reg_en(&mut self) -> TX_REG_EN_W {
        TX_REG_EN_W { w: self }
    }
    #[doc = "Bit 21 - 0: Hardware controlling of TX regulator Enable is disabled. CPU can fully control the TX regulator enable by using TX_REG_EN. 1: Hardware controlling the TX regulator Enable is enabled. In this case, CPU can only set the regulator enable to one by setting TX_REG_EN"]
    #[inline(always)]
    pub fn tx_reg_cfg(&mut self) -> TX_REG_CFG_W {
        TX_REG_CFG_W { w: self }
    }
    #[doc = "Bits 24:29 - The time needed to enable the TX regulator before transmission. The counter runs on CLK_TX_HALF(600Khz)"]
    #[inline(always)]
    pub fn tx_reg_timer(&mut self) -> TX_REG_TIMER_W {
        TX_REG_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl](index.html) module"]
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctrl::R](R) reader structure"]
impl crate::Readable for TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctrl::W](W) writer structure"]
impl crate::Writable for TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x0f20_0041"]
impl crate::Resettable for TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f20_0041
    }
}
