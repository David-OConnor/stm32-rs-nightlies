#[doc = "Register `EXTI_FTSR3` reader"]
pub struct R(crate::R<EXTI_FTSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_FTSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_FTSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_FTSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_FTSR3` writer"]
pub struct W(crate::W<EXTI_FTSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_FTSR3_SPEC>;
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
impl From<crate::W<EXTI_FTSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_FTSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FT65` reader - FT65"]
pub type FT65_R = crate::BitReader<bool>;
#[doc = "Field `FT65` writer - FT65"]
pub type FT65_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FTSR3_SPEC, bool, O>;
#[doc = "Field `FT66` reader - FT66"]
pub type FT66_R = crate::BitReader<bool>;
#[doc = "Field `FT66` writer - FT66"]
pub type FT66_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FTSR3_SPEC, bool, O>;
#[doc = "Field `FT68` reader - FT68"]
pub type FT68_R = crate::BitReader<bool>;
#[doc = "Field `FT68` writer - FT68"]
pub type FT68_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FTSR3_SPEC, bool, O>;
#[doc = "Field `FT73` reader - FT73"]
pub type FT73_R = crate::BitReader<bool>;
#[doc = "Field `FT73` writer - FT73"]
pub type FT73_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FTSR3_SPEC, bool, O>;
#[doc = "Field `FT74` reader - FT74"]
pub type FT74_R = crate::BitReader<bool>;
#[doc = "Field `FT74` writer - FT74"]
pub type FT74_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FTSR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    pub fn ft65(&self) -> FT65_R {
        FT65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    pub fn ft66(&self) -> FT66_R {
        FT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    pub fn ft68(&self) -> FT68_R {
        FT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    pub fn ft73(&self) -> FT73_R {
        FT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    pub fn ft74(&self) -> FT74_R {
        FT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    #[must_use]
    pub fn ft65(&mut self) -> FT65_W<1> {
        FT65_W::new(self)
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    #[must_use]
    pub fn ft66(&mut self) -> FT66_W<2> {
        FT66_W::new(self)
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    #[must_use]
    pub fn ft68(&mut self) -> FT68_W<4> {
        FT68_W::new(self)
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    #[must_use]
    pub fn ft73(&mut self) -> FT73_W<9> {
        FT73_W::new(self)
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    #[must_use]
    pub fn ft74(&mut self) -> FT74_W<10> {
        FT74_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ftsr3](index.html) module"]
pub struct EXTI_FTSR3_SPEC;
impl crate::RegisterSpec for EXTI_FTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_ftsr3::R](R) reader structure"]
impl crate::Readable for EXTI_FTSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_ftsr3::W](W) writer structure"]
impl crate::Writable for EXTI_FTSR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_FTSR3 to value 0"]
impl crate::Resettable for EXTI_FTSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
