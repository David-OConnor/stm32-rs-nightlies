#[doc = "Register `CSLOCKR` reader"]
pub struct R(crate::R<CSLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSLOCKR` writer"]
pub struct W(crate::W<CSLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSLOCKR_SPEC>;
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
impl From<crate::W<CSLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSVTAIRCR` reader - LOCKSVTAIRCR"]
pub type LOCKSVTAIRCR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSVTAIRCR` writer - LOCKSVTAIRCR"]
pub type LOCKSVTAIRCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCKSMPU` reader - LOCKSMPU"]
pub type LOCKSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSMPU` writer - LOCKSMPU"]
pub type LOCKSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCKSAU` reader - LOCKSAU"]
pub type LOCKSAU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSAU` writer - LOCKSAU"]
pub type LOCKSAU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSLOCKR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<0> {
        LOCKSVTAIRCR_W::new(self)
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<1> {
        LOCKSMPU_W::new(self)
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LOCKSAU_W<2> {
        LOCKSAU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG CPU secure lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cslockr](index.html) module"]
pub struct CSLOCKR_SPEC;
impl crate::RegisterSpec for CSLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cslockr::R](R) reader structure"]
impl crate::Readable for CSLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cslockr::W](W) writer structure"]
impl crate::Writable for CSLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSLOCKR to value 0"]
impl crate::Resettable for CSLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
