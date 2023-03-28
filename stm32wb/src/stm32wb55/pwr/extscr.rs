#[doc = "Register `EXTSCR` reader"]
pub struct R(crate::R<EXTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTSCR` writer"]
pub struct W(crate::W<EXTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTSCR_SPEC>;
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
impl From<crate::W<EXTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C1CSSF` writer - Clear CPU1 Stop Standby flags"]
pub type C1CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTSCR_SPEC, bool, O>;
#[doc = "Field `C2CSSF` writer - Clear CPU2 Stop Standby flags"]
pub type C2CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTSCR_SPEC, bool, O>;
#[doc = "Field `CCRPF` writer - Clear Critical Radio system phase"]
pub type CCRPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTSCR_SPEC, bool, O>;
#[doc = "Field `C1SBF` reader - System Standby flag for CPU1"]
pub type C1SBF_R = crate::BitReader<bool>;
#[doc = "Field `C1STOPF` reader - System Stop flag for CPU1"]
pub type C1STOPF_R = crate::BitReader<bool>;
#[doc = "Field `C2SBF` reader - System Standby flag for CPU2"]
pub type C2SBF_R = crate::BitReader<bool>;
#[doc = "Field `C2STOPF` reader - System Stop flag for CPU2"]
pub type C2STOPF_R = crate::BitReader<bool>;
#[doc = "Field `CRPF` reader - Critical Radio system phase"]
pub type CRPF_R = crate::BitReader<bool>;
#[doc = "Field `C1DS` reader - CPU1 deepsleep mode"]
pub type C1DS_R = crate::BitReader<bool>;
#[doc = "Field `C2DS` reader - CPU2 deepsleep mode"]
pub type C2DS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - System Standby flag for CPU1"]
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - System Stop flag for CPU1"]
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - System Standby flag for CPU2"]
    #[inline(always)]
    pub fn c2sbf(&self) -> C2SBF_R {
        C2SBF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - System Stop flag for CPU2"]
    #[inline(always)]
    pub fn c2stopf(&self) -> C2STOPF_R {
        C2STOPF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Critical Radio system phase"]
    #[inline(always)]
    pub fn crpf(&self) -> CRPF_R {
        CRPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 deepsleep mode"]
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU2 deepsleep mode"]
    #[inline(always)]
    pub fn c2ds(&self) -> C2DS_R {
        C2DS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear CPU1 Stop Standby flags"]
    #[inline(always)]
    #[must_use]
    pub fn c1cssf(&mut self) -> C1CSSF_W<0> {
        C1CSSF_W::new(self)
    }
    #[doc = "Bit 1 - Clear CPU2 Stop Standby flags"]
    #[inline(always)]
    #[must_use]
    pub fn c2cssf(&mut self) -> C2CSSF_W<1> {
        C2CSSF_W::new(self)
    }
    #[doc = "Bit 2 - Clear Critical Radio system phase"]
    #[inline(always)]
    #[must_use]
    pub fn ccrpf(&mut self) -> CCRPF_W<2> {
        CCRPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extscr](index.html) module"]
pub struct EXTSCR_SPEC;
impl crate::RegisterSpec for EXTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extscr::R](R) reader structure"]
impl crate::Readable for EXTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extscr::W](W) writer structure"]
impl crate::Writable for EXTSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTSCR to value 0"]
impl crate::Resettable for EXTSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
