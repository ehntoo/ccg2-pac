#[doc = "Register `BIST_CMD` reader"]
pub struct R(crate::R<BIST_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_CMD` writer"]
pub struct W(crate::W<BIST_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_CMD_SPEC>;
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
impl From<crate::W<BIST_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_GO` reader - 1': Start SRAM BIST. Hardware set this field to '0' when BIST is completed. SRAM BIST is functional up to 48 MHz. Note that this field is mutually exclusive with the \"SROM_GO\" field."]
pub struct SRAM_GO_R(crate::FieldReader<bool, bool>);
impl SRAM_GO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_GO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_GO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_GO` writer - 1': Start SRAM BIST. Hardware set this field to '0' when BIST is completed. SRAM BIST is functional up to 48 MHz. Note that this field is mutually exclusive with the \"SROM_GO\" field."]
pub struct SRAM_GO_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_GO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SROM_GO` reader - 1': Start ROM BIST. Hardware set this field to '0' when BIST is completed. SROM BIST is functional up to 24 MHz."]
pub struct SROM_GO_R(crate::FieldReader<bool, bool>);
impl SROM_GO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SROM_GO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SROM_GO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SROM_GO` writer - 1': Start ROM BIST. Hardware set this field to '0' when BIST is completed. SROM BIST is functional up to 24 MHz."]
pub struct SROM_GO_W<'a> {
    w: &'a mut W,
}
impl<'a> SROM_GO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1': Start SRAM BIST. Hardware set this field to '0' when BIST is completed. SRAM BIST is functional up to 48 MHz. Note that this field is mutually exclusive with the \"SROM_GO\" field."]
    #[inline(always)]
    pub fn sram_go(&self) -> SRAM_GO_R {
        SRAM_GO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1': Start ROM BIST. Hardware set this field to '0' when BIST is completed. SROM BIST is functional up to 24 MHz."]
    #[inline(always)]
    pub fn srom_go(&self) -> SROM_GO_R {
        SROM_GO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1': Start SRAM BIST. Hardware set this field to '0' when BIST is completed. SRAM BIST is functional up to 48 MHz. Note that this field is mutually exclusive with the \"SROM_GO\" field."]
    #[inline(always)]
    pub fn sram_go(&mut self) -> SRAM_GO_W {
        SRAM_GO_W { w: self }
    }
    #[doc = "Bit 1 - 1': Start ROM BIST. Hardware set this field to '0' when BIST is completed. SROM BIST is functional up to 24 MHz."]
    #[inline(always)]
    pub fn srom_go(&mut self) -> SROM_GO_W {
        SROM_GO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bist command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_cmd](index.html) module"]
pub struct BIST_CMD_SPEC;
impl crate::RegisterSpec for BIST_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_cmd::R](R) reader structure"]
impl crate::Readable for BIST_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_cmd::W](W) writer structure"]
impl crate::Writable for BIST_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_CMD to value 0"]
impl crate::Resettable for BIST_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
