#[doc = "Register `RX_MEM_DATA0` reader"]
pub struct R(crate::R<RX_MEM_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MEM_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MEM_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MEM_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Header/Data information in the receive SRAM. SOP type is stored in STATUS.SOP_TYPE_DETECTED register. The CPU should perform one read to register address 0x0030 to get the Rx Header(16-bit). The upper 16-bit of the address 0x0030 should be ignored. CPU should extract the 4-byte count from the Rx Header and perform the proper number of read transactions. STATUS.SOP_TYPE_DETECTED contains the SOP type for the packet in the RX SRAM. At the start of every packet, INTR.RCV_PACKET_COMPLETE and INTR.RCV_RST status is evaluated, if its reset, then only a new packet will be written else new packet will be dropped."]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Header/Data information in the receive SRAM. SOP type is stored in STATUS.SOP_TYPE_DETECTED register. The CPU should perform one read to register address 0x0030 to get the Rx Header(16-bit). The upper 16-bit of the address 0x0030 should be ignored. CPU should extract the 4-byte count from the Rx Header and perform the proper number of read transactions. STATUS.SOP_TYPE_DETECTED contains the SOP type for the packet in the RX SRAM. At the start of every packet, INTR.RCV_PACKET_COMPLETE and INTR.RCV_RST status is evaluated, if its reset, then only a new packet will be written else new packet will be dropped."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RX SRAM Data The memory for the RX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0050: The Rx Header 0x0054:0x006C: The RX Data Object Any access to address space 0x0050 - 0x006C will map to SRAM address x0-x15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_mem_data0](index.html) module"]
pub struct RX_MEM_DATA0_SPEC;
impl crate::RegisterSpec for RX_MEM_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_mem_data0::R](R) reader structure"]
impl crate::Readable for RX_MEM_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_MEM_DATA0 to value 0"]
impl crate::Resettable for RX_MEM_DATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
