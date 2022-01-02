#[doc = "Register `PRIV_FLASH` reader"]
pub struct R(crate::R<PRIV_FLASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIV_FLASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIV_FLASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIV_FLASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIV_FLASH` writer"]
pub struct W(crate::W<PRIV_FLASH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIV_FLASH_SPEC>;
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
impl From<crate::W<PRIV_FLASH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIV_FLASH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PROT_LIMIT` reader - Indicates the limit where the privileged area of flash starts in increments of 256 Bytes. \"0\": Entire flash is Privileged. \"1\": First 256 Bytes are User accessable. Any number larger than the size of the flash indicates that the entire flash is user mode accessible. Note that SuperVisory rows are always User accessable. If FLASH_PROT_LIMIT defines a non-empty privileged area, the boot ROM will assume that a system call table exists at the beginning of the Flash privileged area and use it for all SystemCalls made using SYSREQ."]
pub struct FLASH_PROT_LIMIT_R(crate::FieldReader<u16, u16>);
impl FLASH_PROT_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FLASH_PROT_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PROT_LIMIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PROT_LIMIT` writer - Indicates the limit where the privileged area of flash starts in increments of 256 Bytes. \"0\": Entire flash is Privileged. \"1\": First 256 Bytes are User accessable. Any number larger than the size of the flash indicates that the entire flash is user mode accessible. Note that SuperVisory rows are always User accessable. If FLASH_PROT_LIMIT defines a non-empty privileged area, the boot ROM will assume that a system call table exists at the beginning of the Flash privileged area and use it for all SystemCalls made using SYSREQ."]
pub struct FLASH_PROT_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PROT_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Indicates the limit where the privileged area of flash starts in increments of 256 Bytes. \"0\": Entire flash is Privileged. \"1\": First 256 Bytes are User accessable. Any number larger than the size of the flash indicates that the entire flash is user mode accessible. Note that SuperVisory rows are always User accessable. If FLASH_PROT_LIMIT defines a non-empty privileged area, the boot ROM will assume that a system call table exists at the beginning of the Flash privileged area and use it for all SystemCalls made using SYSREQ."]
    #[inline(always)]
    pub fn flash_prot_limit(&self) -> FLASH_PROT_LIMIT_R {
        FLASH_PROT_LIMIT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Indicates the limit where the privileged area of flash starts in increments of 256 Bytes. \"0\": Entire flash is Privileged. \"1\": First 256 Bytes are User accessable. Any number larger than the size of the flash indicates that the entire flash is user mode accessible. Note that SuperVisory rows are always User accessable. If FLASH_PROT_LIMIT defines a non-empty privileged area, the boot ROM will assume that a system call table exists at the beginning of the Flash privileged area and use it for all SystemCalls made using SYSREQ."]
    #[inline(always)]
    pub fn flash_prot_limit(&mut self) -> FLASH_PROT_LIMIT_W {
        FLASH_PROT_LIMIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM privilege register User mode accesses to a privileged address result in an+C48 AHB-Lite bus error. If the regular flash memory capacity is not a power of two, the regular flash memory region has an unpopulated/unaccounted memory are (at the end of the regular flash memory region). A user mode access to an unpopulated, privileged area (as indicated by the LIMIT field(s)) address results in an AHB-Lite bus error. A user mode access to a unpopulated area, without any access violations, behaves as follows: Reads return \"0\" and writes are ignore (RZWI).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priv_flash](index.html) module"]
pub struct PRIV_FLASH_SPEC;
impl crate::RegisterSpec for PRIV_FLASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priv_flash::R](R) reader structure"]
impl crate::Readable for PRIV_FLASH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priv_flash::W](W) writer structure"]
impl crate::Writable for PRIV_FLASH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIV_FLASH to value 0"]
impl crate::Resettable for PRIV_FLASH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
