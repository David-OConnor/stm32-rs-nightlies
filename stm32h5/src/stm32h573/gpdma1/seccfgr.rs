#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC0_R = crate::BitReader<bool>;
#[doc = "Field `SEC0` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC1` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC1_R = crate::BitReader<bool>;
#[doc = "Field `SEC1` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC2` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC2_R = crate::BitReader<bool>;
#[doc = "Field `SEC2` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC3` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC3_R = crate::BitReader<bool>;
#[doc = "Field `SEC3` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC4` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC4_R = crate::BitReader<bool>;
#[doc = "Field `SEC4` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC5` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC5_R = crate::BitReader<bool>;
#[doc = "Field `SEC5` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC6` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC6_R = crate::BitReader<bool>;
#[doc = "Field `SEC6` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC7` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC7_R = crate::BitReader<bool>;
#[doc = "Field `SEC7` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<0> {
        SEC0_W::new(self)
    }
    #[doc = "Bit 1 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<1> {
        SEC1_W::new(self)
    }
    #[doc = "Bit 2 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<2> {
        SEC2_W::new(self)
    }
    #[doc = "Bit 3 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<3> {
        SEC3_W::new(self)
    }
    #[doc = "Bit 4 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<4> {
        SEC4_W::new(self)
    }
    #[doc = "Bit 5 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<5> {
        SEC5_W::new(self)
    }
    #[doc = "Bit 6 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<6> {
        SEC6_W::new(self)
    }
    #[doc = "Bit 7 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<7> {
        SEC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
