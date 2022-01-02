#[doc = "Register `INTER_PACKET_COUNTER` reader"]
pub struct R(crate::R<INTER_PACKET_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTER_PACKET_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTER_PACKET_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTER_PACKET_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTER_PACKET_COUNTER` writer"]
pub struct W(crate::W<INTER_PACKET_COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTER_PACKET_COUNTER_SPEC>;
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
impl From<crate::W<INTER_PACKET_COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTER_PACKET_COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_IDLE_CNT` reader - USED FOR TX->TX This register is used by DUT to create gap between two back to back transmit packets. For example: For a DFP application if DFP wants to send Hard Reset after a valid packet, then this register could be used for complying with Inter-Packet Gap of 25 usec specified by spec. In cable application, after sending Good CRC handshake for request from DUT, this register could be used to comply with 750usec requirement of cable response after sending Good CRC pkt."]
pub struct BUS_IDLE_CNT_R(crate::FieldReader<u16, u16>);
impl BUS_IDLE_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUS_IDLE_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_IDLE_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_IDLE_CNT` writer - USED FOR TX->TX This register is used by DUT to create gap between two back to back transmit packets. For example: For a DFP application if DFP wants to send Hard Reset after a valid packet, then this register could be used for complying with Inter-Packet Gap of 25 usec specified by spec. In cable application, after sending Good CRC handshake for request from DUT, this register could be used to comply with 750usec requirement of cable response after sending Good CRC pkt."]
pub struct BUS_IDLE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_IDLE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `IDLE_COUNTER` reader - RX -> AUTO_GOODCRC_RESPONSE This counter specifies how long the HW should wait after the end of RX packet to send GoodCRC message. This can be used to comply with interpacket gap of 25usec. 0: Counter is disabled. Hardware will issue goodcrc message if needed after end of the RX Packet"]
pub struct IDLE_COUNTER_R(crate::FieldReader<u16, u16>);
impl IDLE_COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDLE_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_COUNTER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_COUNTER` writer - RX -> AUTO_GOODCRC_RESPONSE This counter specifies how long the HW should wait after the end of RX packet to send GoodCRC message. This can be used to comply with interpacket gap of 25usec. 0: Counter is disabled. Hardware will issue goodcrc message if needed after end of the RX Packet"]
pub struct IDLE_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 11)) | ((value as u32 & 0x03ff) << 11);
        self.w
    }
}
#[doc = "Field `IFG_COUNTER` reader - END OF ANY RX ON CC-LINE On any RX Packet, this counter will start at end of RX Packet and will reset on Start of RX Packet. This register can be again used to comply with Interpacket Gap (25 usec + end of IDLE detection(12 usec)). CPU after seeing no activity on the bus can immedeately set the TX_GO/TX_SEND_RST bit and this register will make sure that we don't violate any interpacket gap requirement. 0: Counter is disabled."]
pub struct IFG_COUNTER_R(crate::FieldReader<u16, u16>);
impl IFG_COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IFG_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFG_COUNTER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFG_COUNTER` writer - END OF ANY RX ON CC-LINE On any RX Packet, this counter will start at end of RX Packet and will reset on Start of RX Packet. This register can be again used to comply with Interpacket Gap (25 usec + end of IDLE detection(12 usec)). CPU after seeing no activity on the bus can immedeately set the TX_GO/TX_SEND_RST bit and this register will make sure that we don't violate any interpacket gap requirement. 0: Counter is disabled."]
pub struct IFG_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IFG_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | ((value as u32 & 0x07ff) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - USED FOR TX->TX This register is used by DUT to create gap between two back to back transmit packets. For example: For a DFP application if DFP wants to send Hard Reset after a valid packet, then this register could be used for complying with Inter-Packet Gap of 25 usec specified by spec. In cable application, after sending Good CRC handshake for request from DUT, this register could be used to comply with 750usec requirement of cable response after sending Good CRC pkt."]
    #[inline(always)]
    pub fn bus_idle_cnt(&self) -> BUS_IDLE_CNT_R {
        BUS_IDLE_CNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:20 - RX -> AUTO_GOODCRC_RESPONSE This counter specifies how long the HW should wait after the end of RX packet to send GoodCRC message. This can be used to comply with interpacket gap of 25usec. 0: Counter is disabled. Hardware will issue goodcrc message if needed after end of the RX Packet"]
    #[inline(always)]
    pub fn idle_counter(&self) -> IDLE_COUNTER_R {
        IDLE_COUNTER_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:31 - END OF ANY RX ON CC-LINE On any RX Packet, this counter will start at end of RX Packet and will reset on Start of RX Packet. This register can be again used to comply with Interpacket Gap (25 usec + end of IDLE detection(12 usec)). CPU after seeing no activity on the bus can immedeately set the TX_GO/TX_SEND_RST bit and this register will make sure that we don't violate any interpacket gap requirement. 0: Counter is disabled."]
    #[inline(always)]
    pub fn ifg_counter(&self) -> IFG_COUNTER_R {
        IFG_COUNTER_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - USED FOR TX->TX This register is used by DUT to create gap between two back to back transmit packets. For example: For a DFP application if DFP wants to send Hard Reset after a valid packet, then this register could be used for complying with Inter-Packet Gap of 25 usec specified by spec. In cable application, after sending Good CRC handshake for request from DUT, this register could be used to comply with 750usec requirement of cable response after sending Good CRC pkt."]
    #[inline(always)]
    pub fn bus_idle_cnt(&mut self) -> BUS_IDLE_CNT_W {
        BUS_IDLE_CNT_W { w: self }
    }
    #[doc = "Bits 11:20 - RX -> AUTO_GOODCRC_RESPONSE This counter specifies how long the HW should wait after the end of RX packet to send GoodCRC message. This can be used to comply with interpacket gap of 25usec. 0: Counter is disabled. Hardware will issue goodcrc message if needed after end of the RX Packet"]
    #[inline(always)]
    pub fn idle_counter(&mut self) -> IDLE_COUNTER_W {
        IDLE_COUNTER_W { w: self }
    }
    #[doc = "Bits 21:31 - END OF ANY RX ON CC-LINE On any RX Packet, this counter will start at end of RX Packet and will reset on Start of RX Packet. This register can be again used to comply with Interpacket Gap (25 usec + end of IDLE detection(12 usec)). CPU after seeing no activity on the bus can immedeately set the TX_GO/TX_SEND_RST bit and this register will make sure that we don't violate any interpacket gap requirement. 0: Counter is disabled."]
    #[inline(always)]
    pub fn ifg_counter(&mut self) -> IFG_COUNTER_W {
        IFG_COUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Inter Packet counters Counters used for IDLE/IFG and by this IP All the timers/counters have a resolution of 1 UI (Unit Interval) of a Bit. If transmit rate is 300Khz, then each count will tick for 3.33 usec.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inter_packet_counter](index.html) module"]
pub struct INTER_PACKET_COUNTER_SPEC;
impl crate::RegisterSpec for INTER_PACKET_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inter_packet_counter::R](R) reader structure"]
impl crate::Readable for INTER_PACKET_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inter_packet_counter::W](W) writer structure"]
impl crate::Writable for INTER_PACKET_COUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTER_PACKET_COUNTER to value 0x0100_4008"]
impl crate::Resettable for INTER_PACKET_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_4008
    }
}
