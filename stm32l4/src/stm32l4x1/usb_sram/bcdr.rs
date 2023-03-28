#[doc = "Register `BCDR` reader"]
pub struct R(crate::R<BCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDR` writer"]
pub struct W(crate::W<BCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDR_SPEC>;
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
impl From<crate::W<BCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDEN` reader - Battery charging detector"]
pub type BCDEN_R = crate::BitReader<bool>;
#[doc = "Field `BCDEN` writer - Battery charging detector"]
pub type BCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, bool, O>;
#[doc = "Field `DCDEN` reader - Data contact detection"]
pub type DCDEN_R = crate::BitReader<bool>;
#[doc = "Field `DCDEN` writer - Data contact detection"]
pub type DCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, bool, O>;
#[doc = "Field `PDEN` reader - Primary detection"]
pub type PDEN_R = crate::BitReader<bool>;
#[doc = "Field `PDEN` writer - Primary detection"]
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, bool, O>;
#[doc = "Field `SDEN` reader - Secondary detection"]
pub type SDEN_R = crate::BitReader<bool>;
#[doc = "Field `SDEN` writer - Secondary detection"]
pub type SDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, bool, O>;
#[doc = "Field `DCDET` reader - Data contact detection"]
pub type DCDET_R = crate::BitReader<bool>;
#[doc = "Field `PDET` reader - Primary detection"]
pub type PDET_R = crate::BitReader<bool>;
#[doc = "Field `SDET` reader - Secondary detection"]
pub type SDET_R = crate::BitReader<bool>;
#[doc = "Field `PS2DET` reader - DM pull-up detection status"]
pub type PS2DET_R = crate::BitReader<bool>;
#[doc = "Field `DPPU` reader - DP pull-up control"]
pub type DPPU_R = crate::BitReader<bool>;
#[doc = "Field `DPPU` writer - DP pull-up control"]
pub type DPPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Battery charging detector"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data contact detection"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Primary detection"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secondary detection"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data contact detection"]
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Primary detection"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secondary detection"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery charging detector"]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<0> {
        BCDEN_W::new(self)
    }
    #[doc = "Bit 1 - Data contact detection"]
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<1> {
        DCDEN_W::new(self)
    }
    #[doc = "Bit 2 - Primary detection"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<2> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 3 - Secondary detection"]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<3> {
        SDEN_W::new(self)
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    #[must_use]
    pub fn dppu(&mut self) -> DPPU_W<15> {
        DPPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery charging detector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdr](index.html) module"]
pub struct BCDR_SPEC;
impl crate::RegisterSpec for BCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdr::R](R) reader structure"]
impl crate::Readable for BCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdr::W](W) writer structure"]
impl crate::Writable for BCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCDR to value 0"]
impl crate::Resettable for BCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
