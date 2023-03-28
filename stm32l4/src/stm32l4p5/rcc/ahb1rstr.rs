#[doc = "Register `AHB1RSTR` reader"]
pub struct R(crate::R<AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1RSTR` writer"]
pub struct W(crate::W<AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RSTR_SPEC>;
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
impl From<crate::W<AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type DMA1RST_R = crate::BitReader<DMA1RST_A>;
#[doc = "DMA1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMA1"]
    Reset = 1,
}
impl From<DMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1RST_A {
        match self.bits {
            false => DMA1RST_A::NoEffect,
            true => DMA1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST_A::Reset
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA1RST_A, O>;
impl<'a, const O: u8> DMA1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA1RST_A::NoEffect)
    }
    #[doc = "Reset DMA1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::Reset)
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub type DMA2RST_R = crate::BitReader<DMA2RST_A>;
#[doc = "DMA2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMA2"]
    Reset = 1,
}
impl From<DMA2RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2RST_A {
        match self.bits {
            false => DMA2RST_A::NoEffect,
            true => DMA2RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA2RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA2RST_A::Reset
    }
}
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub type DMA2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA2RST_A, O>;
impl<'a, const O: u8> DMA2RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA2RST_A::NoEffect)
    }
    #[doc = "Reset DMA2"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA2RST_A::Reset)
    }
}
#[doc = "Field `DMAMUX1RST` reader - DMAMUXRST"]
pub type DMAMUX1RST_R = crate::BitReader<DMAMUX1RST_A>;
#[doc = "DMAMUXRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMAMUX1"]
    Reset = 1,
}
impl From<DMAMUX1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX1RST_A {
        match self.bits {
            false => DMAMUX1RST_A::NoEffect,
            true => DMAMUX1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMAMUX1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMAMUX1RST_A::Reset
    }
}
#[doc = "Field `DMAMUX1RST` writer - DMAMUXRST"]
pub type DMAMUX1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMAMUX1RST_A, O>;
impl<'a, const O: u8> DMAMUX1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMAMUX1RST_A::NoEffect)
    }
    #[doc = "Reset DMAMUX1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMAMUX1RST_A::Reset)
    }
}
#[doc = "Field `FLASHRST` reader - Flash memory interface reset"]
pub type FLASHRST_R = crate::BitReader<FLASHRST_A>;
#[doc = "Flash memory interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset Flash memory interface"]
    Reset = 1,
}
impl From<FLASHRST_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHRST_A {
        match self.bits {
            false => FLASHRST_A::NoEffect,
            true => FLASHRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FLASHRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FLASHRST_A::Reset
    }
}
#[doc = "Field `FLASHRST` writer - Flash memory interface reset"]
pub type FLASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, FLASHRST_A, O>;
impl<'a, const O: u8> FLASHRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FLASHRST_A::NoEffect)
    }
    #[doc = "Reset Flash memory interface"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FLASHRST_A::Reset)
    }
}
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CRCRST_R = crate::BitReader<CRCRST_A>;
#[doc = "CRC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset CRC"]
    Reset = 1,
}
impl From<CRCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCRST_A {
        match self.bits {
            false => CRCRST_A::NoEffect,
            true => CRCRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CRCRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRCRST_A::Reset
    }
}
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, CRCRST_A, O>;
impl<'a, const O: u8> CRCRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CRCRST_A::NoEffect)
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRCRST_A::Reset)
    }
}
#[doc = "Field `TSCRST` reader - Touch Sensing Controller reset"]
pub type TSCRST_R = crate::BitReader<TSCRST_A>;
#[doc = "Touch Sensing Controller reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset TSC"]
    Reset = 1,
}
impl From<TSCRST_A> for bool {
    #[inline(always)]
    fn from(variant: TSCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl TSCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCRST_A {
        match self.bits {
            false => TSCRST_A::NoEffect,
            true => TSCRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TSCRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TSCRST_A::Reset
    }
}
#[doc = "Field `TSCRST` writer - Touch Sensing Controller reset"]
pub type TSCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, TSCRST_A, O>;
impl<'a, const O: u8> TSCRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TSCRST_A::NoEffect)
    }
    #[doc = "Reset TSC"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TSCRST_A::Reset)
    }
}
#[doc = "Field `DMA2DRST` reader - DMA2D reset"]
pub type DMA2DRST_R = crate::BitReader<DMA2DRST_A>;
#[doc = "DMA2D reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMA2D"]
    Reset = 1,
}
impl From<DMA2DRST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2DRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2DRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2DRST_A {
        match self.bits {
            false => DMA2DRST_A::NoEffect,
            true => DMA2DRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA2DRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA2DRST_A::Reset
    }
}
#[doc = "Field `DMA2DRST` writer - DMA2D reset"]
pub type DMA2DRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA2DRST_A, O>;
impl<'a, const O: u8> DMA2DRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA2DRST_A::NoEffect)
    }
    #[doc = "Reset DMA2D"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA2DRST_A::Reset)
    }
}
#[doc = "Field `GFXMMURST` reader - GFXMMU reset"]
pub type GFXMMURST_R = crate::BitReader<GFXMMURST_A>;
#[doc = "GFXMMU reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMURST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset GFXMMU"]
    Reset = 1,
}
impl From<GFXMMURST_A> for bool {
    #[inline(always)]
    fn from(variant: GFXMMURST_A) -> Self {
        variant as u8 != 0
    }
}
impl GFXMMURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GFXMMURST_A {
        match self.bits {
            false => GFXMMURST_A::NoEffect,
            true => GFXMMURST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GFXMMURST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GFXMMURST_A::Reset
    }
}
#[doc = "Field `GFXMMURST` writer - GFXMMU reset"]
pub type GFXMMURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, GFXMMURST_A, O>;
impl<'a, const O: u8> GFXMMURST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(GFXMMURST_A::NoEffect)
    }
    #[doc = "Reset GFXMMU"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GFXMMURST_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamux1rst(&self) -> DMAMUX1RST_R {
        DMAMUX1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GFXMMU reset"]
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1rst(&mut self) -> DMAMUX1RST_W<2> {
        DMAMUX1RST_W::new(self)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<8> {
        FLASHRST_W::new(self)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<16> {
        TSCRST_W::new(self)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<17> {
        DMA2DRST_W::new(self)
    }
    #[doc = "Bit 18 - GFXMMU reset"]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W<18> {
        GFXMMURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](index.html) module"]
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1rstr::R](R) reader structure"]
impl crate::Readable for AHB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](W) writer structure"]
impl crate::Writable for AHB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
