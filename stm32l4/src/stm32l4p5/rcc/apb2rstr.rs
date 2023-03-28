#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGRST` reader - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST_A>;
#[doc = "System configuration (SYSCFG) reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SYSCFG + COMP + VREFBUF"]
    Reset = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGRST_A {
        match self.bits {
            false => SYSCFGRST_A::NoEffect,
            true => SYSCFGRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SYSCFGRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST_A::Reset
    }
}
#[doc = "Field `SYSCFGRST` writer - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, SYSCFGRST_A, O>;
impl<'a, const O: u8> SYSCFGRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::NoEffect)
    }
    #[doc = "Reset SYSCFG + COMP + VREFBUF"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::Reset)
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader<TIM1RST_A>;
#[doc = "TIM1 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset TIMx"]
    Reset = 1,
}
impl From<TIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1RST_A {
        match self.bits {
            false => TIM1RST_A::NoEffect,
            true => TIM1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIM1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST_A::Reset
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, TIM1RST_A, O>;
impl<'a, const O: u8> TIM1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIM1RST_A::NoEffect)
    }
    #[doc = "Reset TIMx"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RST_A::Reset)
    }
}
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader<SPI1RST_A>;
#[doc = "SPI1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SPI1"]
    Reset = 1,
}
impl From<SPI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1RST_A {
        match self.bits {
            false => SPI1RST_A::NoEffect,
            true => SPI1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SPI1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SPI1RST_A::Reset
    }
}
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, SPI1RST_A, O>;
impl<'a, const O: u8> SPI1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SPI1RST_A::NoEffect)
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI1RST_A::Reset)
    }
}
#[doc = "Field `TIM8RST` reader - TIM8 timer reset"]
pub use TIM1RST_R as TIM8RST_R;
#[doc = "Field `TIM8RST` writer - TIM8 timer reset"]
pub use TIM1RST_W as TIM8RST_W;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader<USART1RST_A>;
#[doc = "USART1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset UARTx"]
    Reset = 1,
}
impl From<USART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1RST_A {
        match self.bits {
            false => USART1RST_A::NoEffect,
            true => USART1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == USART1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART1RST_A::Reset
    }
}
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, USART1RST_A, O>;
impl<'a, const O: u8> USART1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(USART1RST_A::NoEffect)
    }
    #[doc = "Reset UARTx"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART1RST_A::Reset)
    }
}
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub use TIM1RST_R as TIM15RST_R;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub use TIM1RST_R as TIM16RST_R;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub use TIM1RST_R as TIM17RST_R;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub use TIM1RST_W as TIM15RST_W;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub use TIM1RST_W as TIM16RST_W;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub use TIM1RST_W as TIM17RST_W;
#[doc = "Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_R = crate::BitReader<SAI1RST_A>;
#[doc = "Serial audio interface 1 (SAI1) reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SAIx"]
    Reset = 1,
}
impl From<SAI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1RST_A {
        match self.bits {
            false => SAI1RST_A::NoEffect,
            true => SAI1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SAI1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SAI1RST_A::Reset
    }
}
#[doc = "Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, SAI1RST_A, O>;
impl<'a, const O: u8> SAI1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SAI1RST_A::NoEffect)
    }
    #[doc = "Reset SAIx"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SAI1RST_A::Reset)
    }
}
#[doc = "Field `SAI2RST` reader - Serial audio interface 2 (SAI2) reset"]
pub use SAI1RST_R as SAI2RST_R;
#[doc = "Field `SAI2RST` writer - Serial audio interface 2 (SAI2) reset"]
pub use SAI1RST_W as SAI2RST_W;
#[doc = "Field `DFSDM1RST` reader - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DFSDM1RST_R = crate::BitReader<DFSDM1RST_A>;
#[doc = "Digital filters for sigma-delata modulators (DFSDM) reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DFSDM1"]
    Reset = 1,
}
impl From<DFSDM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1RST_A {
        match self.bits {
            false => DFSDM1RST_A::NoEffect,
            true => DFSDM1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DFSDM1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DFSDM1RST_A::Reset
    }
}
#[doc = "Field `DFSDM1RST` writer - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DFSDM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, DFSDM1RST_A, O>;
impl<'a, const O: u8> DFSDM1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DFSDM1RST_A::NoEffect)
    }
    #[doc = "Reset DFSDM1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DFSDM1RST_A::Reset)
    }
}
#[doc = "Field `LTDCRST` reader - LCD-TFT reset"]
pub type LTDCRST_R = crate::BitReader<LTDCRST_A>;
#[doc = "LCD-TFT reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset LCD-TFT"]
    Reset = 1,
}
impl From<LTDCRST_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl LTDCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCRST_A {
        match self.bits {
            false => LTDCRST_A::NoEffect,
            true => LTDCRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LTDCRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LTDCRST_A::Reset
    }
}
#[doc = "Field `LTDCRST` writer - LCD-TFT reset"]
pub type LTDCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, LTDCRST_A, O>;
impl<'a, const O: u8> LTDCRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(LTDCRST_A::NoEffect)
    }
    #[doc = "Reset LCD-TFT"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LTDCRST_A::Reset)
    }
}
#[doc = "Field `DSIRST` reader - DSI reset"]
pub type DSIRST_R = crate::BitReader<DSIRST_A>;
#[doc = "DSI reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DSI"]
    Reset = 1,
}
impl From<DSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DSIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DSIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIRST_A {
        match self.bits {
            false => DSIRST_A::NoEffect,
            true => DSIRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DSIRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DSIRST_A::Reset
    }
}
#[doc = "Field `DSIRST` writer - DSI reset"]
pub type DSIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, DSIRST_A, O>;
impl<'a, const O: u8> DSIRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DSIRST_A::NoEffect)
    }
    #[doc = "Reset DSI"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DSIRST_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> DFSDM1RST_R {
        DFSDM1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT reset"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI reset"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<13> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<21> {
        SAI1RST_W::new(self)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<22> {
        SAI2RST_W::new(self)
    }
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1rst(&mut self) -> DFSDM1RST_W<24> {
        DFSDM1RST_W::new(self)
    }
    #[doc = "Bit 26 - LCD-TFT reset"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<26> {
        LTDCRST_W::new(self)
    }
    #[doc = "Bit 27 - DSI reset"]
    #[inline(always)]
    #[must_use]
    pub fn dsirst(&mut self) -> DSIRST_W<27> {
        DSIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
