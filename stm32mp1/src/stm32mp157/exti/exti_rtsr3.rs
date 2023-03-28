#[doc = "Register `EXTI_RTSR3` reader"]
pub struct R(crate::R<EXTI_RTSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_RTSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_RTSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_RTSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_RTSR3` writer"]
pub struct W(crate::W<EXTI_RTSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_RTSR3_SPEC>;
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
impl From<crate::W<EXTI_RTSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_RTSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT65` reader - RT65"]
pub type RT65_R = crate::BitReader<bool>;
#[doc = "Field `RT65` writer - RT65"]
pub type RT65_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR3_SPEC, bool, O>;
#[doc = "Field `RT66` reader - RT66"]
pub type RT66_R = crate::BitReader<bool>;
#[doc = "Field `RT66` writer - RT66"]
pub type RT66_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR3_SPEC, bool, O>;
#[doc = "Field `RT68` reader - RT68"]
pub type RT68_R = crate::BitReader<bool>;
#[doc = "Field `RT68` writer - RT68"]
pub type RT68_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR3_SPEC, bool, O>;
#[doc = "Field `RT73` reader - RT73"]
pub type RT73_R = crate::BitReader<bool>;
#[doc = "Field `RT73` writer - RT73"]
pub type RT73_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR3_SPEC, bool, O>;
#[doc = "Field `RT74` reader - RT74"]
pub type RT74_R = crate::BitReader<bool>;
#[doc = "Field `RT74` writer - RT74"]
pub type RT74_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    pub fn rt65(&self) -> RT65_R {
        RT65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    pub fn rt66(&self) -> RT66_R {
        RT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    pub fn rt68(&self) -> RT68_R {
        RT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    pub fn rt73(&self) -> RT73_R {
        RT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    pub fn rt74(&self) -> RT74_R {
        RT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    #[must_use]
    pub fn rt65(&mut self) -> RT65_W<1> {
        RT65_W::new(self)
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    #[must_use]
    pub fn rt66(&mut self) -> RT66_W<2> {
        RT66_W::new(self)
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    #[must_use]
    pub fn rt68(&mut self) -> RT68_W<4> {
        RT68_W::new(self)
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    #[must_use]
    pub fn rt73(&mut self) -> RT73_W<9> {
        RT73_W::new(self)
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    #[must_use]
    pub fn rt74(&mut self) -> RT74_W<10> {
        RT74_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rtsr3](index.html) module"]
pub struct EXTI_RTSR3_SPEC;
impl crate::RegisterSpec for EXTI_RTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_rtsr3::R](R) reader structure"]
impl crate::Readable for EXTI_RTSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_rtsr3::W](W) writer structure"]
impl crate::Writable for EXTI_RTSR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_RTSR3 to value 0"]
impl crate::Resettable for EXTI_RTSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
