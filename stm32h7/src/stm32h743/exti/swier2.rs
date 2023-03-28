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
#[doc = "Field `SWIER49` reader - Software interrupt on line x+32"]
pub type SWIER49_R = crate::BitReader<SWIER49W_A>;
#[doc = "Software interrupt on line x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIER49W_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER49W_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER49W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWIER49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER49W_A> {
        match self.bits {
            true => Some(SWIER49W_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER49W_A::Pend
    }
}
#[doc = "Field `SWIER49` writer - Software interrupt on line x+32"]
pub type SWIER49_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, SWIER49W_A, O>;
impl<'a, const O: u8> SWIER49_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER49W_A::Pend)
    }
}
#[doc = "Field `SWIER51` reader - Software interrupt on line x+32"]
pub use SWIER49_R as SWIER51_R;
#[doc = "Field `SWIER51` writer - Software interrupt on line x+32"]
pub use SWIER49_W as SWIER51_W;
impl R {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier49(&self) -> SWIER49_R {
        SWIER49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier51(&self) -> SWIER51_R {
        SWIER51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    #[must_use]
    pub fn swier49(&mut self) -> SWIER49_W<17> {
        SWIER49_W::new(self)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    #[must_use]
    pub fn swier51(&mut self) -> SWIER51_W<19> {
        SWIER51_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](index.html) module"]
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
