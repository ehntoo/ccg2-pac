#[doc = "Register `PC2` reader"]
pub struct R(crate::R<PC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC2` writer"]
pub struct W(crate::W<PC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC2_SPEC>;
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
impl From<crate::W<PC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP_DIS0` reader - "]
pub struct INP_DIS0_R(crate::FieldReader<bool, bool>);
impl INP_DIS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS0` writer - "]
pub struct INP_DIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS0_W<'a> {
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
#[doc = "Field `INP_DIS1` reader - "]
pub struct INP_DIS1_R(crate::FieldReader<bool, bool>);
impl INP_DIS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS1` writer - "]
pub struct INP_DIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS1_W<'a> {
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
#[doc = "Field `INP_DIS2` reader - "]
pub struct INP_DIS2_R(crate::FieldReader<bool, bool>);
impl INP_DIS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS2` writer - "]
pub struct INP_DIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `INP_DIS3` reader - "]
pub struct INP_DIS3_R(crate::FieldReader<bool, bool>);
impl INP_DIS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS3` writer - "]
pub struct INP_DIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `INP_DIS4` reader - "]
pub struct INP_DIS4_R(crate::FieldReader<bool, bool>);
impl INP_DIS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS4` writer - "]
pub struct INP_DIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `INP_DIS5` reader - "]
pub struct INP_DIS5_R(crate::FieldReader<bool, bool>);
impl INP_DIS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS5` writer - "]
pub struct INP_DIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `INP_DIS6` reader - "]
pub struct INP_DIS6_R(crate::FieldReader<bool, bool>);
impl INP_DIS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS6` writer - "]
pub struct INP_DIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `INP_DIS7` reader - "]
pub struct INP_DIS7_R(crate::FieldReader<bool, bool>);
impl INP_DIS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INP_DIS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_DIS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_DIS7` writer - "]
pub struct INP_DIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_DIS7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn inp_dis0(&self) -> INP_DIS0_R {
        INP_DIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inp_dis1(&self) -> INP_DIS1_R {
        INP_DIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn inp_dis2(&self) -> INP_DIS2_R {
        INP_DIS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inp_dis3(&self) -> INP_DIS3_R {
        INP_DIS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inp_dis4(&self) -> INP_DIS4_R {
        INP_DIS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inp_dis5(&self) -> INP_DIS5_R {
        INP_DIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inp_dis6(&self) -> INP_DIS6_R {
        INP_DIS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inp_dis7(&self) -> INP_DIS7_R {
        INP_DIS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn inp_dis0(&mut self) -> INP_DIS0_W {
        INP_DIS0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inp_dis1(&mut self) -> INP_DIS1_W {
        INP_DIS1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn inp_dis2(&mut self) -> INP_DIS2_W {
        INP_DIS2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inp_dis3(&mut self) -> INP_DIS3_W {
        INP_DIS3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inp_dis4(&mut self) -> INP_DIS4_W {
        INP_DIS4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inp_dis5(&mut self) -> INP_DIS5_W {
        INP_DIS5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inp_dis6(&mut self) -> INP_DIS6_W {
        INP_DIS6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inp_dis7(&mut self) -> INP_DIS7_W {
        INP_DIS7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc2](index.html) module"]
pub struct PC2_SPEC;
impl crate::RegisterSpec for PC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc2::R](R) reader structure"]
impl crate::Readable for PC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc2::W](W) writer structure"]
impl crate::Writable for PC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC2 to value 0"]
impl crate::Resettable for PC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
