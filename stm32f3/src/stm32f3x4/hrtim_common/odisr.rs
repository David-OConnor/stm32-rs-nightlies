#[doc = "Register `ODISR` reader"]
pub struct R(crate::R<ODISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODISR` writer"]
pub struct W(crate::W<ODISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODISR_SPEC>;
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
impl From<crate::W<ODISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TA1ODIS` reader - TA1ODIS"]
pub type TA1ODIS_R = crate::BitReader<TA1ODIS_A>;
#[doc = "TA1ODIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1ODIS_A {
    #[doc = "1: Disable output"]
    Disable = 1,
}
impl From<TA1ODIS_A> for bool {
    #[inline(always)]
    fn from(variant: TA1ODIS_A) -> Self {
        variant as u8 != 0
    }
}
impl TA1ODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TA1ODIS_A> {
        match self.bits {
            true => Some(TA1ODIS_A::Disable),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Disable`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TA1ODIS_A::Disable
    }
}
#[doc = "Field `TA1ODIS` writer - TA1ODIS"]
pub type TA1ODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODISR_SPEC, TA1ODIS_A, O>;
impl<'a, const O: u8> TA1ODIS_W<'a, O> {
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TA1ODIS_A::Disable)
    }
}
#[doc = "Field `TA2ODIS` reader - TA2ODIS"]
pub use TA1ODIS_R as TA2ODIS_R;
#[doc = "Field `TB1ODIS` reader - TB1ODIS"]
pub use TA1ODIS_R as TB1ODIS_R;
#[doc = "Field `TB2ODIS` reader - TB2ODIS"]
pub use TA1ODIS_R as TB2ODIS_R;
#[doc = "Field `TC1ODIS` reader - TC1ODIS"]
pub use TA1ODIS_R as TC1ODIS_R;
#[doc = "Field `TC2ODIS` reader - TC2ODIS"]
pub use TA1ODIS_R as TC2ODIS_R;
#[doc = "Field `TD1ODIS` reader - TD1ODIS"]
pub use TA1ODIS_R as TD1ODIS_R;
#[doc = "Field `TD2ODIS` reader - TD2ODIS"]
pub use TA1ODIS_R as TD2ODIS_R;
#[doc = "Field `TE1ODIS` reader - TE1ODIS"]
pub use TA1ODIS_R as TE1ODIS_R;
#[doc = "Field `TE2ODIS` reader - TE2ODIS"]
pub use TA1ODIS_R as TE2ODIS_R;
#[doc = "Field `TA2ODIS` writer - TA2ODIS"]
pub use TA1ODIS_W as TA2ODIS_W;
#[doc = "Field `TB1ODIS` writer - TB1ODIS"]
pub use TA1ODIS_W as TB1ODIS_W;
#[doc = "Field `TB2ODIS` writer - TB2ODIS"]
pub use TA1ODIS_W as TB2ODIS_W;
#[doc = "Field `TC1ODIS` writer - TC1ODIS"]
pub use TA1ODIS_W as TC1ODIS_W;
#[doc = "Field `TC2ODIS` writer - TC2ODIS"]
pub use TA1ODIS_W as TC2ODIS_W;
#[doc = "Field `TD1ODIS` writer - TD1ODIS"]
pub use TA1ODIS_W as TD1ODIS_W;
#[doc = "Field `TD2ODIS` writer - TD2ODIS"]
pub use TA1ODIS_W as TD2ODIS_W;
#[doc = "Field `TE1ODIS` writer - TE1ODIS"]
pub use TA1ODIS_W as TE1ODIS_W;
#[doc = "Field `TE2ODIS` writer - TE2ODIS"]
pub use TA1ODIS_W as TE2ODIS_W;
impl R {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    pub fn ta1odis(&self) -> TA1ODIS_R {
        TA1ODIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    pub fn ta2odis(&self) -> TA2ODIS_R {
        TA2ODIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    pub fn tb1odis(&self) -> TB1ODIS_R {
        TB1ODIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    pub fn tb2odis(&self) -> TB2ODIS_R {
        TB2ODIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    pub fn tc1odis(&self) -> TC1ODIS_R {
        TC1ODIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    pub fn tc2odis(&self) -> TC2ODIS_R {
        TC2ODIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    pub fn td1odis(&self) -> TD1ODIS_R {
        TD1ODIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    pub fn td2odis(&self) -> TD2ODIS_R {
        TD2ODIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    pub fn te1odis(&self) -> TE1ODIS_R {
        TE1ODIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    pub fn te2odis(&self) -> TE2ODIS_R {
        TE2ODIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta1odis(&mut self) -> TA1ODIS_W<0> {
        TA1ODIS_W::new(self)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta2odis(&mut self) -> TA2ODIS_W<1> {
        TA2ODIS_W::new(self)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb1odis(&mut self) -> TB1ODIS_W<2> {
        TB1ODIS_W::new(self)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb2odis(&mut self) -> TB2ODIS_W<3> {
        TB2ODIS_W::new(self)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc1odis(&mut self) -> TC1ODIS_W<4> {
        TC1ODIS_W::new(self)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc2odis(&mut self) -> TC2ODIS_W<5> {
        TC2ODIS_W::new(self)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td1odis(&mut self) -> TD1ODIS_W<6> {
        TD1ODIS_W::new(self)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td2odis(&mut self) -> TD2ODIS_W<7> {
        TD2ODIS_W::new(self)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te1odis(&mut self) -> TE1ODIS_W<8> {
        TE1ODIS_W::new(self)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te2odis(&mut self) -> TE2ODIS_W<9> {
        TE2ODIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odisr](index.html) module"]
pub struct ODISR_SPEC;
impl crate::RegisterSpec for ODISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odisr::R](R) reader structure"]
impl crate::Readable for ODISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odisr::W](W) writer structure"]
impl crate::Writable for ODISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODISR to value 0"]
impl crate::Resettable for ODISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
