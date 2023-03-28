#[doc = "Register `DCACHE_IER` reader"]
pub struct R(crate::R<DCACHE_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_IER` writer"]
pub struct W(crate::W<DCACHE_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_IER_SPEC>;
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
impl From<crate::W<DCACHE_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSYENDIE` reader - interrupt enable on busy end Set by SW to enable an interrupt generation at the end of a cache full invalidate operation."]
pub type BSYENDIE_R = crate::BitReader<bool>;
#[doc = "Field `BSYENDIE` writer - interrupt enable on busy end Set by SW to enable an interrupt generation at the end of a cache full invalidate operation."]
pub type BSYENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_IER_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (eviction or clean operation write-back error)"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (eviction or clean operation write-back error)"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_IER_SPEC, bool, O>;
#[doc = "Field `CMDENDIE` reader - interrupt enable on command end Set by software to enable an interrupt generation at the end of a cache command (clean and/or invalidate an address range)"]
pub type CMDENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDENDIE` writer - interrupt enable on command end Set by software to enable an interrupt generation at the end of a cache command (clean and/or invalidate an address range)"]
pub type CMDENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - interrupt enable on busy end Set by SW to enable an interrupt generation at the end of a cache full invalidate operation."]
    #[inline(always)]
    pub fn bsyendie(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (eviction or clean operation write-back error)"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt enable on command end Set by software to enable an interrupt generation at the end of a cache command (clean and/or invalidate an address range)"]
    #[inline(always)]
    pub fn cmdendie(&self) -> CMDENDIE_R {
        CMDENDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - interrupt enable on busy end Set by SW to enable an interrupt generation at the end of a cache full invalidate operation."]
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<1> {
        BSYENDIE_W::new(self)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (eviction or clean operation write-back error)"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<2> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - interrupt enable on command end Set by software to enable an interrupt generation at the end of a cache command (clean and/or invalidate an address range)"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendie(&mut self) -> CMDENDIE_W<4> {
        CMDENDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCACHE interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_ier](index.html) module"]
pub struct DCACHE_IER_SPEC;
impl crate::RegisterSpec for DCACHE_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_ier::R](R) reader structure"]
impl crate::Readable for DCACHE_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_ier::W](W) writer structure"]
impl crate::Writable for DCACHE_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_IER to value 0"]
impl crate::Resettable for DCACHE_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
