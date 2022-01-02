#[doc = "Register `CLK_IMO_TRIM2` reader"]
pub struct R(crate::R<CLK_IMO_TRIM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_IMO_TRIM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_IMO_TRIM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_IMO_TRIM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_IMO_TRIM2` writer"]
pub struct W(crate::W<CLK_IMO_TRIM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_IMO_TRIM2_SPEC>;
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
impl From<crate::W<CLK_IMO_TRIM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_IMO_TRIM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSOFFSET` reader - Frequency trim bits. These bits are not trimmed during manufacturing and kept at 0 under normal operation. This field is hardware updated during USB osclock mode. This field is mapped to the least significant bits of the IMO trim imo_clk_trim\\[2:0\\]. The step size of 1 LSB on this field is approximately 15 kHz."]
pub struct FSOFFSET_R(crate::FieldReader<u8, u8>);
impl FSOFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSOFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSOFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSOFFSET` writer - Frequency trim bits. These bits are not trimmed during manufacturing and kept at 0 under normal operation. This field is hardware updated during USB osclock mode. This field is mapped to the least significant bits of the IMO trim imo_clk_trim\\[2:0\\]. The step size of 1 LSB on this field is approximately 15 kHz."]
pub struct FSOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Frequency trim bits. These bits are not trimmed during manufacturing and kept at 0 under normal operation. This field is hardware updated during USB osclock mode. This field is mapped to the least significant bits of the IMO trim imo_clk_trim\\[2:0\\]. The step size of 1 LSB on this field is approximately 15 kHz."]
    #[inline(always)]
    pub fn fsoffset(&self) -> FSOFFSET_R {
        FSOFFSET_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frequency trim bits. These bits are not trimmed during manufacturing and kept at 0 under normal operation. This field is hardware updated during USB osclock mode. This field is mapped to the least significant bits of the IMO trim imo_clk_trim\\[2:0\\]. The step size of 1 LSB on this field is approximately 15 kHz."]
    #[inline(always)]
    pub fn fsoffset(&mut self) -> FSOFFSET_W {
        FSOFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMO Trim Register IMO Trim Bits. Entire register is engineering only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_imo_trim2](index.html) module"]
pub struct CLK_IMO_TRIM2_SPEC;
impl crate::RegisterSpec for CLK_IMO_TRIM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_imo_trim2::R](R) reader structure"]
impl crate::Readable for CLK_IMO_TRIM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_imo_trim2::W](W) writer structure"]
impl crate::Writable for CLK_IMO_TRIM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_IMO_TRIM2 to value 0"]
impl crate::Resettable for CLK_IMO_TRIM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
