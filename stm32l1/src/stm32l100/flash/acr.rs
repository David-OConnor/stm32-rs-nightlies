#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::BitReader<bool>;
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<bool>;
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `ACC64` reader - 64-bit access"]
pub type ACC64_R = crate::BitReader<bool>;
#[doc = "Field `ACC64` writer - 64-bit access"]
pub type ACC64_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `SLEEP_PD` reader - Flash mode during Sleep"]
pub type SLEEP_PD_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_PD` writer - Flash mode during Sleep"]
pub type SLEEP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `RUN_PD` reader - Flash mode during Run"]
pub type RUN_PD_R = crate::BitReader<bool>;
#[doc = "Field `RUN_PD` writer - Flash mode during Run"]
pub type RUN_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 64-bit access"]
    #[inline(always)]
    pub fn acc64(&self) -> ACC64_R {
        ACC64_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<1> {
        PRFTEN_W::new(self)
    }
    #[doc = "Bit 2 - 64-bit access"]
    #[inline(always)]
    #[must_use]
    pub fn acc64(&mut self) -> ACC64_W<2> {
        ACC64_W::new(self)
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<3> {
        SLEEP_PD_W::new(self)
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<4> {
        RUN_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
