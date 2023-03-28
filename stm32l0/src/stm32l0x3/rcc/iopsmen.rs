#[doc = "Register `IOPSMEN` reader"]
pub struct R(crate::R<IOPSMEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPSMEN` writer"]
pub struct W(crate::W<IOPSMEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMEN_SPEC>;
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
impl From<crate::W<IOPSMEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPASMEN` reader - IOPASMEN"]
pub type IOPASMEN_R = crate::BitReader<IOPASMEN_A>;
#[doc = "IOPASMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPASMEN_A {
    #[doc = "0: Port x clock is disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    Enabled = 1,
}
impl From<IOPASMEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOPASMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IOPASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPASMEN_A {
        match self.bits {
            false => IOPASMEN_A::Disabled,
            true => IOPASMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPASMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPASMEN_A::Enabled
    }
}
#[doc = "Field `IOPASMEN` writer - IOPASMEN"]
pub type IOPASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMEN_SPEC, IOPASMEN_A, O>;
impl<'a, const O: u8> IOPASMEN_W<'a, O> {
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPASMEN_A::Disabled)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPASMEN_A::Enabled)
    }
}
#[doc = "Field `IOPBSMEN` reader - IOPBSMEN"]
pub use IOPASMEN_R as IOPBSMEN_R;
#[doc = "Field `IOPCSMEN` reader - IOPCSMEN"]
pub use IOPASMEN_R as IOPCSMEN_R;
#[doc = "Field `IOPDSMEN` reader - IOPDSMEN"]
pub use IOPASMEN_R as IOPDSMEN_R;
#[doc = "Field `IOPESMEN` reader - Port E clock enable during Sleep mode bit"]
pub use IOPASMEN_R as IOPESMEN_R;
#[doc = "Field `IOPHSMEN` reader - IOPHSMEN"]
pub use IOPASMEN_R as IOPHSMEN_R;
#[doc = "Field `IOPBSMEN` writer - IOPBSMEN"]
pub use IOPASMEN_W as IOPBSMEN_W;
#[doc = "Field `IOPCSMEN` writer - IOPCSMEN"]
pub use IOPASMEN_W as IOPCSMEN_W;
#[doc = "Field `IOPDSMEN` writer - IOPDSMEN"]
pub use IOPASMEN_W as IOPDSMEN_W;
#[doc = "Field `IOPESMEN` writer - Port E clock enable during Sleep mode bit"]
pub use IOPASMEN_W as IOPESMEN_W;
#[doc = "Field `IOPHSMEN` writer - IOPHSMEN"]
pub use IOPASMEN_W as IOPHSMEN_W;
impl R {
    #[doc = "Bit 0 - IOPASMEN"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IOPBSMEN"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IOPCSMEN"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOPDSMEN"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&self) -> IOPESMEN_R {
        IOPESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - IOPHSMEN"]
    #[inline(always)]
    pub fn iophsmen(&self) -> IOPHSMEN_R {
        IOPHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IOPASMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopasmen(&mut self) -> IOPASMEN_W<0> {
        IOPASMEN_W::new(self)
    }
    #[doc = "Bit 1 - IOPBSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W<1> {
        IOPBSMEN_W::new(self)
    }
    #[doc = "Bit 2 - IOPCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W<2> {
        IOPCSMEN_W::new(self)
    }
    #[doc = "Bit 3 - IOPDSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W<3> {
        IOPDSMEN_W::new(self)
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn iopesmen(&mut self) -> IOPESMEN_W<4> {
        IOPESMEN_W::new(self)
    }
    #[doc = "Bit 7 - IOPHSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iophsmen(&mut self) -> IOPHSMEN_W<7> {
        IOPHSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmen](index.html) module"]
pub struct IOPSMEN_SPEC;
impl crate::RegisterSpec for IOPSMEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopsmen::R](R) reader structure"]
impl crate::Readable for IOPSMEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopsmen::W](W) writer structure"]
impl crate::Writable for IOPSMEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOPSMEN to value 0x8f"]
impl crate::Resettable for IOPSMEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x8f;
}
