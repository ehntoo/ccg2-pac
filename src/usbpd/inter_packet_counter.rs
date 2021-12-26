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
#[doc = "Field `BUS_IDLE_CNT` reader - "]
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
#[doc = "Field `BUS_IDLE_CNT` writer - "]
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
#[doc = "Field `IDLE_COUNTER` reader - "]
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
#[doc = "Field `IDLE_COUNTER` writer - "]
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
#[doc = "Field `IFG_COUNTER` reader - "]
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
#[doc = "Field `IFG_COUNTER` writer - "]
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
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bus_idle_cnt(&self) -> BUS_IDLE_CNT_R {
        BUS_IDLE_CNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn idle_counter(&self) -> IDLE_COUNTER_R {
        IDLE_COUNTER_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn ifg_counter(&self) -> IFG_COUNTER_R {
        IFG_COUNTER_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bus_idle_cnt(&mut self) -> BUS_IDLE_CNT_W {
        BUS_IDLE_CNT_W { w: self }
    }
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn idle_counter(&mut self) -> IDLE_COUNTER_W {
        IDLE_COUNTER_W { w: self }
    }
    #[doc = "Bits 21:31"]
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inter_packet_counter](index.html) module"]
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
