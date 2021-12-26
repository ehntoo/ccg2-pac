#[doc = "Register `PORT_SEL2` reader"]
pub struct R(crate::R<PORT_SEL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_SEL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_SEL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_SEL2` writer"]
pub struct W(crate::W<PORT_SEL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SEL2_SPEC>;
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
impl From<crate::W<PORT_SEL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_SEL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO0_SEL_A {
    #[doc = "0: `0`"]
    GPIO = 0,
    #[doc = "1: `1`"]
    GPIO_DSI = 1,
    #[doc = "2: `10`"]
    DSI_DSI = 2,
    #[doc = "3: `11`"]
    DSI_GPIO = 3,
    #[doc = "4: `100`"]
    CSD_SENSE = 4,
    #[doc = "5: `101`"]
    CSD_SHIELD = 5,
    #[doc = "6: `110`"]
    AMUXA = 6,
    #[doc = "7: `111`"]
    AMUXB = 7,
    #[doc = "8: `1000`"]
    ACT_0 = 8,
    #[doc = "9: `1001`"]
    ACT_1 = 9,
    #[doc = "10: `1010`"]
    ACT_2 = 10,
    #[doc = "11: `1011`"]
    ACT_3 = 11,
    #[doc = "12: `1100`"]
    DS_0 = 12,
    #[doc = "13: `1101`"]
    DS_1 = 13,
    #[doc = "14: `1110`"]
    DS_2 = 14,
    #[doc = "15: `1111`"]
    DS_3 = 15,
}
impl From<IO0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO0_SEL` reader - "]
pub struct IO0_SEL_R(crate::FieldReader<u8, IO0_SEL_A>);
impl IO0_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO0_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO0_SEL_A {
        match self.bits {
            0 => IO0_SEL_A::GPIO,
            1 => IO0_SEL_A::GPIO_DSI,
            2 => IO0_SEL_A::DSI_DSI,
            3 => IO0_SEL_A::DSI_GPIO,
            4 => IO0_SEL_A::CSD_SENSE,
            5 => IO0_SEL_A::CSD_SHIELD,
            6 => IO0_SEL_A::AMUXA,
            7 => IO0_SEL_A::AMUXB,
            8 => IO0_SEL_A::ACT_0,
            9 => IO0_SEL_A::ACT_1,
            10 => IO0_SEL_A::ACT_2,
            11 => IO0_SEL_A::ACT_3,
            12 => IO0_SEL_A::DS_0,
            13 => IO0_SEL_A::DS_1,
            14 => IO0_SEL_A::DS_2,
            15 => IO0_SEL_A::DS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == IO0_SEL_A::GPIO
    }
    #[doc = "Checks if the value of the field is `GPIO_DSI`"]
    #[inline(always)]
    pub fn is_gpio_dsi(&self) -> bool {
        **self == IO0_SEL_A::GPIO_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_DSI`"]
    #[inline(always)]
    pub fn is_dsi_dsi(&self) -> bool {
        **self == IO0_SEL_A::DSI_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_GPIO`"]
    #[inline(always)]
    pub fn is_dsi_gpio(&self) -> bool {
        **self == IO0_SEL_A::DSI_GPIO
    }
    #[doc = "Checks if the value of the field is `CSD_SENSE`"]
    #[inline(always)]
    pub fn is_csd_sense(&self) -> bool {
        **self == IO0_SEL_A::CSD_SENSE
    }
    #[doc = "Checks if the value of the field is `CSD_SHIELD`"]
    #[inline(always)]
    pub fn is_csd_shield(&self) -> bool {
        **self == IO0_SEL_A::CSD_SHIELD
    }
    #[doc = "Checks if the value of the field is `AMUXA`"]
    #[inline(always)]
    pub fn is_amuxa(&self) -> bool {
        **self == IO0_SEL_A::AMUXA
    }
    #[doc = "Checks if the value of the field is `AMUXB`"]
    #[inline(always)]
    pub fn is_amuxb(&self) -> bool {
        **self == IO0_SEL_A::AMUXB
    }
    #[doc = "Checks if the value of the field is `ACT_0`"]
    #[inline(always)]
    pub fn is_act_0(&self) -> bool {
        **self == IO0_SEL_A::ACT_0
    }
    #[doc = "Checks if the value of the field is `ACT_1`"]
    #[inline(always)]
    pub fn is_act_1(&self) -> bool {
        **self == IO0_SEL_A::ACT_1
    }
    #[doc = "Checks if the value of the field is `ACT_2`"]
    #[inline(always)]
    pub fn is_act_2(&self) -> bool {
        **self == IO0_SEL_A::ACT_2
    }
    #[doc = "Checks if the value of the field is `ACT_3`"]
    #[inline(always)]
    pub fn is_act_3(&self) -> bool {
        **self == IO0_SEL_A::ACT_3
    }
    #[doc = "Checks if the value of the field is `DS_0`"]
    #[inline(always)]
    pub fn is_ds_0(&self) -> bool {
        **self == IO0_SEL_A::DS_0
    }
    #[doc = "Checks if the value of the field is `DS_1`"]
    #[inline(always)]
    pub fn is_ds_1(&self) -> bool {
        **self == IO0_SEL_A::DS_1
    }
    #[doc = "Checks if the value of the field is `DS_2`"]
    #[inline(always)]
    pub fn is_ds_2(&self) -> bool {
        **self == IO0_SEL_A::DS_2
    }
    #[doc = "Checks if the value of the field is `DS_3`"]
    #[inline(always)]
    pub fn is_ds_3(&self) -> bool {
        **self == IO0_SEL_A::DS_3
    }
}
impl core::ops::Deref for IO0_SEL_R {
    type Target = crate::FieldReader<u8, IO0_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO0_SEL` writer - "]
pub struct IO0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO0_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn gpio_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO_DSI)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dsi_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_DSI)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn dsi_gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_GPIO)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn csd_sense(self) -> &'a mut W {
        self.variant(IO0_SEL_A::CSD_SENSE)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn csd_shield(self) -> &'a mut W {
        self.variant(IO0_SEL_A::CSD_SHIELD)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn amuxa(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn amuxb(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXB)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn act_0(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn act_1(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn act_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn act_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_3)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn ds_0(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_0)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn ds_1(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn ds_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_2)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn ds_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `IO1_SEL` reader - "]
pub struct IO1_SEL_R(crate::FieldReader<u8, u8>);
impl IO1_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO1_SEL` writer - "]
pub struct IO1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `IO2_SEL` reader - "]
pub struct IO2_SEL_R(crate::FieldReader<u8, u8>);
impl IO2_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO2_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO2_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO2_SEL` writer - "]
pub struct IO2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `IO3_SEL` reader - "]
pub struct IO3_SEL_R(crate::FieldReader<u8, u8>);
impl IO3_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO3_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO3_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO3_SEL` writer - "]
pub struct IO3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn io0_sel(&self) -> IO0_SEL_R {
        IO0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn io1_sel(&self) -> IO1_SEL_R {
        IO1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn io2_sel(&self) -> IO2_SEL_R {
        IO2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn io3_sel(&self) -> IO3_SEL_R {
        IO3_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn io0_sel(&mut self) -> IO0_SEL_W {
        IO0_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn io1_sel(&mut self) -> IO1_SEL_W {
        IO1_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn io2_sel(&mut self) -> IO2_SEL_W {
        IO2_SEL_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn io3_sel(&mut self) -> IO3_SEL_W {
        IO3_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel2](index.html) module"]
pub struct PORT_SEL2_SPEC;
impl crate::RegisterSpec for PORT_SEL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_sel2::R](R) reader structure"]
impl crate::Readable for PORT_SEL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_sel2::W](W) writer structure"]
impl crate::Writable for PORT_SEL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_SEL2 to value 0"]
impl crate::Resettable for PORT_SEL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
