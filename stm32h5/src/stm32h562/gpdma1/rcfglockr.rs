#[doc = "Register `RCFGLOCKR` reader"]
pub struct R(crate::R<RCFGLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCFGLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCFGLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCFGLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCFGLOCKR` writer"]
pub struct W(crate::W<RCFGLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCFGLOCKR_SPEC>;
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
impl From<crate::W<RCFGLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCFGLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK0` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK0_R = crate::BitReader<bool>;
#[doc = "Field `LOCK0` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK1` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK1_R = crate::BitReader<bool>;
#[doc = "Field `LOCK1` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK2` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK2_R = crate::BitReader<bool>;
#[doc = "Field `LOCK2` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK3` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK3_R = crate::BitReader<bool>;
#[doc = "Field `LOCK3` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK4` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK4_R = crate::BitReader<bool>;
#[doc = "Field `LOCK4` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK5` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK5_R = crate::BitReader<bool>;
#[doc = "Field `LOCK5` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK6` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK6_R = crate::BitReader<bool>;
#[doc = "Field `LOCK6` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK7` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK7_R = crate::BitReader<bool>;
#[doc = "Field `LOCK7` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFGLOCKR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<0> {
        LOCK0_W::new(self)
    }
    #[doc = "Bit 1 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<1> {
        LOCK1_W::new(self)
    }
    #[doc = "Bit 2 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<2> {
        LOCK2_W::new(self)
    }
    #[doc = "Bit 3 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<3> {
        LOCK3_W::new(self)
    }
    #[doc = "Bit 4 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock4(&mut self) -> LOCK4_W<4> {
        LOCK4_W::new(self)
    }
    #[doc = "Bit 5 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock5(&mut self) -> LOCK5_W<5> {
        LOCK5_W::new(self)
    }
    #[doc = "Bit 6 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock6(&mut self) -> LOCK6_W<6> {
        LOCK6_W::new(self)
    }
    #[doc = "Bit 7 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock7(&mut self) -> LOCK7_W<7> {
        LOCK7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfglockr](index.html) module"]
pub struct RCFGLOCKR_SPEC;
impl crate::RegisterSpec for RCFGLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcfglockr::R](R) reader structure"]
impl crate::Readable for RCFGLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcfglockr::W](W) writer structure"]
impl crate::Writable for RCFGLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCFGLOCKR to value 0"]
impl crate::Resettable for RCFGLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
