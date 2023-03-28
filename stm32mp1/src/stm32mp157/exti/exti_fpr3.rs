#[doc = "Register `EXTI_FPR3` reader"]
pub struct R(crate::R<EXTI_FPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_FPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_FPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_FPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_FPR3` writer"]
pub struct W(crate::W<EXTI_FPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_FPR3_SPEC>;
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
impl From<crate::W<EXTI_FPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_FPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIF65` reader - FPIF65"]
pub type FPIF65_R = crate::BitReader<bool>;
#[doc = "Field `FPIF65` writer - FPIF65"]
pub type FPIF65_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FPR3_SPEC, bool, O>;
#[doc = "Field `FPIF66` reader - FPIF66"]
pub type FPIF66_R = crate::BitReader<bool>;
#[doc = "Field `FPIF66` writer - FPIF66"]
pub type FPIF66_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FPR3_SPEC, bool, O>;
#[doc = "Field `FPIF68` reader - FPIF68"]
pub type FPIF68_R = crate::BitReader<bool>;
#[doc = "Field `FPIF68` writer - FPIF68"]
pub type FPIF68_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FPR3_SPEC, bool, O>;
#[doc = "Field `FPIF73` reader - FPIF73"]
pub type FPIF73_R = crate::BitReader<bool>;
#[doc = "Field `FPIF73` writer - FPIF73"]
pub type FPIF73_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FPR3_SPEC, bool, O>;
#[doc = "Field `FPIF74` reader - FPIF74"]
pub type FPIF74_R = crate::BitReader<bool>;
#[doc = "Field `FPIF74` writer - FPIF74"]
pub type FPIF74_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_FPR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    pub fn fpif65(&self) -> FPIF65_R {
        FPIF65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    pub fn fpif66(&self) -> FPIF66_R {
        FPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    pub fn fpif68(&self) -> FPIF68_R {
        FPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    pub fn fpif73(&self) -> FPIF73_R {
        FPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    pub fn fpif74(&self) -> FPIF74_R {
        FPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    #[must_use]
    pub fn fpif65(&mut self) -> FPIF65_W<1> {
        FPIF65_W::new(self)
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    #[must_use]
    pub fn fpif66(&mut self) -> FPIF66_W<2> {
        FPIF66_W::new(self)
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    #[must_use]
    pub fn fpif68(&mut self) -> FPIF68_W<4> {
        FPIF68_W::new(self)
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    #[must_use]
    pub fn fpif73(&mut self) -> FPIF73_W<9> {
        FPIF73_W::new(self)
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    #[must_use]
    pub fn fpif74(&mut self) -> FPIF74_W<10> {
        FPIF74_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_fpr3](index.html) module"]
pub struct EXTI_FPR3_SPEC;
impl crate::RegisterSpec for EXTI_FPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_fpr3::R](R) reader structure"]
impl crate::Readable for EXTI_FPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_fpr3::W](W) writer structure"]
impl crate::Writable for EXTI_FPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_FPR3 to value 0"]
impl crate::Resettable for EXTI_FPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
