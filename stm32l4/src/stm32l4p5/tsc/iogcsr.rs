#[doc = "Register `IOGCSR` reader"]
pub struct R(crate::R<IOGCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOGCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOGCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOGCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOGCSR` writer"]
pub struct W(crate::W<IOGCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOGCSR_SPEC>;
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
impl From<crate::W<IOGCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOGCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `G1E` reader - Analog I/O group x enable"]
pub type G1E_R = crate::BitReader<G1E_A>;
#[doc = "Analog I/O group x enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1E_A {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    Disabled = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    Enabled = 1,
}
impl From<G1E_A> for bool {
    #[inline(always)]
    fn from(variant: G1E_A) -> Self {
        variant as u8 != 0
    }
}
impl G1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> G1E_A {
        match self.bits {
            false => G1E_A::Disabled,
            true => G1E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == G1E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == G1E_A::Enabled
    }
}
#[doc = "Field `G1E` writer - Analog I/O group x enable"]
pub type G1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, G1E_A, O>;
impl<'a, const O: u8> G1E_W<'a, O> {
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(G1E_A::Disabled)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(G1E_A::Enabled)
    }
}
#[doc = "Field `G2E` reader - Analog I/O group x enable"]
pub use G1E_R as G2E_R;
#[doc = "Field `G3E` reader - Analog I/O group x enable"]
pub use G1E_R as G3E_R;
#[doc = "Field `G4E` reader - Analog I/O group x enable"]
pub use G1E_R as G4E_R;
#[doc = "Field `G5E` reader - Analog I/O group x enable"]
pub use G1E_R as G5E_R;
#[doc = "Field `G6E` reader - Analog I/O group x enable"]
pub use G1E_R as G6E_R;
#[doc = "Field `G7E` reader - Analog I/O group x enable"]
pub use G1E_R as G7E_R;
#[doc = "Field `G8E` reader - Analog I/O group x enable"]
pub use G1E_R as G8E_R;
#[doc = "Field `G2E` writer - Analog I/O group x enable"]
pub use G1E_W as G2E_W;
#[doc = "Field `G3E` writer - Analog I/O group x enable"]
pub use G1E_W as G3E_W;
#[doc = "Field `G4E` writer - Analog I/O group x enable"]
pub use G1E_W as G4E_W;
#[doc = "Field `G5E` writer - Analog I/O group x enable"]
pub use G1E_W as G5E_W;
#[doc = "Field `G6E` writer - Analog I/O group x enable"]
pub use G1E_W as G6E_W;
#[doc = "Field `G7E` writer - Analog I/O group x enable"]
pub use G1E_W as G7E_W;
#[doc = "Field `G8E` writer - Analog I/O group x enable"]
pub use G1E_W as G8E_W;
#[doc = "Field `G1S` reader - Analog I/O group x status"]
pub type G1S_R = crate::BitReader<G1S_A>;
#[doc = "Analog I/O group x status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1S_A {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    Ongoing = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    Complete = 1,
}
impl From<G1S_A> for bool {
    #[inline(always)]
    fn from(variant: G1S_A) -> Self {
        variant as u8 != 0
    }
}
impl G1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> G1S_A {
        match self.bits {
            false => G1S_A::Ongoing,
            true => G1S_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `Ongoing`"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == G1S_A::Ongoing
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == G1S_A::Complete
    }
}
#[doc = "Field `G2S` reader - Analog I/O group x status"]
pub use G1S_R as G2S_R;
#[doc = "Field `G3S` reader - Analog I/O group x status"]
pub use G1S_R as G3S_R;
#[doc = "Field `G4S` reader - Analog I/O group x status"]
pub use G1S_R as G4S_R;
#[doc = "Field `G5S` reader - Analog I/O group x status"]
pub use G1S_R as G5S_R;
#[doc = "Field `G6S` reader - Analog I/O group x status"]
pub use G1S_R as G6S_R;
#[doc = "Field `G7S` reader - Analog I/O group x status"]
pub use G1S_R as G7S_R;
#[doc = "Field `G8S` reader - Analog I/O group x status"]
pub use G1S_R as G8S_R;
impl R {
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g1e(&self) -> G1E_R {
        G1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g2e(&self) -> G2E_R {
        G2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g3e(&self) -> G3E_R {
        G3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g4e(&self) -> G4E_R {
        G4E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g5e(&self) -> G5E_R {
        G5E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g6e(&self) -> G6E_R {
        G6E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g7e(&self) -> G7E_R {
        G7E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g8e(&self) -> G8E_R {
        G8E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g1s(&self) -> G1S_R {
        G1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g2s(&self) -> G2S_R {
        G2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g3s(&self) -> G3S_R {
        G3S_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g4s(&self) -> G4S_R {
        G4S_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g5s(&self) -> G5S_R {
        G5S_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g6s(&self) -> G6S_R {
        G6S_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g7s(&self) -> G7S_R {
        G7S_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g8s(&self) -> G8S_R {
        G8S_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g1e(&mut self) -> G1E_W<0> {
        G1E_W::new(self)
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g2e(&mut self) -> G2E_W<1> {
        G2E_W::new(self)
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g3e(&mut self) -> G3E_W<2> {
        G3E_W::new(self)
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g4e(&mut self) -> G4E_W<3> {
        G4E_W::new(self)
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g5e(&mut self) -> G5E_W<4> {
        G5E_W::new(self)
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g6e(&mut self) -> G6E_W<5> {
        G6E_W::new(self)
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g7e(&mut self) -> G7E_W<6> {
        G7E_W::new(self)
    }
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g8e(&mut self) -> G8E_W<7> {
        G8E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O group control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iogcsr](index.html) module"]
pub struct IOGCSR_SPEC;
impl crate::RegisterSpec for IOGCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iogcsr::R](R) reader structure"]
impl crate::Readable for IOGCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iogcsr::W](W) writer structure"]
impl crate::Writable for IOGCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOGCSR to value 0"]
impl crate::Resettable for IOGCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
