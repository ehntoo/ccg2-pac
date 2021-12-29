#[doc = "Register `BIST_STATUS` reader"]
pub struct R(crate::R<BIST_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_STATUS` writer"]
pub struct W(crate::W<BIST_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_STATUS_SPEC>;
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
impl From<crate::W<BIST_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUB_STEP` reader - "]
pub struct SUB_STEP_R(crate::FieldReader<u8, u8>);
impl SUB_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUB_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUB_STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STEP` reader - "]
pub struct STEP_R(crate::FieldReader<u8, u8>);
impl STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAM_A {
    #[doc = "0: `0`"]
    SRAM_CPUSS = 0,
    #[doc = "1: `1`"]
    SRAM_DMAC = 1,
    #[doc = "2: `10`"]
    EXT_SRAM_0 = 2,
    #[doc = "3: `11`"]
    EXT_SRAM_1 = 3,
    #[doc = "4: `100`"]
    EXT_SRAM_2 = 4,
    #[doc = "5: `101`"]
    EXT_SRAM_3 = 5,
    #[doc = "6: `110`"]
    EXT_SRAM_4 = 6,
    #[doc = "7: `111`"]
    EXT_SRAM_5 = 7,
    #[doc = "8: `1000`"]
    EXT_SRAM_6 = 8,
    #[doc = "9: `1001`"]
    EXT_SRAM_7 = 9,
    #[doc = "10: `1010`"]
    EXT_SRAM_8 = 10,
    #[doc = "11: `1011`"]
    EXT_SRAM_9 = 11,
    #[doc = "12: `1100`"]
    EXT_SRAM_10 = 12,
    #[doc = "13: `1101`"]
    EXT_SRAM_11 = 13,
    #[doc = "14: `1110`"]
    EXT_SRAM_12 = 14,
    #[doc = "15: `1111`"]
    EXT_SRAM_13 = 15,
}
impl From<SRAM_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAM` reader - "]
pub struct SRAM_R(crate::FieldReader<u8, SRAM_A>);
impl SRAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_A {
        match self.bits {
            0 => SRAM_A::SRAM_CPUSS,
            1 => SRAM_A::SRAM_DMAC,
            2 => SRAM_A::EXT_SRAM_0,
            3 => SRAM_A::EXT_SRAM_1,
            4 => SRAM_A::EXT_SRAM_2,
            5 => SRAM_A::EXT_SRAM_3,
            6 => SRAM_A::EXT_SRAM_4,
            7 => SRAM_A::EXT_SRAM_5,
            8 => SRAM_A::EXT_SRAM_6,
            9 => SRAM_A::EXT_SRAM_7,
            10 => SRAM_A::EXT_SRAM_8,
            11 => SRAM_A::EXT_SRAM_9,
            12 => SRAM_A::EXT_SRAM_10,
            13 => SRAM_A::EXT_SRAM_11,
            14 => SRAM_A::EXT_SRAM_12,
            15 => SRAM_A::EXT_SRAM_13,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_CPUSS`"]
    #[inline(always)]
    pub fn is_sram_cpuss(&self) -> bool {
        **self == SRAM_A::SRAM_CPUSS
    }
    #[doc = "Checks if the value of the field is `SRAM_DMAC`"]
    #[inline(always)]
    pub fn is_sram_dmac(&self) -> bool {
        **self == SRAM_A::SRAM_DMAC
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_0`"]
    #[inline(always)]
    pub fn is_ext_sram_0(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_0
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_1`"]
    #[inline(always)]
    pub fn is_ext_sram_1(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_1
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_2`"]
    #[inline(always)]
    pub fn is_ext_sram_2(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_2
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_3`"]
    #[inline(always)]
    pub fn is_ext_sram_3(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_3
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_4`"]
    #[inline(always)]
    pub fn is_ext_sram_4(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_4
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_5`"]
    #[inline(always)]
    pub fn is_ext_sram_5(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_5
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_6`"]
    #[inline(always)]
    pub fn is_ext_sram_6(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_6
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_7`"]
    #[inline(always)]
    pub fn is_ext_sram_7(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_7
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_8`"]
    #[inline(always)]
    pub fn is_ext_sram_8(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_8
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_9`"]
    #[inline(always)]
    pub fn is_ext_sram_9(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_9
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_10`"]
    #[inline(always)]
    pub fn is_ext_sram_10(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_10
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_11`"]
    #[inline(always)]
    pub fn is_ext_sram_11(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_11
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_12`"]
    #[inline(always)]
    pub fn is_ext_sram_12(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_12
    }
    #[doc = "Checks if the value of the field is `EXT_SRAM_13`"]
    #[inline(always)]
    pub fn is_ext_sram_13(&self) -> bool {
        **self == SRAM_A::EXT_SRAM_13
    }
}
impl core::ops::Deref for SRAM_R {
    type Target = crate::FieldReader<u8, SRAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAIL` reader - "]
pub struct FAIL_R(crate::FieldReader<bool, bool>);
impl FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAIL` writer - "]
pub struct FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sub_step(&self) -> SUB_STEP_R {
        SUB_STEP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fail(&mut self) -> FAIL_W {
        FAIL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_status](index.html) module"]
pub struct BIST_STATUS_SPEC;
impl crate::RegisterSpec for BIST_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_status::R](R) reader structure"]
impl crate::Readable for BIST_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_status::W](W) writer structure"]
impl crate::Writable for BIST_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_STATUS to value 0"]
impl crate::Resettable for BIST_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
