#[doc = "Register `AHB2LPENR` reader"]
pub struct R(crate::R<AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2LPENR` writer"]
pub struct W(crate::W<AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2LPENR_SPEC>;
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
impl From<crate::W<AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESLPEN` reader - AES module clock enable during Sleep mode"]
pub type AESLPEN_R = crate::BitReader<AESLPEN_A>;
#[doc = "AES module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<AESLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AESLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESLPEN_A {
        match self.bits {
            false => AESLPEN_A::DisabledInSleep,
            true => AESLPEN_A::EnabledInSleep,
        }
    }
    #[doc = "Checks if the value of the field is `DisabledInSleep`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == AESLPEN_A::DisabledInSleep
    }
    #[doc = "Checks if the value of the field is `EnabledInSleep`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == AESLPEN_A::EnabledInSleep
    }
}
#[doc = "Field `AESLPEN` writer - AES module clock enable during Sleep mode"]
pub type AESLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, AESLPEN_A, O>;
impl<'a, const O: u8> AESLPEN_W<'a, O> {
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(AESLPEN_A::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(AESLPEN_A::EnabledInSleep)
    }
}
#[doc = "Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode"]
pub use AESLPEN_R as RNGLPEN_R;
#[doc = "Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode"]
pub use AESLPEN_R as OTGFSLPEN_R;
#[doc = "Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode"]
pub use AESLPEN_W as RNGLPEN_W;
#[doc = "Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode"]
pub use AESLPEN_W as OTGFSLPEN_W;
impl R {
    #[doc = "Bit 4 - AES module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn aeslpen(&self) -> AESLPEN_R {
        AESLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - AES module clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn aeslpen(&mut self) -> AESLPEN_W<4> {
        AESLPEN_W::new(self)
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<6> {
        RNGLPEN_W::new(self)
    }
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W<7> {
        OTGFSLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2lpenr](index.html) module"]
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2lpenr::R](R) reader structure"]
impl crate::Readable for AHB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2lpenr::W](W) writer structure"]
impl crate::Writable for AHB2LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0xf1"]
impl crate::Resettable for AHB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf1;
}
