#[doc = "Register `LPTIM_IER` reader"]
pub struct R(crate::R<LPTIM_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_IER` writer"]
pub struct W(crate::W<LPTIM_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_IER_SPEC>;
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
impl From<crate::W<LPTIM_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPMIE` reader - CMPMIE"]
pub type CMPMIE_R = crate::BitReader<bool>;
#[doc = "Field `CMPMIE` writer - CMPMIE"]
pub type CMPMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_IER_SPEC, bool, O>;
#[doc = "Field `ARRMIE` reader - ARRMIE"]
pub type ARRMIE_R = crate::BitReader<bool>;
#[doc = "Field `ARRMIE` writer - ARRMIE"]
pub type ARRMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_IER_SPEC, bool, O>;
#[doc = "Field `EXTTRIGIE` reader - EXTTRIGIE"]
pub type EXTTRIGIE_R = crate::BitReader<bool>;
#[doc = "Field `EXTTRIGIE` writer - EXTTRIGIE"]
pub type EXTTRIGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_IER_SPEC, bool, O>;
#[doc = "Field `CMPOKIE` reader - CMPOKIE"]
pub type CMPOKIE_R = crate::BitReader<bool>;
#[doc = "Field `CMPOKIE` writer - CMPOKIE"]
pub type CMPOKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_IER_SPEC, bool, O>;
#[doc = "Field `ARROKIE` reader - ARROKIE"]
pub type ARROKIE_R = crate::BitReader<bool>;
#[doc = "Field `ARROKIE` writer - ARROKIE"]
pub type ARROKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_IER_SPEC, bool, O>;
#[doc = "Field `UPIE` reader - UPIE"]
pub type UPIE_R = crate::BitReader<bool>;
#[doc = "Field `UPIE` writer - UPIE"]
pub type UPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_IER_SPEC, bool, O>;
#[doc = "Field `DOWNIE` reader - DOWNIE"]
pub type DOWNIE_R = crate::BitReader<bool>;
#[doc = "Field `DOWNIE` writer - DOWNIE"]
pub type DOWNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMPMIE"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ARRMIE"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTTRIGIE"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMPOKIE"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARROKIE"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UPIE"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOWNIE"]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPMIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmie(&mut self) -> CMPMIE_W<0> {
        CMPMIE_W::new(self)
    }
    #[doc = "Bit 1 - ARRMIE"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<1> {
        ARRMIE_W::new(self)
    }
    #[doc = "Bit 2 - EXTTRIGIE"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<2> {
        EXTTRIGIE_W::new(self)
    }
    #[doc = "Bit 3 - CMPOKIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<3> {
        CMPOKIE_W::new(self)
    }
    #[doc = "Bit 4 - ARROKIE"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<4> {
        ARROKIE_W::new(self)
    }
    #[doc = "Bit 5 - UPIE"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<5> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 6 - DOWNIE"]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<6> {
        DOWNIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_ier](index.html) module"]
pub struct LPTIM_IER_SPEC;
impl crate::RegisterSpec for LPTIM_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_ier::R](R) reader structure"]
impl crate::Readable for LPTIM_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_ier::W](W) writer structure"]
impl crate::Writable for LPTIM_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_IER to value 0"]
impl crate::Resettable for LPTIM_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
