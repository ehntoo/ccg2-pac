#[doc = "Register `RX_RESERVED2_ORDER_SET` reader"]
pub struct R(crate::R<RX_RESERVED2_ORDER_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_RESERVED2_ORDER_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_RESERVED2_ORDER_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_RESERVED2_ORDER_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_RESERVED2_ORDER_SET` writer"]
pub struct W(crate::W<RX_RESERVED2_ORDER_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_RESERVED2_ORDER_SET_SPEC>;
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
impl From<crate::W<RX_RESERVED2_ORDER_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_RESERVED2_ORDER_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_RESERVED2_OS` reader - "]
pub struct RX_RESERVED2_OS_R(crate::FieldReader<u32, u32>);
impl RX_RESERVED2_OS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RX_RESERVED2_OS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RESERVED2_OS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_RESERVED2_OS` writer - "]
pub struct RX_RESERVED2_OS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RESERVED2_OS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_reserved2_os(&self) -> RX_RESERVED2_OS_R {
        RX_RESERVED2_OS_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_reserved2_os(&mut self) -> RX_RESERVED2_OS_W {
        RX_RESERVED2_OS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_reserved2_order_set](index.html) module"]
pub struct RX_RESERVED2_ORDER_SET_SPEC;
impl crate::RegisterSpec for RX_RESERVED2_ORDER_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_reserved2_order_set::R](R) reader structure"]
impl crate::Readable for RX_RESERVED2_ORDER_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_reserved2_order_set::W](W) writer structure"]
impl crate::Writable for RX_RESERVED2_ORDER_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_RESERVED2_ORDER_SET to value 0"]
impl crate::Resettable for RX_RESERVED2_ORDER_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
