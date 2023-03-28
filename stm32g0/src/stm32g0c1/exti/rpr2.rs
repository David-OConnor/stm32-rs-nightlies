#[doc = "Register `RPR2` reader"]
pub struct R(crate::R<RPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR2` writer"]
pub struct W(crate::W<RPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR2_SPEC>;
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
impl From<crate::W<RPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF2` reader - Rising edge event pending for configurable line 34"]
pub type RPIF2_R = crate::BitReader<RPIF2R_A>;
#[doc = "Rising edge event pending for configurable line 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF2R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<RPIF2R_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF2R_A) -> Self {
        variant as u8 != 0
    }
}
impl RPIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPIF2R_A {
        match self.bits {
            false => RPIF2R_A::NotPending,
            true => RPIF2R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF2R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF2R_A::Pending
    }
}
#[doc = "Rising edge event pending for configurable line 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF2W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<RPIF2W_AW> for bool {
    #[inline(always)]
    fn from(variant: RPIF2W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF2` writer - Rising edge event pending for configurable line 34"]
pub type RPIF2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, RPR2_SPEC, RPIF2W_AW, O>;
impl<'a, const O: u8> RPIF2_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF2W_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 2 - Rising edge event pending for configurable line 34"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Rising edge event pending for configurable line 34"]
    #[inline(always)]
    #[must_use]
    pub fn rpif2(&mut self) -> RPIF2_W<2> {
        RPIF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising edge pending register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr2](index.html) module"]
pub struct RPR2_SPEC;
impl crate::RegisterSpec for RPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr2::R](R) reader structure"]
impl crate::Readable for RPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr2::W](W) writer structure"]
impl crate::Writable for RPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x04;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
