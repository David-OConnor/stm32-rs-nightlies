#[doc = "Register `ICACHE_IER` reader"]
pub struct R(crate::R<ICACHE_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_IER` writer"]
pub struct W(crate::W<ICACHE_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_IER_SPEC>;
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
impl From<crate::W<ICACHE_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSYENDIE` reader - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
pub type BSYENDIE_R = crate::BitReader<bool>;
#[doc = "Field `BSYENDIE` writer - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
pub type BSYENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_IER_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
    #[inline(always)]
    pub fn bsyendie(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<1> {
        BSYENDIE_W::new(self)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<2> {
        ERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_ier](index.html) module"]
pub struct ICACHE_IER_SPEC;
impl crate::RegisterSpec for ICACHE_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_ier::R](R) reader structure"]
impl crate::Readable for ICACHE_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_ier::W](W) writer structure"]
impl crate::Writable for ICACHE_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_IER to value 0"]
impl crate::Resettable for ICACHE_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
