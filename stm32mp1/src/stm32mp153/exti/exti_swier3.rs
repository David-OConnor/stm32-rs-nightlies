#[doc = "Register `EXTI_SWIER3` reader"]
pub struct R(crate::R<EXTI_SWIER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_SWIER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_SWIER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_SWIER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_SWIER3` writer"]
pub struct W(crate::W<EXTI_SWIER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_SWIER3_SPEC>;
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
impl From<crate::W<EXTI_SWIER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_SWIER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI65` reader - SWI65"]
pub type SWI65_R = crate::BitReader<bool>;
#[doc = "Field `SWI65` writer - SWI65"]
pub type SWI65_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER3_SPEC, bool, O>;
#[doc = "Field `SWI66` reader - SWI66"]
pub type SWI66_R = crate::BitReader<bool>;
#[doc = "Field `SWI66` writer - SWI66"]
pub type SWI66_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER3_SPEC, bool, O>;
#[doc = "Field `SWI68` reader - SWI68"]
pub type SWI68_R = crate::BitReader<bool>;
#[doc = "Field `SWI68` writer - SWI68"]
pub type SWI68_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER3_SPEC, bool, O>;
#[doc = "Field `SWI73` reader - SWI73"]
pub type SWI73_R = crate::BitReader<bool>;
#[doc = "Field `SWI73` writer - SWI73"]
pub type SWI73_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER3_SPEC, bool, O>;
#[doc = "Field `SWI74` reader - SWI74"]
pub type SWI74_R = crate::BitReader<bool>;
#[doc = "Field `SWI74` writer - SWI74"]
pub type SWI74_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    pub fn swi65(&self) -> SWI65_R {
        SWI65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    pub fn swi66(&self) -> SWI66_R {
        SWI66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    pub fn swi68(&self) -> SWI68_R {
        SWI68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    pub fn swi73(&self) -> SWI73_R {
        SWI73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    pub fn swi74(&self) -> SWI74_R {
        SWI74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    #[must_use]
    pub fn swi65(&mut self) -> SWI65_W<1> {
        SWI65_W::new(self)
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    #[must_use]
    pub fn swi66(&mut self) -> SWI66_W<2> {
        SWI66_W::new(self)
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    #[must_use]
    pub fn swi68(&mut self) -> SWI68_W<4> {
        SWI68_W::new(self)
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    #[must_use]
    pub fn swi73(&mut self) -> SWI73_W<9> {
        SWI73_W::new(self)
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    #[must_use]
    pub fn swi74(&mut self) -> SWI74_W<10> {
        SWI74_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swier3](index.html) module"]
pub struct EXTI_SWIER3_SPEC;
impl crate::RegisterSpec for EXTI_SWIER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_swier3::R](R) reader structure"]
impl crate::Readable for EXTI_SWIER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_swier3::W](W) writer structure"]
impl crate::Writable for EXTI_SWIER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_SWIER3 to value 0"]
impl crate::Resettable for EXTI_SWIER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
