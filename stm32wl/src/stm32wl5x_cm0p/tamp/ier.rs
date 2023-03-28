#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1IE` reader - TAMP1IE"]
pub type TAMP1IE_R = crate::BitReader<TAMP1IE_A>;
#[doc = "TAMP1IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1IE_A {
    #[doc = "0: Tamper x interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tampoer x interrupt enabled"]
    Enabled = 1,
}
impl From<TAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1IE_A {
        match self.bits {
            false => TAMP1IE_A::Disabled,
            true => TAMP1IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1IE_A::Enabled
    }
}
#[doc = "Field `TAMP1IE` writer - TAMP1IE"]
pub type TAMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TAMP1IE_A, O>;
impl<'a, const O: u8> TAMP1IE_W<'a, O> {
    #[doc = "Tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::Disabled)
    }
    #[doc = "Tampoer x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::Enabled)
    }
}
#[doc = "Field `TAMP2IE` reader - TAMP2IE"]
pub use TAMP1IE_R as TAMP2IE_R;
#[doc = "Field `TAMP3IE` reader - TAMP3IE"]
pub use TAMP1IE_R as TAMP3IE_R;
#[doc = "Field `TAMP2IE` writer - TAMP2IE"]
pub use TAMP1IE_W as TAMP2IE_W;
#[doc = "Field `TAMP3IE` writer - TAMP3IE"]
pub use TAMP1IE_W as TAMP3IE_W;
#[doc = "Field `ITAMP3IE` reader - ITAMP3IE"]
pub type ITAMP3IE_R = crate::BitReader<ITAMP3IE_A>;
#[doc = "ITAMP3IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3IE_A {
    #[doc = "0: Internal tamper x interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Internal tamper x interrupt enabled"]
    Enabled = 1,
}
impl From<ITAMP3IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3IE_A) -> Self {
        variant as u8 != 0
    }
}
impl ITAMP3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3IE_A {
        match self.bits {
            false => ITAMP3IE_A::Disabled,
            true => ITAMP3IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITAMP3IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITAMP3IE_A::Enabled
    }
}
#[doc = "Field `ITAMP3IE` writer - ITAMP3IE"]
pub type ITAMP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ITAMP3IE_A, O>;
impl<'a, const O: u8> ITAMP3IE_W<'a, O> {
    #[doc = "Internal tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP3IE_A::Disabled)
    }
    #[doc = "Internal tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP3IE_A::Enabled)
    }
}
#[doc = "Field `ITAMP5IE` reader - ITAMP5IE"]
pub use ITAMP3IE_R as ITAMP5IE_R;
#[doc = "Field `ITAMP6IE` reader - ITAMP6IE"]
pub use ITAMP3IE_R as ITAMP6IE_R;
#[doc = "Field `ITAMP8IE` reader - ITAMP8IE"]
pub use ITAMP3IE_R as ITAMP8IE_R;
#[doc = "Field `ITAMP5IE` writer - ITAMP5IE"]
pub use ITAMP3IE_W as ITAMP5IE_W;
#[doc = "Field `ITAMP6IE` writer - ITAMP6IE"]
pub use ITAMP3IE_W as ITAMP6IE_W;
#[doc = "Field `ITAMP8IE` writer - ITAMP8IE"]
pub use ITAMP3IE_W as ITAMP8IE_W;
impl R {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<0> {
        TAMP1IE_W::new(self)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<1> {
        TAMP2IE_W::new(self)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<2> {
        TAMP3IE_W::new(self)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<18> {
        ITAMP3IE_W::new(self)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<20> {
        ITAMP5IE_W::new(self)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W<21> {
        ITAMP6IE_W::new(self)
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W<23> {
        ITAMP8IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
