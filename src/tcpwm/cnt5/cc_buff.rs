#[doc = "Register `CC_BUFF` reader"]
pub struct R(crate::R<CC_BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_BUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC_BUFF` writer"]
pub struct W(crate::W<CC_BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_BUFF_SPEC>;
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
impl From<crate::W<CC_BUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC` reader - Additional buffer for counter CC register."]
pub struct CC_R(crate::FieldReader<u16, u16>);
impl CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` writer - Additional buffer for counter CC register."]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Additional buffer for counter CC register."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Additional buffer for counter CC register."]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter buffered compare/capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_buff](index.html) module"]
pub struct CC_BUFF_SPEC;
impl crate::RegisterSpec for CC_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc_buff::R](R) reader structure"]
impl crate::Readable for CC_BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc_buff::W](W) writer structure"]
impl crate::Writable for CC_BUFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC_BUFF to value 0xffff"]
impl crate::Resettable for CC_BUFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
