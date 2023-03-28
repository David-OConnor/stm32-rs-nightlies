#[doc = "Register `CR4` reader"]
pub struct R(crate::R<CR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR4` writer"]
pub struct W(crate::W<CR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR4_SPEC>;
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
impl From<crate::W<CR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP1` reader - Wakeup pin WKUP1 polarity"]
pub type WP1_R = crate::BitReader<WP1_A>;
#[doc = "Wakeup pin WKUP1 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP1_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WP1_A> for bool {
    #[inline(always)]
    fn from(variant: WP1_A) -> Self {
        variant as u8 != 0
    }
}
impl WP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP1_A {
        match self.bits {
            false => WP1_A::RisingEdge,
            true => WP1_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP1_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP1_A::FallingEdge
    }
}
#[doc = "Field `WP1` writer - Wakeup pin WKUP1 polarity"]
pub type WP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, WP1_A, O>;
impl<'a, const O: u8> WP1_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WP1_A::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WP1_A::FallingEdge)
    }
}
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity"]
pub type WP2_R = crate::BitReader<WP2_A>;
#[doc = "Wakeup pin WKUP2 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP2_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WP2_A> for bool {
    #[inline(always)]
    fn from(variant: WP2_A) -> Self {
        variant as u8 != 0
    }
}
impl WP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP2_A {
        match self.bits {
            false => WP2_A::RisingEdge,
            true => WP2_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP2_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP2_A::FallingEdge
    }
}
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity"]
pub type WP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, WP2_A, O>;
impl<'a, const O: u8> WP2_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WP2_A::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WP2_A::FallingEdge)
    }
}
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity"]
pub type WP3_R = crate::BitReader<WP3_A>;
#[doc = "Wakeup pin WKUP3 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP3_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WP3_A> for bool {
    #[inline(always)]
    fn from(variant: WP3_A) -> Self {
        variant as u8 != 0
    }
}
impl WP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP3_A {
        match self.bits {
            false => WP3_A::RisingEdge,
            true => WP3_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP3_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP3_A::FallingEdge
    }
}
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity"]
pub type WP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, WP3_A, O>;
impl<'a, const O: u8> WP3_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WP3_A::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WP3_A::FallingEdge)
    }
}
#[doc = "Field `VBE` reader - VBAT battery charging enable"]
pub type VBE_R = crate::BitReader<VBE_A>;
#[doc = "VBAT battery charging enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE_A {
    #[doc = "0: VBAT battery charging disabled"]
    Disabled = 0,
    #[doc = "1: VBAT battery charging enabled"]
    Enabled = 1,
}
impl From<VBE_A> for bool {
    #[inline(always)]
    fn from(variant: VBE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBE_A {
        match self.bits {
            false => VBE_A::Disabled,
            true => VBE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBE_A::Enabled
    }
}
#[doc = "Field `VBE` writer - VBAT battery charging enable"]
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, VBE_A, O>;
impl<'a, const O: u8> VBE_W<'a, O> {
    #[doc = "VBAT battery charging disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBE_A::Disabled)
    }
    #[doc = "VBAT battery charging enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBE_A::Enabled)
    }
}
#[doc = "Field `VBRS` reader - VBAT battery charging resistor selection"]
pub type VBRS_R = crate::BitReader<VBRS_A>;
#[doc = "VBAT battery charging resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS_A {
    #[doc = "0: VBAT charging through a 5 kΩ resistor"]
    R5k = 0,
    #[doc = "1: VBAT charging through a 1.5 kΩ resistor"]
    R15k = 1,
}
impl From<VBRS_A> for bool {
    #[inline(always)]
    fn from(variant: VBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl VBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBRS_A {
        match self.bits {
            false => VBRS_A::R5k,
            true => VBRS_A::R15k,
        }
    }
    #[doc = "Checks if the value of the field is `R5k`"]
    #[inline(always)]
    pub fn is_r5k(&self) -> bool {
        *self == VBRS_A::R5k
    }
    #[doc = "Checks if the value of the field is `R15k`"]
    #[inline(always)]
    pub fn is_r1_5k(&self) -> bool {
        *self == VBRS_A::R15k
    }
}
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, VBRS_A, O>;
impl<'a, const O: u8> VBRS_W<'a, O> {
    #[doc = "VBAT charging through a 5 kΩ resistor"]
    #[inline(always)]
    pub fn r5k(self) -> &'a mut W {
        self.variant(VBRS_A::R5k)
    }
    #[doc = "VBAT charging through a 1.5 kΩ resistor"]
    #[inline(always)]
    pub fn r1_5k(self) -> &'a mut W {
        self.variant(VBRS_A::R15k)
    }
}
#[doc = "Field `WRFBUSYP` reader - Wakeup Radio BUSY polarity"]
pub type WRFBUSYP_R = crate::BitReader<WRFBUSYP_A>;
#[doc = "Wakeup Radio BUSY polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRFBUSYP_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WRFBUSYP_A> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYP_A) -> Self {
        variant as u8 != 0
    }
}
impl WRFBUSYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRFBUSYP_A {
        match self.bits {
            false => WRFBUSYP_A::RisingEdge,
            true => WRFBUSYP_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WRFBUSYP_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WRFBUSYP_A::FallingEdge
    }
}
#[doc = "Field `WRFBUSYP` writer - Wakeup Radio BUSY polarity"]
pub type WRFBUSYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, WRFBUSYP_A, O>;
impl<'a, const O: u8> WRFBUSYP_W<'a, O> {
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WRFBUSYP_A::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WRFBUSYP_A::FallingEdge)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup Radio BUSY polarity"]
    #[inline(always)]
    pub fn wrfbusyp(&self) -> WRFBUSYP_R {
        WRFBUSYP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<0> {
        WP1_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<1> {
        WP2_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<2> {
        WP3_W::new(self)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup Radio BUSY polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wrfbusyp(&mut self) -> WRFBUSYP_W<11> {
        WRFBUSYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr4](index.html) module"]
pub struct CR4_SPEC;
impl crate::RegisterSpec for CR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr4::R](R) reader structure"]
impl crate::Readable for CR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr4::W](W) writer structure"]
impl crate::Writable for CR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
