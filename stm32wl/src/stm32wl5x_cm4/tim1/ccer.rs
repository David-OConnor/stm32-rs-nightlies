#[doc = "Register `CCER` reader"]
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCER` writer"]
pub struct W(crate::W<CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCER_SPEC>;
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
impl From<crate::W<CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1E` reader - CC1E"]
pub type CC1E_R = crate::BitReader<CC1E_A>;
#[doc = "CC1E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E_A {
    #[doc = "0: Capture disabled"]
    Disabled = 0,
    #[doc = "1: Capture enabled"]
    Enabled = 1,
}
impl From<CC1E_A> for bool {
    #[inline(always)]
    fn from(variant: CC1E_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1E_A {
        match self.bits {
            false => CC1E_A::Disabled,
            true => CC1E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1E_A::Enabled
    }
}
#[doc = "Field `CC1E` writer - CC1E"]
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, CC1E_A, O>;
impl<'a, const O: u8> CC1E_W<'a, O> {
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1E_A::Disabled)
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1E_A::Enabled)
    }
}
#[doc = "Field `CC1P` reader - CC1P"]
pub type CC1P_R = crate::BitReader<CC1P_A>;
#[doc = "CC1P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P_A {
    #[doc = "0: Noninverted/rising edge"]
    RisingEdge = 0,
    #[doc = "1: Inverted/falling edge"]
    FallingEdge = 1,
}
impl From<CC1P_A> for bool {
    #[inline(always)]
    fn from(variant: CC1P_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1P_A {
        match self.bits {
            false => CC1P_A::RisingEdge,
            true => CC1P_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CC1P_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CC1P_A::FallingEdge
    }
}
#[doc = "Field `CC1P` writer - CC1P"]
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, CC1P_A, O>;
impl<'a, const O: u8> CC1P_W<'a, O> {
    #[doc = "Noninverted/rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC1P_A::RisingEdge)
    }
    #[doc = "Inverted/falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC1P_A::FallingEdge)
    }
}
#[doc = "Field `CC1NE` reader - CC1NE"]
pub type CC1NE_R = crate::BitReader<CC1NE_A>;
#[doc = "CC1NE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NE_A {
    #[doc = "0: Complementary output disabled"]
    Disabled = 0,
    #[doc = "1: Complementary output enabled"]
    Enabled = 1,
}
impl From<CC1NE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NE_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1NE_A {
        match self.bits {
            false => CC1NE_A::Disabled,
            true => CC1NE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1NE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1NE_A::Enabled
    }
}
#[doc = "Field `CC1NE` writer - CC1NE"]
pub type CC1NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, CC1NE_A, O>;
impl<'a, const O: u8> CC1NE_W<'a, O> {
    #[doc = "Complementary output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1NE_A::Disabled)
    }
    #[doc = "Complementary output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1NE_A::Enabled)
    }
}
#[doc = "Field `CC1NP` reader - CC1NP"]
pub type CC1NP_R = crate::BitReader<CC1NP_A>;
#[doc = "CC1NP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NP_A {
    #[doc = "0: OCxN active high"]
    ActiveHigh = 0,
    #[doc = "1: OCxN active low"]
    ActiveLow = 1,
}
impl From<CC1NP_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NP_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1NP_A {
        match self.bits {
            false => CC1NP_A::ActiveHigh,
            true => CC1NP_A::ActiveLow,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == CC1NP_A::ActiveHigh
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == CC1NP_A::ActiveLow
    }
}
#[doc = "Field `CC1NP` writer - CC1NP"]
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, CC1NP_A, O>;
impl<'a, const O: u8> CC1NP_W<'a, O> {
    #[doc = "OCxN active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CC1NP_A::ActiveHigh)
    }
    #[doc = "OCxN active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CC1NP_A::ActiveLow)
    }
}
#[doc = "Field `CC2E` reader - CC2E"]
pub use CC1E_R as CC2E_R;
#[doc = "Field `CC3E` reader - CC3E"]
pub use CC1E_R as CC3E_R;
#[doc = "Field `CC4E` reader - CC4E"]
pub use CC1E_R as CC4E_R;
#[doc = "Field `CC5E` reader - CC5E"]
pub use CC1E_R as CC5E_R;
#[doc = "Field `CC6E` reader - CC6E"]
pub use CC1E_R as CC6E_R;
#[doc = "Field `CC2E` writer - CC2E"]
pub use CC1E_W as CC2E_W;
#[doc = "Field `CC3E` writer - CC3E"]
pub use CC1E_W as CC3E_W;
#[doc = "Field `CC4E` writer - CC4E"]
pub use CC1E_W as CC4E_W;
#[doc = "Field `CC5E` writer - CC5E"]
pub use CC1E_W as CC5E_W;
#[doc = "Field `CC6E` writer - CC6E"]
pub use CC1E_W as CC6E_W;
#[doc = "Field `CC2NE` reader - CC2NE"]
pub use CC1NE_R as CC2NE_R;
#[doc = "Field `CC3NE` reader - CC3NE"]
pub use CC1NE_R as CC3NE_R;
#[doc = "Field `CC2NE` writer - CC2NE"]
pub use CC1NE_W as CC2NE_W;
#[doc = "Field `CC3NE` writer - CC3NE"]
pub use CC1NE_W as CC3NE_W;
#[doc = "Field `CC2NP` reader - CC2NP"]
pub use CC1NP_R as CC2NP_R;
#[doc = "Field `CC3NP` reader - CC3NP"]
pub use CC1NP_R as CC3NP_R;
#[doc = "Field `CC2NP` writer - CC2NP"]
pub use CC1NP_W as CC2NP_W;
#[doc = "Field `CC3NP` writer - CC3NP"]
pub use CC1NP_W as CC3NP_W;
#[doc = "Field `CC2P` reader - CC2P"]
pub use CC1P_R as CC2P_R;
#[doc = "Field `CC3P` reader - CC3P"]
pub use CC1P_R as CC3P_R;
#[doc = "Field `CC4P` reader - CC4P"]
pub use CC1P_R as CC4P_R;
#[doc = "Field `CC5P` reader - CC5P"]
pub use CC1P_R as CC5P_R;
#[doc = "Field `CC6P` reader - CC6P"]
pub use CC1P_R as CC6P_R;
#[doc = "Field `CC2P` writer - CC2P"]
pub use CC1P_W as CC2P_W;
#[doc = "Field `CC3P` writer - CC3P"]
pub use CC1P_W as CC3P_W;
#[doc = "Field `CC4P` writer - CC4P"]
pub use CC1P_W as CC4P_W;
#[doc = "Field `CC5P` writer - CC5P"]
pub use CC1P_W as CC5P_W;
#[doc = "Field `CC6P` writer - CC6P"]
pub use CC1P_W as CC6P_W;
impl R {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2NE"]
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CC3E"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC3P"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC3NE"]
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3NP"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4E"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CC4P"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CC5E"]
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC5P"]
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - CC6E"]
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CC6P"]
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<0> {
        CC1E_W::new(self)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<1> {
        CC1P_W::new(self)
    }
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<2> {
        CC1NE_W::new(self)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<4> {
        CC2E_W::new(self)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<5> {
        CC2P_W::new(self)
    }
    #[doc = "Bit 6 - CC2NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ne(&mut self) -> CC2NE_W<6> {
        CC2NE_W::new(self)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<7> {
        CC2NP_W::new(self)
    }
    #[doc = "Bit 8 - CC3E"]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> CC3E_W<8> {
        CC3E_W::new(self)
    }
    #[doc = "Bit 9 - CC3P"]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> CC3P_W<9> {
        CC3P_W::new(self)
    }
    #[doc = "Bit 10 - CC3NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ne(&mut self) -> CC3NE_W<10> {
        CC3NE_W::new(self)
    }
    #[doc = "Bit 11 - CC3NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> CC3NP_W<11> {
        CC3NP_W::new(self)
    }
    #[doc = "Bit 12 - CC4E"]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> CC4E_W<12> {
        CC4E_W::new(self)
    }
    #[doc = "Bit 13 - CC4P"]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> CC4P_W<13> {
        CC4P_W::new(self)
    }
    #[doc = "Bit 16 - CC5E"]
    #[inline(always)]
    #[must_use]
    pub fn cc5e(&mut self) -> CC5E_W<16> {
        CC5E_W::new(self)
    }
    #[doc = "Bit 17 - CC5P"]
    #[inline(always)]
    #[must_use]
    pub fn cc5p(&mut self) -> CC5P_W<17> {
        CC5P_W::new(self)
    }
    #[doc = "Bit 20 - CC6E"]
    #[inline(always)]
    #[must_use]
    pub fn cc6e(&mut self) -> CC6E_W<20> {
        CC6E_W::new(self)
    }
    #[doc = "Bit 21 - CC6P"]
    #[inline(always)]
    #[must_use]
    pub fn cc6p(&mut self) -> CC6P_W<21> {
        CC6P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](index.html) module"]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccer::R](R) reader structure"]
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccer::W](W) writer structure"]
impl crate::Writable for CCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
