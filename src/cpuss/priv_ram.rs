#[doc = "Register `PRIV_RAM` reader"]
pub struct R(crate::R<PRIV_RAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIV_RAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIV_RAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIV_RAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIV_RAM` writer"]
pub struct W(crate::W<PRIV_RAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIV_RAM_SPEC>;
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
impl From<crate::W<PRIV_RAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIV_RAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_PROT_LIMIT` reader - "]
pub struct RAM_PROT_LIMIT_R(crate::FieldReader<u16, u16>);
impl RAM_PROT_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RAM_PROT_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_PROT_LIMIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_PROT_LIMIT` writer - "]
pub struct RAM_PROT_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_PROT_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn ram_prot_limit(&self) -> RAM_PROT_LIMIT_R {
        RAM_PROT_LIMIT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn ram_prot_limit(&mut self) -> RAM_PROT_LIMIT_W {
        RAM_PROT_LIMIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priv_ram](index.html) module"]
pub struct PRIV_RAM_SPEC;
impl crate::RegisterSpec for PRIV_RAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priv_ram::R](R) reader structure"]
impl crate::Readable for PRIV_RAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priv_ram::W](W) writer structure"]
impl crate::Writable for PRIV_RAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIV_RAM to value 0"]
impl crate::Resettable for PRIV_RAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
