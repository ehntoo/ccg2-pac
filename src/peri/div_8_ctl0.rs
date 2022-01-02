#[doc = "Register `DIV_8_CTL0` reader"]
pub struct R(crate::R<DIV_8_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_8_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_8_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_8_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_8_CTL0` writer"]
pub struct W(crate::W<DIV_8_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_8_CTL0_SPEC>;
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
impl From<crate::W<DIV_8_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_8_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT8_DIV` reader - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50% duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 % duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to \"0\" when transitioning from DeepSleep to Active power mode."]
pub struct INT8_DIV_R(crate::FieldReader<u8, u8>);
impl INT8_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT8_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT8_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT8_DIV` writer - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50% duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 % duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to \"0\" when transitioning from DeepSleep to Active power mode."]
pub struct INT8_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INT8_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50% duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 % duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to \"0\" when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&self) -> INT8_DIV_R {
        INT8_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50% duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 % duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to \"0\" when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&mut self) -> INT8_DIV_W {
        INT8_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divider control register (for 8.0 divider) Smallest of the divider types.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_8_ctl0](index.html) module"]
pub struct DIV_8_CTL0_SPEC;
impl crate::RegisterSpec for DIV_8_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_8_ctl0::R](R) reader structure"]
impl crate::Readable for DIV_8_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_8_ctl0::W](W) writer structure"]
impl crate::Writable for DIV_8_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_8_CTL0 to value 0"]
impl crate::Resettable for DIV_8_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
