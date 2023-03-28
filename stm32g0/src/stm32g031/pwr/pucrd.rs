#[doc = "Register `PUCRD` reader"]
pub struct R(crate::R<PUCRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRD` writer"]
pub struct W(crate::W<PUCRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRD_SPEC>;
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
impl From<crate::W<PUCRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU0` reader - Port D pull-up bit y (y=0..15)"]
pub type PU0_R = crate::BitReader<bool>;
#[doc = "Field `PU0` writer - Port D pull-up bit y (y=0..15)"]
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRD_SPEC, bool, O>;
#[doc = "Field `PU1` reader - Port D pull-up bit y (y=0..15)"]
pub type PU1_R = crate::BitReader<bool>;
#[doc = "Field `PU1` writer - Port D pull-up bit y (y=0..15)"]
pub type PU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRD_SPEC, bool, O>;
#[doc = "Field `PU2` reader - Port D pull-up bit y (y=0..15)"]
pub type PU2_R = crate::BitReader<bool>;
#[doc = "Field `PU2` writer - Port D pull-up bit y (y=0..15)"]
pub type PU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRD_SPEC, bool, O>;
#[doc = "Field `PU3` reader - Port D pull-up bit y (y=0..15)"]
pub type PU3_R = crate::BitReader<bool>;
#[doc = "Field `PU3` writer - Port D pull-up bit y (y=0..15)"]
pub type PU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    #[doc = "Bit 1 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    #[doc = "Bit 2 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    #[doc = "Bit 3 - Port D pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port D pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrd](index.html) module"]
pub struct PUCRD_SPEC;
impl crate::RegisterSpec for PUCRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrd::R](R) reader structure"]
impl crate::Readable for PUCRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrd::W](W) writer structure"]
impl crate::Writable for PUCRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUCRD to value 0"]
impl crate::Resettable for PUCRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
