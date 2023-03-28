#[doc = "Register `DCACHE_CMDREADDRR` reader"]
pub struct R(crate::R<DCACHE_CMDREADDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CMDREADDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CMDREADDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CMDREADDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_CMDREADDRR` writer"]
pub struct W(crate::W<DCACHE_CMDREADDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CMDREADDRR_SPEC>;
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
impl From<crate::W<DCACHE_CMDREADDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CMDREADDRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDENDADDR` reader - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
pub type CMDENDADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDENDADDR` writer - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
pub type CMDENDADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCACHE_CMDREADDRR_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 4:31 - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
    #[inline(always)]
    pub fn cmdendaddr(&self) -> CMDENDADDR_R {
        CMDENDADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - end address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written."]
    #[inline(always)]
    #[must_use]
    pub fn cmdendaddr(&mut self) -> CMDENDADDR_W<4> {
        CMDENDADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCACHE command range end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_cmdreaddrr](index.html) module"]
pub struct DCACHE_CMDREADDRR_SPEC;
impl crate::RegisterSpec for DCACHE_CMDREADDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_cmdreaddrr::R](R) reader structure"]
impl crate::Readable for DCACHE_CMDREADDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_cmdreaddrr::W](W) writer structure"]
impl crate::Writable for DCACHE_CMDREADDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_CMDREADDRR to value 0"]
impl crate::Resettable for DCACHE_CMDREADDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
