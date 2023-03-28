#[doc = "Register `CPUPR2` reader"]
pub struct R(crate::R<CPUPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUPR2` writer"]
pub struct W(crate::W<CPUPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUPR2_SPEC>;
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
impl From<crate::W<CPUPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR49` reader - Configurable event inputs x+32 Pending bit"]
pub type PR49_R = crate::BitReader<PR49R_A>;
#[doc = "Configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR49R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR49R_A> for bool {
    #[inline(always)]
    fn from(variant: PR49R_A) -> Self {
        variant as u8 != 0
    }
}
impl PR49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR49R_A {
        match self.bits {
            false => PR49R_A::NotPending,
            true => PR49R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR49R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR49R_A::Pending
    }
}
#[doc = "Configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR49W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR49W_AW> for bool {
    #[inline(always)]
    fn from(variant: PR49W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR49` writer - Configurable event inputs x+32 Pending bit"]
pub type PR49_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CPUPR2_SPEC, PR49W_AW, O>;
impl<'a, const O: u8> PR49_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR49W_AW::Clear)
    }
}
#[doc = "Field `PR51` reader - Configurable event inputs x+32 Pending bit"]
pub use PR49_R as PR51_R;
#[doc = "Field `PR51` writer - Configurable event inputs x+32 Pending bit"]
pub use PR49_W as PR51_W;
impl R {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr49(&mut self) -> PR49_W<17> {
        PR49_W::new(self)
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr51(&mut self) -> PR51_W<19> {
        PR51_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpupr2](index.html) module"]
pub struct CPUPR2_SPEC;
impl crate::RegisterSpec for CPUPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpupr2::R](R) reader structure"]
impl crate::Readable for CPUPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpupr2::W](W) writer structure"]
impl crate::Writable for CPUPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x000a_0000;
}
#[doc = "`reset()` method sets CPUPR2 to value 0"]
impl crate::Resettable for CPUPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
