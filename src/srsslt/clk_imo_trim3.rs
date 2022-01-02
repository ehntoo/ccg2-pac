#[doc = "Register `CLK_IMO_TRIM3` reader"]
pub struct R(crate::R<CLK_IMO_TRIM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_IMO_TRIM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_IMO_TRIM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_IMO_TRIM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_IMO_TRIM3` writer"]
pub struct W(crate::W<CLK_IMO_TRIM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_IMO_TRIM3_SPEC>;
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
impl From<crate::W<CLK_IMO_TRIM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_IMO_TRIM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEPSIZE` reader - IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
pub struct STEPSIZE_R(crate::FieldReader<u8, u8>);
impl STEPSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STEPSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STEPSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STEPSIZE` writer - IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
pub struct STEPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `TCTRIM` reader - IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
pub struct TCTRIM_R(crate::FieldReader<u8, u8>);
impl TCTRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCTRIM` writer - IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
pub struct TCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn stepsize(&self) -> STEPSIZE_R {
        STEPSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn tctrim(&self) -> TCTRIM_R {
        TCTRIM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn stepsize(&mut self) -> STEPSIZE_W {
        STEPSIZE_W { w: self }
    }
    #[doc = "Bits 5:6 - IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn tctrim(&mut self) -> TCTRIM_W {
        TCTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMO Trim Register IMO Trim Bits. Entire register is engineering only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_imo_trim3](index.html) module"]
pub struct CLK_IMO_TRIM3_SPEC;
impl crate::RegisterSpec for CLK_IMO_TRIM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_imo_trim3::R](R) reader structure"]
impl crate::Readable for CLK_IMO_TRIM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_imo_trim3::W](W) writer structure"]
impl crate::Writable for CLK_IMO_TRIM3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_IMO_TRIM3 to value 0x50"]
impl crate::Resettable for CLK_IMO_TRIM3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x50
    }
}
