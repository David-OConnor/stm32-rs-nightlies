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
#[doc = "Field `EOAIE` reader - End of acquisition interrupt enable"]
pub type EOAIE_R = crate::BitReader<EOAIE_A>;
#[doc = "End of acquisition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOAIE_A {
    #[doc = "0: End of acquisition interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of acquisition interrupt enabled"]
    Enabled = 1,
}
impl From<EOAIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOAIE_A {
        match self.bits {
            false => EOAIE_A::Disabled,
            true => EOAIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOAIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOAIE_A::Enabled
    }
}
#[doc = "Field `EOAIE` writer - End of acquisition interrupt enable"]
pub type EOAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOAIE_A, O>;
impl<'a, const O: u8> EOAIE_W<'a, O> {
    #[doc = "End of acquisition interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOAIE_A::Disabled)
    }
    #[doc = "End of acquisition interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOAIE_A::Enabled)
    }
}
#[doc = "Field `MCEIE` reader - Max count error interrupt enable"]
pub type MCEIE_R = crate::BitReader<MCEIE_A>;
#[doc = "Max count error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCEIE_A {
    #[doc = "0: Max count error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Max count error interrupt enabled"]
    Enabled = 1,
}
impl From<MCEIE_A> for bool {
    #[inline(always)]
    fn from(variant: MCEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCEIE_A {
        match self.bits {
            false => MCEIE_A::Disabled,
            true => MCEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCEIE_A::Enabled
    }
}
#[doc = "Field `MCEIE` writer - Max count error interrupt enable"]
pub type MCEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, MCEIE_A, O>;
impl<'a, const O: u8> MCEIE_W<'a, O> {
    #[doc = "Max count error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCEIE_A::Disabled)
    }
    #[doc = "Max count error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCEIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - End of acquisition interrupt enable"]
    #[inline(always)]
    pub fn eoaie(&self) -> EOAIE_R {
        EOAIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error interrupt enable"]
    #[inline(always)]
    pub fn mceie(&self) -> MCEIE_R {
        MCEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoaie(&mut self) -> EOAIE_W<0> {
        EOAIE_W::new(self)
    }
    #[doc = "Bit 1 - Max count error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceie(&mut self) -> MCEIE_W<1> {
        MCEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
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
