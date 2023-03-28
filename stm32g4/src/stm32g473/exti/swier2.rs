#[doc = "Register `SWIER2` reader"]
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER2` writer"]
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI32` reader - Software interrupt on line 32"]
pub type SWI32_R = crate::BitReader<SWI32W_A>;
#[doc = "Software interrupt on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI32W_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI32W_A> for bool {
    #[inline(always)]
    fn from(variant: SWI32W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI32W_A> {
        match self.bits {
            true => Some(SWI32W_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI32W_A::Pend
    }
}
#[doc = "Field `SWI32` writer - Software interrupt on line 32"]
pub type SWI32_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, SWI32W_A, O>;
impl<'a, const O: u8> SWI32_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI32W_A::Pend)
    }
}
#[doc = "Field `SWI33` reader - Software interrupt on line 33"]
pub use SWI32_R as SWI33_R;
#[doc = "Field `SWI40` reader - Software interrupt on line 40"]
pub use SWI32_R as SWI40_R;
#[doc = "Field `SWI41` reader - Software interrupt on line 41"]
pub use SWI32_R as SWI41_R;
#[doc = "Field `SWI33` writer - Software interrupt on line 33"]
pub use SWI32_W as SWI33_W;
#[doc = "Field `SWI40` writer - Software interrupt on line 40"]
pub use SWI32_W as SWI40_W;
#[doc = "Field `SWI41` writer - Software interrupt on line 41"]
pub use SWI32_W as SWI41_W;
impl R {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swi32(&self) -> SWI32_R {
        SWI32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swi33(&self) -> SWI33_R {
        SWI33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt on line 40"]
    #[inline(always)]
    pub fn swi40(&self) -> SWI40_R {
        SWI40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt on line 41"]
    #[inline(always)]
    pub fn swi41(&self) -> SWI41_R {
        SWI41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    #[must_use]
    pub fn swi32(&mut self) -> SWI32_W<0> {
        SWI32_W::new(self)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    #[must_use]
    pub fn swi33(&mut self) -> SWI33_W<1> {
        SWI33_W::new(self)
    }
    #[doc = "Bit 8 - Software interrupt on line 40"]
    #[inline(always)]
    #[must_use]
    pub fn swi40(&mut self) -> SWI40_W<8> {
        SWI40_W::new(self)
    }
    #[doc = "Bit 9 - Software interrupt on line 41"]
    #[inline(always)]
    #[must_use]
    pub fn swi41(&mut self) -> SWI41_W<9> {
        SWI41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](index.html) module"]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier2::R](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier2::W](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
