#[doc = "Register `TST_DDFT_CTRL` reader"]
pub struct R(crate::R<TST_DDFT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_DDFT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_DDFT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_DDFT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_DDFT_CTRL` writer"]
pub struct W(crate::W<TST_DDFT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_DDFT_CTRL_SPEC>;
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
impl From<crate::W<TST_DDFT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_DDFT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFT_SEL0_A {
    #[doc = "0: `0`"]
    SRC0 = 0,
    #[doc = "1: `1`"]
    SRC1 = 1,
    #[doc = "2: `10`"]
    SRC2 = 2,
    #[doc = "3: `11`"]
    SRC3 = 3,
    #[doc = "4: `100`"]
    SRC4 = 4,
    #[doc = "5: `101`"]
    SRC5 = 5,
    #[doc = "6: `110`"]
    SRC6 = 6,
    #[doc = "7: `111`"]
    SRC7 = 7,
    #[doc = "8: `1000`"]
    CLK0 = 8,
    #[doc = "9: `1001`"]
    CLK1 = 9,
    #[doc = "10: `1010`"]
    PWR0 = 10,
    #[doc = "11: `1011`"]
    PWR1 = 11,
    #[doc = "12: `1100`"]
    RES0 = 12,
    #[doc = "13: `1101`"]
    RES1 = 13,
    #[doc = "14: `1110`"]
    ADFT_COMP = 14,
    #[doc = "15: `1111`"]
    VSS = 15,
}
impl From<DFT_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DFT_SEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DFT_SEL0` reader - "]
pub struct DFT_SEL0_R(crate::FieldReader<u8, DFT_SEL0_A>);
impl DFT_SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFT_SEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFT_SEL0_A {
        match self.bits {
            0 => DFT_SEL0_A::SRC0,
            1 => DFT_SEL0_A::SRC1,
            2 => DFT_SEL0_A::SRC2,
            3 => DFT_SEL0_A::SRC3,
            4 => DFT_SEL0_A::SRC4,
            5 => DFT_SEL0_A::SRC5,
            6 => DFT_SEL0_A::SRC6,
            7 => DFT_SEL0_A::SRC7,
            8 => DFT_SEL0_A::CLK0,
            9 => DFT_SEL0_A::CLK1,
            10 => DFT_SEL0_A::PWR0,
            11 => DFT_SEL0_A::PWR1,
            12 => DFT_SEL0_A::RES0,
            13 => DFT_SEL0_A::RES1,
            14 => DFT_SEL0_A::ADFT_COMP,
            15 => DFT_SEL0_A::VSS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRC0`"]
    #[inline(always)]
    pub fn is_src0(&self) -> bool {
        **self == DFT_SEL0_A::SRC0
    }
    #[doc = "Checks if the value of the field is `SRC1`"]
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        **self == DFT_SEL0_A::SRC1
    }
    #[doc = "Checks if the value of the field is `SRC2`"]
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        **self == DFT_SEL0_A::SRC2
    }
    #[doc = "Checks if the value of the field is `SRC3`"]
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        **self == DFT_SEL0_A::SRC3
    }
    #[doc = "Checks if the value of the field is `SRC4`"]
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        **self == DFT_SEL0_A::SRC4
    }
    #[doc = "Checks if the value of the field is `SRC5`"]
    #[inline(always)]
    pub fn is_src5(&self) -> bool {
        **self == DFT_SEL0_A::SRC5
    }
    #[doc = "Checks if the value of the field is `SRC6`"]
    #[inline(always)]
    pub fn is_src6(&self) -> bool {
        **self == DFT_SEL0_A::SRC6
    }
    #[doc = "Checks if the value of the field is `SRC7`"]
    #[inline(always)]
    pub fn is_src7(&self) -> bool {
        **self == DFT_SEL0_A::SRC7
    }
    #[doc = "Checks if the value of the field is `CLK0`"]
    #[inline(always)]
    pub fn is_clk0(&self) -> bool {
        **self == DFT_SEL0_A::CLK0
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        **self == DFT_SEL0_A::CLK1
    }
    #[doc = "Checks if the value of the field is `PWR0`"]
    #[inline(always)]
    pub fn is_pwr0(&self) -> bool {
        **self == DFT_SEL0_A::PWR0
    }
    #[doc = "Checks if the value of the field is `PWR1`"]
    #[inline(always)]
    pub fn is_pwr1(&self) -> bool {
        **self == DFT_SEL0_A::PWR1
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        **self == DFT_SEL0_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        **self == DFT_SEL0_A::RES1
    }
    #[doc = "Checks if the value of the field is `ADFT_COMP`"]
    #[inline(always)]
    pub fn is_adft_comp(&self) -> bool {
        **self == DFT_SEL0_A::ADFT_COMP
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        **self == DFT_SEL0_A::VSS
    }
}
impl core::ops::Deref for DFT_SEL0_R {
    type Target = crate::FieldReader<u8, DFT_SEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_SEL0` writer - "]
pub struct DFT_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFT_SEL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn src0(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn src5(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn src6(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn src7(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SRC7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn clk0(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::CLK0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn clk1(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::CLK1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pwr0(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::PWR0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn pwr1(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::PWR1)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::RES0)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::RES1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn adft_comp(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::ADFT_COMP)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::VSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFT_SEL1_A {
    #[doc = "0: `0`"]
    SRC0 = 0,
    #[doc = "1: `1`"]
    SRC1 = 1,
    #[doc = "2: `10`"]
    SRC2 = 2,
    #[doc = "3: `11`"]
    SRC3 = 3,
    #[doc = "4: `100`"]
    SRC4 = 4,
    #[doc = "5: `101`"]
    SRC5 = 5,
    #[doc = "6: `110`"]
    SRC6 = 6,
    #[doc = "7: `111`"]
    SRC7 = 7,
    #[doc = "8: `1000`"]
    CLK0 = 8,
    #[doc = "9: `1001`"]
    CLK1 = 9,
    #[doc = "10: `1010`"]
    PWR0 = 10,
    #[doc = "11: `1011`"]
    PWR1 = 11,
    #[doc = "12: `1100`"]
    RES0 = 12,
    #[doc = "13: `1101`"]
    RES1 = 13,
    #[doc = "14: `1110`"]
    ADFT_COMP = 14,
    #[doc = "15: `1111`"]
    VSS = 15,
}
impl From<DFT_SEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: DFT_SEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DFT_SEL1` reader - "]
pub struct DFT_SEL1_R(crate::FieldReader<u8, DFT_SEL1_A>);
impl DFT_SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFT_SEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFT_SEL1_A {
        match self.bits {
            0 => DFT_SEL1_A::SRC0,
            1 => DFT_SEL1_A::SRC1,
            2 => DFT_SEL1_A::SRC2,
            3 => DFT_SEL1_A::SRC3,
            4 => DFT_SEL1_A::SRC4,
            5 => DFT_SEL1_A::SRC5,
            6 => DFT_SEL1_A::SRC6,
            7 => DFT_SEL1_A::SRC7,
            8 => DFT_SEL1_A::CLK0,
            9 => DFT_SEL1_A::CLK1,
            10 => DFT_SEL1_A::PWR0,
            11 => DFT_SEL1_A::PWR1,
            12 => DFT_SEL1_A::RES0,
            13 => DFT_SEL1_A::RES1,
            14 => DFT_SEL1_A::ADFT_COMP,
            15 => DFT_SEL1_A::VSS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRC0`"]
    #[inline(always)]
    pub fn is_src0(&self) -> bool {
        **self == DFT_SEL1_A::SRC0
    }
    #[doc = "Checks if the value of the field is `SRC1`"]
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        **self == DFT_SEL1_A::SRC1
    }
    #[doc = "Checks if the value of the field is `SRC2`"]
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        **self == DFT_SEL1_A::SRC2
    }
    #[doc = "Checks if the value of the field is `SRC3`"]
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        **self == DFT_SEL1_A::SRC3
    }
    #[doc = "Checks if the value of the field is `SRC4`"]
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        **self == DFT_SEL1_A::SRC4
    }
    #[doc = "Checks if the value of the field is `SRC5`"]
    #[inline(always)]
    pub fn is_src5(&self) -> bool {
        **self == DFT_SEL1_A::SRC5
    }
    #[doc = "Checks if the value of the field is `SRC6`"]
    #[inline(always)]
    pub fn is_src6(&self) -> bool {
        **self == DFT_SEL1_A::SRC6
    }
    #[doc = "Checks if the value of the field is `SRC7`"]
    #[inline(always)]
    pub fn is_src7(&self) -> bool {
        **self == DFT_SEL1_A::SRC7
    }
    #[doc = "Checks if the value of the field is `CLK0`"]
    #[inline(always)]
    pub fn is_clk0(&self) -> bool {
        **self == DFT_SEL1_A::CLK0
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        **self == DFT_SEL1_A::CLK1
    }
    #[doc = "Checks if the value of the field is `PWR0`"]
    #[inline(always)]
    pub fn is_pwr0(&self) -> bool {
        **self == DFT_SEL1_A::PWR0
    }
    #[doc = "Checks if the value of the field is `PWR1`"]
    #[inline(always)]
    pub fn is_pwr1(&self) -> bool {
        **self == DFT_SEL1_A::PWR1
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        **self == DFT_SEL1_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        **self == DFT_SEL1_A::RES1
    }
    #[doc = "Checks if the value of the field is `ADFT_COMP`"]
    #[inline(always)]
    pub fn is_adft_comp(&self) -> bool {
        **self == DFT_SEL1_A::ADFT_COMP
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        **self == DFT_SEL1_A::VSS
    }
}
impl core::ops::Deref for DFT_SEL1_R {
    type Target = crate::FieldReader<u8, DFT_SEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_SEL1` writer - "]
pub struct DFT_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_SEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFT_SEL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn src0(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn src5(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn src6(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn src7(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SRC7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn clk0(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::CLK0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn clk1(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::CLK1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pwr0(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::PWR0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn pwr1(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::PWR1)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::RES0)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::RES1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn adft_comp(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::ADFT_COMP)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::VSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - "]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - "]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dft_sel0(&self) -> DFT_SEL0_R {
        DFT_SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dft_sel1(&self) -> DFT_SEL1_R {
        DFT_SEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dft_sel0(&mut self) -> DFT_SEL0_W {
        DFT_SEL0_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dft_sel1(&mut self) -> DFT_SEL1_W {
        DFT_SEL1_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_ddft_ctrl](index.html) module"]
pub struct TST_DDFT_CTRL_SPEC;
impl crate::RegisterSpec for TST_DDFT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst_ddft_ctrl::R](R) reader structure"]
impl crate::Readable for TST_DDFT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_ddft_ctrl::W](W) writer structure"]
impl crate::Writable for TST_DDFT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TST_DDFT_CTRL to value 0x0f0f"]
impl crate::Resettable for TST_DDFT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f0f
    }
}
