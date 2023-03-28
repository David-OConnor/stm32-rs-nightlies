#[doc = "Register `C1EMR2` reader"]
pub struct R(crate::R<C1EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1EMR2` writer"]
pub struct W(crate::W<C1EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1EMR2_SPEC>;
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
impl From<crate::W<C1EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM40` reader - Wakeup with event generation Mask on Event input"]
pub type EM40_R = crate::BitReader<EM40_A>;
#[doc = "Wakeup with event generation Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM40_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<EM40_A> for bool {
    #[inline(always)]
    fn from(variant: EM40_A) -> Self {
        variant as u8 != 0
    }
}
impl EM40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM40_A {
        match self.bits {
            false => EM40_A::Masked,
            true => EM40_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM40_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM40_A::Unmasked
    }
}
#[doc = "Field `EM40` writer - Wakeup with event generation Mask on Event input"]
pub type EM40_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1EMR2_SPEC, EM40_A, O>;
impl<'a, const O: u8> EM40_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM40_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM40_A::Unmasked)
    }
}
#[doc = "Field `EM41` reader - Wakeup with event generation Mask on Event input"]
pub use EM40_R as EM41_R;
#[doc = "Field `EM41` writer - Wakeup with event generation Mask on Event input"]
pub use EM40_W as EM41_W;
impl R {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<8> {
        EM40_W::new(self)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<9> {
        EM41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1emr2](index.html) module"]
pub struct C1EMR2_SPEC;
impl crate::RegisterSpec for C1EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1emr2::R](R) reader structure"]
impl crate::Readable for C1EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1emr2::W](W) writer structure"]
impl crate::Writable for C1EMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1EMR2 to value 0"]
impl crate::Resettable for C1EMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
