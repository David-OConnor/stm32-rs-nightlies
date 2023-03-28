#[doc = "Register `TIM6_CCMR2ALTERNATE22` reader"]
pub struct R(crate::R<TIM6_CCMR2ALTERNATE22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_CCMR2ALTERNATE22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_CCMR2ALTERNATE22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_CCMR2ALTERNATE22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM6_CCMR2ALTERNATE22` writer"]
pub struct W(crate::W<TIM6_CCMR2ALTERNATE22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_CCMR2ALTERNATE22_SPEC>;
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
impl From<crate::W<TIM6_CCMR2ALTERNATE22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_CCMR2ALTERNATE22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC3S` reader - CC3S"]
pub type CC3S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC3S` writer - CC3S"]
pub type CC3S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM6_CCMR2ALTERNATE22_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC3PSC` reader - IC3PSC"]
pub type IC3PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC3PSC` writer - IC3PSC"]
pub type IC3PSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM6_CCMR2ALTERNATE22_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC3F` reader - IC3F"]
pub type IC3F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC3F` writer - IC3F"]
pub type IC3F_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM6_CCMR2ALTERNATE22_SPEC, u8, u8, 4, O>;
#[doc = "Field `CC4S` reader - CC4S"]
pub type CC4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC4S` writer - CC4S"]
pub type CC4S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM6_CCMR2ALTERNATE22_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC4PSC` reader - IC4PSC"]
pub type IC4PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC4PSC` writer - IC4PSC"]
pub type IC4PSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM6_CCMR2ALTERNATE22_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC4F` reader - IC4F"]
pub type IC4F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC4F` writer - IC4F"]
pub type IC4F_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIM6_CCMR2ALTERNATE22_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IC3PSC"]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - IC3F"]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - IC4PSC"]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - IC4F"]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<0> {
        CC3S_W::new(self)
    }
    #[doc = "Bits 2:3 - IC3PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> IC3PSC_W<2> {
        IC3PSC_W::new(self)
    }
    #[doc = "Bits 4:7 - IC3F"]
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> IC3F_W<4> {
        IC3F_W::new(self)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<8> {
        CC4S_W::new(self)
    }
    #[doc = "Bits 10:11 - IC4PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> IC4PSC_W<10> {
        IC4PSC_W::new(self)
    }
    #[doc = "Bits 12:15 - IC4F"]
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> IC4F_W<12> {
        IC4F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccmr2alternate22](index.html) module"]
pub struct TIM6_CCMR2ALTERNATE22_SPEC;
impl crate::RegisterSpec for TIM6_CCMR2ALTERNATE22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim6_ccmr2alternate22::R](R) reader structure"]
impl crate::Readable for TIM6_CCMR2ALTERNATE22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim6_ccmr2alternate22::W](W) writer structure"]
impl crate::Writable for TIM6_CCMR2ALTERNATE22_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM6_CCMR2ALTERNATE22 to value 0"]
impl crate::Resettable for TIM6_CCMR2ALTERNATE22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
