#[doc = "Register `RX_CTRL` reader"]
pub struct R(crate::R<RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CTRL` writer"]
pub struct W(crate::W<RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CTRL_SPEC>;
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
impl From<crate::W<RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_WIDTH` reader - "]
pub struct DATA_WIDTH_R(crate::FieldReader<u8, u8>);
impl DATA_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_WIDTH` writer - "]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `MSB_FIRST` reader - "]
pub struct MSB_FIRST_R(crate::FieldReader<bool, bool>);
impl MSB_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSB_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSB_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSB_FIRST` writer - "]
pub struct MSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_FIRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `MEDIAN` reader - "]
pub struct MEDIAN_R(crate::FieldReader<bool, bool>);
impl MEDIAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEDIAN` writer - "]
pub struct MEDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn median(&self) -> MEDIAN_R {
        MEDIAN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn median(&mut self) -> MEDIAN_W {
        MEDIAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl](index.html) module"]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ctrl::R](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ctrl::W](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CTRL to value 0x0107"]
impl crate::Resettable for RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0107
    }
}
