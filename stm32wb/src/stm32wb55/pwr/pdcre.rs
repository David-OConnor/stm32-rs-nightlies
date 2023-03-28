#[doc = "Register `PDCRE` reader"]
pub struct R(crate::R<PDCRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRE` writer"]
pub struct W(crate::W<PDCRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRE_SPEC>;
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
impl From<crate::W<PDCRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD0` reader - Port E pull-down bit y (y=0..15)"]
pub type PD0_R = crate::BitReader<bool>;
#[doc = "Field `PD0` writer - Port E pull-down bit y (y=0..15)"]
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRE_SPEC, bool, O>;
#[doc = "Field `PD1` reader - Port E pull-down bit y (y=0..15)"]
pub type PD1_R = crate::BitReader<bool>;
#[doc = "Field `PD1` writer - Port E pull-down bit y (y=0..15)"]
pub type PD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRE_SPEC, bool, O>;
#[doc = "Field `PD2` reader - Port E pull-down bit y (y=0..15)"]
pub type PD2_R = crate::BitReader<bool>;
#[doc = "Field `PD2` writer - Port E pull-down bit y (y=0..15)"]
pub type PD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRE_SPEC, bool, O>;
#[doc = "Field `PD3` reader - Port E pull-down bit y (y=0..15)"]
pub type PD3_R = crate::BitReader<bool>;
#[doc = "Field `PD3` writer - Port E pull-down bit y (y=0..15)"]
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRE_SPEC, bool, O>;
#[doc = "Field `PD4` reader - Port E pull-down bit y (y=0..15)"]
pub type PD4_R = crate::BitReader<bool>;
#[doc = "Field `PD4` writer - Port E pull-down bit y (y=0..15)"]
pub type PD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    #[doc = "Bit 1 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    #[doc = "Bit 2 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    #[doc = "Bit 3 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    #[doc = "Bit 4 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port E pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcre](index.html) module"]
pub struct PDCRE_SPEC;
impl crate::RegisterSpec for PDCRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcre::R](R) reader structure"]
impl crate::Readable for PDCRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcre::W](W) writer structure"]
impl crate::Writable for PDCRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDCRE to value 0"]
impl crate::Resettable for PDCRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
