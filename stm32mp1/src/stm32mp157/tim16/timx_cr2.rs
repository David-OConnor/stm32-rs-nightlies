#[doc = "Register `TIMx_CR2` reader"]
pub struct R(crate::R<TIMX_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CR2` writer"]
pub struct W(crate::W<TIMX_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CR2_SPEC>;
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
impl From<crate::W<TIMX_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCPC` reader - CCPC"]
pub type CCPC_R = crate::BitReader<bool>;
#[doc = "Field `CCPC` writer - CCPC"]
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR2_SPEC, bool, O>;
#[doc = "Field `CCUS` reader - CCUS"]
pub type CCUS_R = crate::BitReader<bool>;
#[doc = "Field `CCUS` writer - CCUS"]
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR2_SPEC, bool, O>;
#[doc = "Field `CCDS` reader - CCDS"]
pub type CCDS_R = crate::BitReader<bool>;
#[doc = "Field `CCDS` writer - CCDS"]
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR2_SPEC, bool, O>;
#[doc = "Field `OIS1` reader - OIS1"]
pub type OIS1_R = crate::BitReader<bool>;
#[doc = "Field `OIS1` writer - OIS1"]
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR2_SPEC, bool, O>;
#[doc = "Field `OIS1N` reader - OIS1N"]
pub type OIS1N_R = crate::BitReader<bool>;
#[doc = "Field `OIS1N` writer - OIS1N"]
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr2](index.html) module"]
pub struct TIMX_CR2_SPEC;
impl crate::RegisterSpec for TIMX_CR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timx_cr2::R](R) reader structure"]
impl crate::Readable for TIMX_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_cr2::W](W) writer structure"]
impl crate::Writable for TIMX_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMx_CR2 to value 0"]
impl crate::Resettable for TIMX_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
