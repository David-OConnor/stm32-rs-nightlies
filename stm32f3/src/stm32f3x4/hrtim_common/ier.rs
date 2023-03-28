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
#[doc = "Field `FLT1IE` reader - Fault 1 Interrupt Enable"]
pub type FLT1IE_R = crate::BitReader<FLT1IE_A>;
#[doc = "Fault 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1IE_A {
    #[doc = "0: Fault interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Fault interrupt enabled"]
    Enabled = 1,
}
impl From<FLT1IE_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1IE_A {
        match self.bits {
            false => FLT1IE_A::Disabled,
            true => FLT1IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT1IE_A::Enabled
    }
}
#[doc = "Field `FLT1IE` writer - Fault 1 Interrupt Enable"]
pub type FLT1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FLT1IE_A, O>;
impl<'a, const O: u8> FLT1IE_W<'a, O> {
    #[doc = "Fault interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT1IE_A::Disabled)
    }
    #[doc = "Fault interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT1IE_A::Enabled)
    }
}
#[doc = "Field `FLT2IE` reader - Fault 2 Interrupt Enable"]
pub use FLT1IE_R as FLT2IE_R;
#[doc = "Field `FLT3IE` reader - Fault 3 Interrupt Enable"]
pub use FLT1IE_R as FLT3IE_R;
#[doc = "Field `FLT4IE` reader - Fault 4 Interrupt Enable"]
pub use FLT1IE_R as FLT4IE_R;
#[doc = "Field `FLT5IE` reader - Fault 5 Interrupt Enable"]
pub use FLT1IE_R as FLT5IE_R;
#[doc = "Field `SYSFLTIE` reader - System Fault Interrupt Enable"]
pub use FLT1IE_R as SYSFLTIE_R;
#[doc = "Field `FLT2IE` writer - Fault 2 Interrupt Enable"]
pub use FLT1IE_W as FLT2IE_W;
#[doc = "Field `FLT3IE` writer - Fault 3 Interrupt Enable"]
pub use FLT1IE_W as FLT3IE_W;
#[doc = "Field `FLT4IE` writer - Fault 4 Interrupt Enable"]
pub use FLT1IE_W as FLT4IE_W;
#[doc = "Field `FLT5IE` writer - Fault 5 Interrupt Enable"]
pub use FLT1IE_W as FLT5IE_W;
#[doc = "Field `SYSFLTIE` writer - System Fault Interrupt Enable"]
pub use FLT1IE_W as SYSFLTIE_W;
#[doc = "Field `DLLRDYIE` reader - DLL Ready Interrupt Enable"]
pub type DLLRDYIE_R = crate::BitReader<DLLRDYIE_A>;
#[doc = "DLL Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRDYIE_A {
    #[doc = "0: DLL ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DLL Ready interrupt enabled"]
    Enabled = 1,
}
impl From<DLLRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DLLRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLRDYIE_A {
        match self.bits {
            false => DLLRDYIE_A::Disabled,
            true => DLLRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLLRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLLRDYIE_A::Enabled
    }
}
#[doc = "Field `DLLRDYIE` writer - DLL Ready Interrupt Enable"]
pub type DLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, DLLRDYIE_A, O>;
impl<'a, const O: u8> DLLRDYIE_W<'a, O> {
    #[doc = "DLL ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLLRDYIE_A::Disabled)
    }
    #[doc = "DLL Ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLLRDYIE_A::Enabled)
    }
}
#[doc = "Field `BMPERIE` reader - Burst mode period Interrupt Enable"]
pub type BMPERIE_R = crate::BitReader<BMPERIE_A>;
#[doc = "Burst mode period Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERIE_A {
    #[doc = "0: Burst mode period interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Burst mode period interrupt enabled"]
    Enabled = 1,
}
impl From<BMPERIE_A> for bool {
    #[inline(always)]
    fn from(variant: BMPERIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BMPERIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMPERIE_A {
        match self.bits {
            false => BMPERIE_A::Disabled,
            true => BMPERIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BMPERIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BMPERIE_A::Enabled
    }
}
#[doc = "Field `BMPERIE` writer - Burst mode period Interrupt Enable"]
pub type BMPERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, BMPERIE_A, O>;
impl<'a, const O: u8> BMPERIE_W<'a, O> {
    #[doc = "Burst mode period interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BMPERIE_A::Disabled)
    }
    #[doc = "Burst mode period interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BMPERIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn flt1ie(&self) -> FLT1IE_R {
        FLT1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    pub fn flt2ie(&self) -> FLT2IE_R {
        FLT2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    pub fn flt3ie(&self) -> FLT3IE_R {
        FLT3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    pub fn flt4ie(&self) -> FLT4IE_R {
        FLT4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    pub fn flt5ie(&self) -> FLT5IE_R {
        FLT5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    pub fn sysfltie(&self) -> SYSFLTIE_R {
        SYSFLTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dllrdyie(&self) -> DLLRDYIE_R {
        DLLRDYIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    pub fn bmperie(&self) -> BMPERIE_R {
        BMPERIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1ie(&mut self) -> FLT1IE_W<0> {
        FLT1IE_W::new(self)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2ie(&mut self) -> FLT2IE_W<1> {
        FLT2IE_W::new(self)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3ie(&mut self) -> FLT3IE_W<2> {
        FLT3IE_W::new(self)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4ie(&mut self) -> FLT4IE_W<3> {
        FLT4IE_W::new(self)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5ie(&mut self) -> FLT5IE_W<4> {
        FLT5IE_W::new(self)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysfltie(&mut self) -> SYSFLTIE_W<5> {
        SYSFLTIE_W::new(self)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdyie(&mut self) -> DLLRDYIE_W<16> {
        DLLRDYIE_W::new(self)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmperie(&mut self) -> BMPERIE_W<17> {
        BMPERIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
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
