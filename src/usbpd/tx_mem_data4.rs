#[doc = "Register `TX_MEM_DATA4` reader"]
pub struct R(crate::R<TX_MEM_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_MEM_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_MEM_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_MEM_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_MEM_DATA4` writer"]
pub struct W(crate::W<TX_MEM_DATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_MEM_DATA4_SPEC>;
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
impl From<crate::W<TX_MEM_DATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_MEM_DATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Header/Data information in the transmitter SRAM. SOP/CRC/EOP will be appened by HW. The CPU is required to program the TX Header into lower 16-bit of register address 0x0050. The data read(Tx Header, 16-bit) from this address provides the number of 4-bytes required to be transmitted. If the byte count is zero in the Tx Header, then no more read will be issued to SRAM."]
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
#[doc = "Field `DATA` writer - Header/Data information in the transmitter SRAM. SOP/CRC/EOP will be appened by HW. The CPU is required to program the TX Header into lower 16-bit of register address 0x0050. The data read(Tx Header, 16-bit) from this address provides the number of 4-bytes required to be transmitted. If the byte count is zero in the Tx Header, then no more read will be issued to SRAM."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Header/Data information in the transmitter SRAM. SOP/CRC/EOP will be appened by HW. The CPU is required to program the TX Header into lower 16-bit of register address 0x0050. The data read(Tx Header, 16-bit) from this address provides the number of 4-bytes required to be transmitted. If the byte count is zero in the Tx Header, then no more read will be issued to SRAM."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Header/Data information in the transmitter SRAM. SOP/CRC/EOP will be appened by HW. The CPU is required to program the TX Header into lower 16-bit of register address 0x0050. The data read(Tx Header, 16-bit) from this address provides the number of 4-bytes required to be transmitted. If the byte count is zero in the Tx Header, then no more read will be issued to SRAM."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX SRAM Data The memory for the TX USB power controller is a 32 byte SRAM. Address map for this Memory is: 0x0030: The Tx Header 0x0034:0x004C: The TX Data Object Any access to address space 0x0030 - 0x004C will map to SRAM address x0-x15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_mem_data4](index.html) module"]
pub struct TX_MEM_DATA4_SPEC;
impl crate::RegisterSpec for TX_MEM_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_mem_data4::R](R) reader structure"]
impl crate::Readable for TX_MEM_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_mem_data4::W](W) writer structure"]
impl crate::Writable for TX_MEM_DATA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_MEM_DATA4 to value 0"]
impl crate::Resettable for TX_MEM_DATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
