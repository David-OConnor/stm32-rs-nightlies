#[doc = "Register `CPUPR3` reader"]
pub struct R(crate::R<CPUPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUPR3` writer"]
pub struct W(crate::W<CPUPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUPR3_SPEC>;
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
impl From<crate::W<CPUPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR82` reader - Configurable event inputs x+64 Pending bit"]
pub type PR82_R = crate::BitReader<PR82R_A>;
#[doc = "Configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR82R_A> for bool {
    #[inline(always)]
    fn from(variant: PR82R_A) -> Self {
        variant as u8 != 0
    }
}
impl PR82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR82R_A {
        match self.bits {
            false => PR82R_A::NotPending,
            true => PR82R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR82R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR82R_A::Pending
    }
}
#[doc = "Configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR82W_AW> for bool {
    #[inline(always)]
    fn from(variant: PR82W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR82` writer - Configurable event inputs x+64 Pending bit"]
pub type PR82_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CPUPR3_SPEC, PR82W_AW, O>;
impl<'a, const O: u8> PR82_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR82W_AW::Clear)
    }
}
#[doc = "Field `PR84` reader - Configurable event inputs x+64 Pending bit"]
pub use PR82_R as PR84_R;
#[doc = "Field `PR85` reader - Configurable event inputs x+64 Pending bit"]
pub use PR82_R as PR85_R;
#[doc = "Field `PR86` reader - Configurable event inputs x+64 Pending bit"]
pub use PR82_R as PR86_R;
#[doc = "Field `PR84` writer - Configurable event inputs x+64 Pending bit"]
pub use PR82_W as PR84_W;
#[doc = "Field `PR85` writer - Configurable event inputs x+64 Pending bit"]
pub use PR82_W as PR85_W;
#[doc = "Field `PR86` writer - Configurable event inputs x+64 Pending bit"]
pub use PR82_W as PR86_W;
impl R {
    #[doc = "Bit 18 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&self) -> PR82_R {
        PR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&self) -> PR84_R {
        PR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&self) -> PR85_R {
        PR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&self) -> PR86_R {
        PR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr82(&mut self) -> PR82_W<18> {
        PR82_W::new(self)
    }
    #[doc = "Bit 20 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr84(&mut self) -> PR84_W<20> {
        PR84_W::new(self)
    }
    #[doc = "Bit 21 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr85(&mut self) -> PR85_W<21> {
        PR85_W::new(self)
    }
    #[doc = "Bit 22 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pr86(&mut self) -> PR86_W<22> {
        PR86_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpupr3](index.html) module"]
pub struct CPUPR3_SPEC;
impl crate::RegisterSpec for CPUPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpupr3::R](R) reader structure"]
impl crate::Readable for CPUPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpupr3::W](W) writer structure"]
impl crate::Writable for CPUPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0074_0000;
}
#[doc = "`reset()` method sets CPUPR3 to value 0"]
impl crate::Resettable for CPUPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
