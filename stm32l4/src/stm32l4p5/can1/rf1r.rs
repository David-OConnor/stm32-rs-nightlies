#[doc = "Register `RF1R` reader"]
pub struct R(crate::R<RF1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF1R` writer"]
pub struct W(crate::W<RF1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF1R_SPEC>;
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
impl From<crate::W<RF1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMP1` reader - FMP1"]
pub type FMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FULL1` reader - FULL1"]
pub type FULL1_R = crate::BitReader<bool>;
#[doc = "Field `FULL1` writer - FULL1"]
pub type FULL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF1R_SPEC, bool, O>;
#[doc = "Field `FOVR1` reader - FOVR1"]
pub type FOVR1_R = crate::BitReader<bool>;
#[doc = "Field `FOVR1` writer - FOVR1"]
pub type FOVR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF1R_SPEC, bool, O>;
#[doc = "Field `RFOM1` reader - RFOM1"]
pub type RFOM1_R = crate::BitReader<bool>;
#[doc = "Field `RFOM1` writer - RFOM1"]
pub type RFOM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF1R_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FMP1"]
    #[inline(always)]
    pub fn fmp1(&self) -> FMP1_R {
        FMP1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&self) -> FOVR1_R {
        FOVR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&self) -> RFOM1_R {
        RFOM1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    #[must_use]
    pub fn full1(&mut self) -> FULL1_W<3> {
        FULL1_W::new(self)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    #[must_use]
    pub fn fovr1(&mut self) -> FOVR1_W<4> {
        FOVR1_W::new(self)
    }
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    #[must_use]
    pub fn rfom1(&mut self) -> RFOM1_W<5> {
        RFOM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "receive FIFO 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf1r](index.html) module"]
pub struct RF1R_SPEC;
impl crate::RegisterSpec for RF1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf1r::R](R) reader structure"]
impl crate::Readable for RF1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf1r::W](W) writer structure"]
impl crate::Writable for RF1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RF1R to value 0"]
impl crate::Resettable for RF1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
