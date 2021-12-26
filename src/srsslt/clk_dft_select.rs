#[doc = "Register `CLK_DFT_SELECT` reader"]
pub struct R(crate::R<CLK_DFT_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_DFT_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_DFT_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_DFT_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_DFT_SELECT` writer"]
pub struct W(crate::W<CLK_DFT_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_DFT_SELECT_SPEC>;
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
impl From<crate::W<CLK_DFT_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_DFT_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFT_SEL0_A {
    #[doc = "0: `0`"]
    NC = 0,
    #[doc = "1: `1`"]
    ILO = 1,
    #[doc = "2: `10`"]
    IMO = 2,
    #[doc = "3: `11`"]
    ECO = 3,
    #[doc = "4: `100`"]
    EXTCLK = 4,
    #[doc = "5: `101`"]
    HFCLK = 5,
    #[doc = "6: `110`"]
    LFCLK = 6,
    #[doc = "7: `111`"]
    SYSCLK = 7,
    #[doc = "8: `1000`"]
    PUMPCLK = 8,
    #[doc = "9: `1001`"]
    SLPCTRLCLK = 9,
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
    pub fn variant(&self) -> Option<DFT_SEL0_A> {
        match self.bits {
            0 => Some(DFT_SEL0_A::NC),
            1 => Some(DFT_SEL0_A::ILO),
            2 => Some(DFT_SEL0_A::IMO),
            3 => Some(DFT_SEL0_A::ECO),
            4 => Some(DFT_SEL0_A::EXTCLK),
            5 => Some(DFT_SEL0_A::HFCLK),
            6 => Some(DFT_SEL0_A::LFCLK),
            7 => Some(DFT_SEL0_A::SYSCLK),
            8 => Some(DFT_SEL0_A::PUMPCLK),
            9 => Some(DFT_SEL0_A::SLPCTRLCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        **self == DFT_SEL0_A::NC
    }
    #[doc = "Checks if the value of the field is `ILO`"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        **self == DFT_SEL0_A::ILO
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        **self == DFT_SEL0_A::IMO
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        **self == DFT_SEL0_A::ECO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        **self == DFT_SEL0_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        **self == DFT_SEL0_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `LFCLK`"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        **self == DFT_SEL0_A::LFCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == DFT_SEL0_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PUMPCLK`"]
    #[inline(always)]
    pub fn is_pumpclk(&self) -> bool {
        **self == DFT_SEL0_A::PUMPCLK
    }
    #[doc = "Checks if the value of the field is `SLPCTRLCLK`"]
    #[inline(always)]
    pub fn is_slpctrlclk(&self) -> bool {
        **self == DFT_SEL0_A::SLPCTRLCLK
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::NC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::ILO)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::IMO)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::ECO)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::EXTCLK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::HFCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::LFCLK)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SYSCLK)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pumpclk(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::PUMPCLK)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn slpctrlclk(self) -> &'a mut W {
        self.variant(DFT_SEL0_A::SLPCTRLCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFT_DIV0_A {
    #[doc = "0: `0`"]
    NO_DIV = 0,
    #[doc = "1: `1`"]
    DIV_BY_2 = 1,
    #[doc = "2: `10`"]
    DIV_BY_4 = 2,
    #[doc = "3: `11`"]
    DIV_BY_8 = 3,
}
impl From<DFT_DIV0_A> for u8 {
    #[inline(always)]
    fn from(variant: DFT_DIV0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DFT_DIV0` reader - "]
pub struct DFT_DIV0_R(crate::FieldReader<u8, DFT_DIV0_A>);
impl DFT_DIV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFT_DIV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFT_DIV0_A {
        match self.bits {
            0 => DFT_DIV0_A::NO_DIV,
            1 => DFT_DIV0_A::DIV_BY_2,
            2 => DFT_DIV0_A::DIV_BY_4,
            3 => DFT_DIV0_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        **self == DFT_DIV0_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        **self == DFT_DIV0_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        **self == DFT_DIV0_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        **self == DFT_DIV0_A::DIV_BY_8
    }
}
impl core::ops::Deref for DFT_DIV0_R {
    type Target = crate::FieldReader<u8, DFT_DIV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_DIV0` writer - "]
pub struct DFT_DIV0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_DIV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFT_DIV0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(DFT_DIV0_A::NO_DIV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(DFT_DIV0_A::DIV_BY_2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(DFT_DIV0_A::DIV_BY_4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(DFT_DIV0_A::DIV_BY_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFT_EDGE0_A {
    #[doc = "0: `0`"]
    POSEDGE = 0,
    #[doc = "1: `1`"]
    NEGEDGE = 1,
}
impl From<DFT_EDGE0_A> for bool {
    #[inline(always)]
    fn from(variant: DFT_EDGE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFT_EDGE0` reader - "]
pub struct DFT_EDGE0_R(crate::FieldReader<bool, DFT_EDGE0_A>);
impl DFT_EDGE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_EDGE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFT_EDGE0_A {
        match self.bits {
            false => DFT_EDGE0_A::POSEDGE,
            true => DFT_EDGE0_A::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        **self == DFT_EDGE0_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        **self == DFT_EDGE0_A::NEGEDGE
    }
}
impl core::ops::Deref for DFT_EDGE0_R {
    type Target = crate::FieldReader<bool, DFT_EDGE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_EDGE0` writer - "]
pub struct DFT_EDGE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_EDGE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFT_EDGE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(DFT_EDGE0_A::POSEDGE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(DFT_EDGE0_A::NEGEDGE)
    }
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFT_SEL1_A {
    #[doc = "0: `0`"]
    NC = 0,
    #[doc = "1: `1`"]
    ILO = 1,
    #[doc = "2: `10`"]
    IMO = 2,
    #[doc = "3: `11`"]
    ECO = 3,
    #[doc = "4: `100`"]
    EXTCLK = 4,
    #[doc = "5: `101`"]
    HFCLK = 5,
    #[doc = "6: `110`"]
    LFCLK = 6,
    #[doc = "7: `111`"]
    SYSCLK = 7,
    #[doc = "8: `1000`"]
    PUMPCLK = 8,
    #[doc = "9: `1001`"]
    SLPCTRLCLK = 9,
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
    pub fn variant(&self) -> Option<DFT_SEL1_A> {
        match self.bits {
            0 => Some(DFT_SEL1_A::NC),
            1 => Some(DFT_SEL1_A::ILO),
            2 => Some(DFT_SEL1_A::IMO),
            3 => Some(DFT_SEL1_A::ECO),
            4 => Some(DFT_SEL1_A::EXTCLK),
            5 => Some(DFT_SEL1_A::HFCLK),
            6 => Some(DFT_SEL1_A::LFCLK),
            7 => Some(DFT_SEL1_A::SYSCLK),
            8 => Some(DFT_SEL1_A::PUMPCLK),
            9 => Some(DFT_SEL1_A::SLPCTRLCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        **self == DFT_SEL1_A::NC
    }
    #[doc = "Checks if the value of the field is `ILO`"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        **self == DFT_SEL1_A::ILO
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        **self == DFT_SEL1_A::IMO
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        **self == DFT_SEL1_A::ECO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        **self == DFT_SEL1_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        **self == DFT_SEL1_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `LFCLK`"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        **self == DFT_SEL1_A::LFCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == DFT_SEL1_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PUMPCLK`"]
    #[inline(always)]
    pub fn is_pumpclk(&self) -> bool {
        **self == DFT_SEL1_A::PUMPCLK
    }
    #[doc = "Checks if the value of the field is `SLPCTRLCLK`"]
    #[inline(always)]
    pub fn is_slpctrlclk(&self) -> bool {
        **self == DFT_SEL1_A::SLPCTRLCLK
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::NC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::ILO)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::IMO)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::ECO)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::EXTCLK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::HFCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::LFCLK)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SYSCLK)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pumpclk(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::PUMPCLK)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn slpctrlclk(self) -> &'a mut W {
        self.variant(DFT_SEL1_A::SLPCTRLCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFT_DIV1_A {
    #[doc = "0: `0`"]
    NO_DIV = 0,
    #[doc = "1: `1`"]
    DIV_BY_2 = 1,
    #[doc = "2: `10`"]
    DIV_BY_4 = 2,
    #[doc = "3: `11`"]
    DIV_BY_8 = 3,
}
impl From<DFT_DIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: DFT_DIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DFT_DIV1` reader - "]
pub struct DFT_DIV1_R(crate::FieldReader<u8, DFT_DIV1_A>);
impl DFT_DIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFT_DIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFT_DIV1_A {
        match self.bits {
            0 => DFT_DIV1_A::NO_DIV,
            1 => DFT_DIV1_A::DIV_BY_2,
            2 => DFT_DIV1_A::DIV_BY_4,
            3 => DFT_DIV1_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        **self == DFT_DIV1_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        **self == DFT_DIV1_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        **self == DFT_DIV1_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        **self == DFT_DIV1_A::DIV_BY_8
    }
}
impl core::ops::Deref for DFT_DIV1_R {
    type Target = crate::FieldReader<u8, DFT_DIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_DIV1` writer - "]
pub struct DFT_DIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_DIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFT_DIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(DFT_DIV1_A::NO_DIV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(DFT_DIV1_A::DIV_BY_2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(DFT_DIV1_A::DIV_BY_4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(DFT_DIV1_A::DIV_BY_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFT_EDGE1_A {
    #[doc = "0: `0`"]
    POSEDGE = 0,
    #[doc = "1: `1`"]
    NEGEDGE = 1,
}
impl From<DFT_EDGE1_A> for bool {
    #[inline(always)]
    fn from(variant: DFT_EDGE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFT_EDGE1` reader - "]
pub struct DFT_EDGE1_R(crate::FieldReader<bool, DFT_EDGE1_A>);
impl DFT_EDGE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFT_EDGE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFT_EDGE1_A {
        match self.bits {
            false => DFT_EDGE1_A::POSEDGE,
            true => DFT_EDGE1_A::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        **self == DFT_EDGE1_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        **self == DFT_EDGE1_A::NEGEDGE
    }
}
impl core::ops::Deref for DFT_EDGE1_R {
    type Target = crate::FieldReader<bool, DFT_EDGE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFT_EDGE1` writer - "]
pub struct DFT_EDGE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFT_EDGE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFT_EDGE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(DFT_EDGE1_A::POSEDGE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(DFT_EDGE1_A::NEGEDGE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dft_sel0(&self) -> DFT_SEL0_R {
        DFT_SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dft_div0(&self) -> DFT_DIV0_R {
        DFT_DIV0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dft_edge0(&self) -> DFT_EDGE0_R {
        DFT_EDGE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dft_sel1(&self) -> DFT_SEL1_R {
        DFT_SEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dft_div1(&self) -> DFT_DIV1_R {
        DFT_DIV1_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dft_edge1(&self) -> DFT_EDGE1_R {
        DFT_EDGE1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dft_sel0(&mut self) -> DFT_SEL0_W {
        DFT_SEL0_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dft_div0(&mut self) -> DFT_DIV0_W {
        DFT_DIV0_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dft_edge0(&mut self) -> DFT_EDGE0_W {
        DFT_EDGE0_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dft_sel1(&mut self) -> DFT_SEL1_W {
        DFT_SEL1_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dft_div1(&mut self) -> DFT_DIV1_W {
        DFT_DIV1_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dft_edge1(&mut self) -> DFT_EDGE1_W {
        DFT_EDGE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_dft_select](index.html) module"]
pub struct CLK_DFT_SELECT_SPEC;
impl crate::RegisterSpec for CLK_DFT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_dft_select::R](R) reader structure"]
impl crate::Readable for CLK_DFT_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_dft_select::W](W) writer structure"]
impl crate::Writable for CLK_DFT_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_DFT_SELECT to value 0"]
impl crate::Resettable for CLK_DFT_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
