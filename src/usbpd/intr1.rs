#[doc = "Register `INTR1` reader"]
pub struct R(crate::R<INTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR1` writer"]
pub struct W(crate::W<INTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR1_SPEC>;
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
impl From<crate::W<INTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCONN1_CHANGED` reader - "]
pub struct VCONN1_CHANGED_R(crate::FieldReader<bool, bool>);
impl VCONN1_CHANGED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN1_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN1_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN1_CHANGED` writer - "]
pub struct VCONN1_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> VCONN1_CHANGED_W<'a> {
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
#[doc = "Field `VCONN2_CHANGED` reader - "]
pub struct VCONN2_CHANGED_R(crate::FieldReader<bool, bool>);
impl VCONN2_CHANGED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCONN2_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCONN2_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCONN2_CHANGED` writer - "]
pub struct VCONN2_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> VCONN2_CHANGED_W<'a> {
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
#[doc = "Field `CC1_CHANGED` reader - "]
pub struct CC1_CHANGED_R(crate::FieldReader<bool, bool>);
impl CC1_CHANGED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC1_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1_CHANGED` writer - "]
pub struct CC1_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_CHANGED_W<'a> {
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
#[doc = "Field `CC2_CHANGED` reader - "]
pub struct CC2_CHANGED_R(crate::FieldReader<bool, bool>);
impl CC2_CHANGED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC2_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2_CHANGED` writer - "]
pub struct CC2_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2_CHANGED_W<'a> {
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
#[doc = "Field `VCMP_LA_CHANGED` reader - "]
pub struct VCMP_LA_CHANGED_R(crate::FieldReader<bool, bool>);
impl VCMP_LA_CHANGED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_LA_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_LA_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_LA_CHANGED` writer - "]
pub struct VCMP_LA_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_LA_CHANGED_W<'a> {
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
#[doc = "Field `VCMP_UP_CHANGED` reader - "]
pub struct VCMP_UP_CHANGED_R(crate::FieldReader<bool, bool>);
impl VCMP_UP_CHANGED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_UP_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_UP_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_UP_CHANGED` writer - "]
pub struct VCMP_UP_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_UP_CHANGED_W<'a> {
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
#[doc = "Field `VCMP_DN_CHANGED` reader - "]
pub struct VCMP_DN_CHANGED_R(crate::FieldReader<bool, bool>);
impl VCMP_DN_CHANGED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCMP_DN_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCMP_DN_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCMP_DN_CHANGED` writer - "]
pub struct VCMP_DN_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> VCMP_DN_CHANGED_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vconn1_changed(&self) -> VCONN1_CHANGED_R {
        VCONN1_CHANGED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vconn2_changed(&self) -> VCONN2_CHANGED_R {
        VCONN2_CHANGED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc1_changed(&self) -> CC1_CHANGED_R {
        CC1_CHANGED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc2_changed(&self) -> CC2_CHANGED_R {
        CC2_CHANGED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vcmp_la_changed(&self) -> VCMP_LA_CHANGED_R {
        VCMP_LA_CHANGED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vcmp_up_changed(&self) -> VCMP_UP_CHANGED_R {
        VCMP_UP_CHANGED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vcmp_dn_changed(&self) -> VCMP_DN_CHANGED_R {
        VCMP_DN_CHANGED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vconn1_changed(&mut self) -> VCONN1_CHANGED_W {
        VCONN1_CHANGED_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vconn2_changed(&mut self) -> VCONN2_CHANGED_W {
        VCONN2_CHANGED_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc1_changed(&mut self) -> CC1_CHANGED_W {
        CC1_CHANGED_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc2_changed(&mut self) -> CC2_CHANGED_W {
        CC2_CHANGED_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vcmp_la_changed(&mut self) -> VCMP_LA_CHANGED_W {
        VCMP_LA_CHANGED_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vcmp_up_changed(&mut self) -> VCMP_UP_CHANGED_W {
        VCMP_UP_CHANGED_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vcmp_dn_changed(&mut self) -> VCMP_DN_CHANGED_W {
        VCMP_DN_CHANGED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr1](index.html) module"]
pub struct INTR1_SPEC;
impl crate::RegisterSpec for INTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr1::R](R) reader structure"]
impl crate::Readable for INTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr1::W](W) writer structure"]
impl crate::Writable for INTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR1 to value 0"]
impl crate::Resettable for INTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
