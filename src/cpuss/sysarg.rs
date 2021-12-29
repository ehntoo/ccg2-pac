#[doc = "Register `SYSARG` reader"]
pub struct R(crate::R<SYSARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSARG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSARG` writer"]
pub struct W(crate::W<SYSARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSARG_SPEC>;
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
impl From<crate::W<SYSARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCALL_ARG` reader - "]
pub struct SYSCALL_ARG_R(crate::FieldReader<u32, u32>);
impl SYSCALL_ARG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SYSCALL_ARG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCALL_ARG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCALL_ARG` writer - "]
pub struct SYSCALL_ARG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCALL_ARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscall_arg(&self) -> SYSCALL_ARG_R {
        SYSCALL_ARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscall_arg(&mut self) -> SYSCALL_ARG_W {
        SYSCALL_ARG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysarg](index.html) module"]
pub struct SYSARG_SPEC;
impl crate::RegisterSpec for SYSARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysarg::R](R) reader structure"]
impl crate::Readable for SYSARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysarg::W](W) writer structure"]
impl crate::Writable for SYSARG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSARG to value 0"]
impl crate::Resettable for SYSARG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
