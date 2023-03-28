#[doc = "Register `TIM5_CCMR1ALTERNATE5` reader"]
pub struct R(crate::R<TIM5_CCMR1ALTERNATE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_CCMR1ALTERNATE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_CCMR1ALTERNATE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_CCMR1ALTERNATE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_CCMR1ALTERNATE5` writer"]
pub struct W(crate::W<TIM5_CCMR1ALTERNATE5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_CCMR1ALTERNATE5_SPEC>;
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
impl From<crate::W<TIM5_CCMR1ALTERNATE5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_CCMR1ALTERNATE5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - CC1S"]
pub type CC1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1S` writer - CC1S"]
pub type CC1S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1ALTERNATE5_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1PSC` reader - IC1PSC"]
pub type IC1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1PSC` writer - IC1PSC"]
pub type IC1PSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1ALTERNATE5_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1F` reader - IC1F"]
pub type IC1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1F` writer - IC1F"]
pub type IC1F_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1ALTERNATE5_SPEC, u8, u8, 4, O>;
#[doc = "Field `CC2S` reader - CC2S"]
pub type CC2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2S` writer - CC2S"]
pub type CC2S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1ALTERNATE5_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC2PSC` reader - IC2PSC"]
pub type IC2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2PSC` writer - IC2PSC"]
pub type IC2PSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1ALTERNATE5_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC2F` reader - IC2F"]
pub type IC2F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2F` writer - IC2F"]
pub type IC2F_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM5_CCMR1ALTERNATE5_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IC1PSC"]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - IC1F"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - IC2PSC"]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - IC2F"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    #[doc = "Bits 2:3 - IC1PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<2> {
        IC1PSC_W::new(self)
    }
    #[doc = "Bits 4:7 - IC1F"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<4> {
        IC1F_W::new(self)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    #[doc = "Bits 10:11 - IC2PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<10> {
        IC2PSC_W::new(self)
    }
    #[doc = "Bits 12:15 - IC2F"]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<12> {
        IC2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccmr1alternate5](index.html) module"]
pub struct TIM5_CCMR1ALTERNATE5_SPEC;
impl crate::RegisterSpec for TIM5_CCMR1ALTERNATE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim5_ccmr1alternate5::R](R) reader structure"]
impl crate::Readable for TIM5_CCMR1ALTERNATE5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_ccmr1alternate5::W](W) writer structure"]
impl crate::Writable for TIM5_CCMR1ALTERNATE5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM5_CCMR1ALTERNATE5 to value 0"]
impl crate::Resettable for TIM5_CCMR1ALTERNATE5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
