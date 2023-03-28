#[doc = "Register `AHB3RSTR` reader"]
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3RSTR` writer"]
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCRST` reader - Flexible memory controller reset"]
pub type FMCRST_R = crate::BitReader<FMCRST_A>;
#[doc = "Flexible memory controller reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCRST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset FMC"]
    Reset = 1,
}
impl From<FMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCRST_A {
        match self.bits {
            false => FMCRST_A::NoEffect,
            true => FMCRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FMCRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FMCRST_A::Reset
    }
}
#[doc = "Field `FMCRST` writer - Flexible memory controller reset"]
pub type FMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, FMCRST_A, O>;
impl<'a, const O: u8> FMCRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FMCRST_A::NoEffect)
    }
    #[doc = "Reset FMC"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMCRST_A::Reset)
    }
}
#[doc = "Field `OSPI1RST` reader - OctoSPI1 memory interface reset"]
pub type OSPI1RST_R = crate::BitReader<OSPI1RST_A>;
#[doc = "OctoSPI1 memory interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1RST_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset OctoSPIx"]
    Reset = 1,
}
impl From<OSPI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: OSPI1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl OSPI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPI1RST_A {
        match self.bits {
            false => OSPI1RST_A::NoEffect,
            true => OSPI1RST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSPI1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OSPI1RST_A::Reset
    }
}
#[doc = "Field `OSPI1RST` writer - OctoSPI1 memory interface reset"]
pub type OSPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, OSPI1RST_A, O>;
impl<'a, const O: u8> OSPI1RST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OSPI1RST_A::NoEffect)
    }
    #[doc = "Reset OctoSPIx"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OSPI1RST_A::Reset)
    }
}
#[doc = "Field `OSPI2RST` reader - OctOSPI2 memory interface reset"]
pub use OSPI1RST_R as OSPI2RST_R;
#[doc = "Field `OSPI2RST` writer - OctOSPI2 memory interface reset"]
pub use OSPI1RST_W as OSPI2RST_W;
impl R {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface reset"]
    #[inline(always)]
    pub fn ospi1rst(&self) -> OSPI1RST_R {
        OSPI1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OctOSPI2 memory interface reset"]
    #[inline(always)]
    pub fn ospi2rst(&self) -> OSPI2RST_R {
        OSPI2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<0> {
        FMCRST_W::new(self)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn ospi1rst(&mut self) -> OSPI1RST_W<8> {
        OSPI1RST_W::new(self)
    }
    #[doc = "Bit 9 - OctOSPI2 memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn ospi2rst(&mut self) -> OSPI2RST_W<9> {
        OSPI2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](index.html) module"]
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3rstr::R](R) reader structure"]
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure"]
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
