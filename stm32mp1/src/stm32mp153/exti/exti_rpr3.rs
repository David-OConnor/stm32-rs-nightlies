#[doc = "Register `EXTI_RPR3` reader"]
pub struct R(crate::R<EXTI_RPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_RPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_RPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_RPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_RPR3` writer"]
pub struct W(crate::W<EXTI_RPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_RPR3_SPEC>;
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
impl From<crate::W<EXTI_RPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_RPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF65` reader - RPIF65"]
pub type RPIF65_R = crate::BitReader<bool>;
#[doc = "Field `RPIF65` writer - RPIF65"]
pub type RPIF65_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR3_SPEC, bool, O>;
#[doc = "Field `RPIF66` reader - RPIF66"]
pub type RPIF66_R = crate::BitReader<bool>;
#[doc = "Field `RPIF66` writer - RPIF66"]
pub type RPIF66_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR3_SPEC, bool, O>;
#[doc = "Field `RPIF68` reader - RPIF68"]
pub type RPIF68_R = crate::BitReader<bool>;
#[doc = "Field `RPIF68` writer - RPIF68"]
pub type RPIF68_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR3_SPEC, bool, O>;
#[doc = "Field `RPIF73` reader - RPIF73"]
pub type RPIF73_R = crate::BitReader<bool>;
#[doc = "Field `RPIF73` writer - RPIF73"]
pub type RPIF73_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR3_SPEC, bool, O>;
#[doc = "Field `RPIF74` reader - RPIF74"]
pub type RPIF74_R = crate::BitReader<bool>;
#[doc = "Field `RPIF74` writer - RPIF74"]
pub type RPIF74_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    pub fn rpif65(&self) -> RPIF65_R {
        RPIF65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    pub fn rpif66(&self) -> RPIF66_R {
        RPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    pub fn rpif68(&self) -> RPIF68_R {
        RPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    pub fn rpif73(&self) -> RPIF73_R {
        RPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    pub fn rpif74(&self) -> RPIF74_R {
        RPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    #[must_use]
    pub fn rpif65(&mut self) -> RPIF65_W<1> {
        RPIF65_W::new(self)
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    #[must_use]
    pub fn rpif66(&mut self) -> RPIF66_W<2> {
        RPIF66_W::new(self)
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    #[must_use]
    pub fn rpif68(&mut self) -> RPIF68_W<4> {
        RPIF68_W::new(self)
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    #[must_use]
    pub fn rpif73(&mut self) -> RPIF73_W<9> {
        RPIF73_W::new(self)
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    #[must_use]
    pub fn rpif74(&mut self) -> RPIF74_W<10> {
        RPIF74_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rpr3](index.html) module"]
pub struct EXTI_RPR3_SPEC;
impl crate::RegisterSpec for EXTI_RPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_rpr3::R](R) reader structure"]
impl crate::Readable for EXTI_RPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_rpr3::W](W) writer structure"]
impl crate::Writable for EXTI_RPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_RPR3 to value 0"]
impl crate::Resettable for EXTI_RPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
