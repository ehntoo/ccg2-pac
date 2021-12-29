#[doc = "Register `BIST_ADDR` reader"]
pub struct R(crate::R<BIST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COL_ADDR` reader - "]
pub struct COL_ADDR_R(crate::FieldReader<u16, u16>);
impl COL_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COL_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COL_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROW_ADDR` reader - "]
pub struct ROW_ADDR_R(crate::FieldReader<u16, u16>);
impl ROW_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ROW_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROW_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn col_addr(&self) -> COL_ADDR_R {
        COL_ADDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn row_addr(&self) -> ROW_ADDR_R {
        ROW_ADDR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_addr](index.html) module"]
pub struct BIST_ADDR_SPEC;
impl crate::RegisterSpec for BIST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_addr::R](R) reader structure"]
impl crate::Readable for BIST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BIST_ADDR to value 0"]
impl crate::Resettable for BIST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
