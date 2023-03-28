#[doc = "Register `DDRPHYC_DLLGCR` reader"]
pub struct R(crate::R<DDRPHYC_DLLGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DLLGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DLLGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DLLGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DLLGCR` writer"]
pub struct W(crate::W<DDRPHYC_DLLGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DLLGCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DLLGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DLLGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRES` reader - DRES"]
pub type DRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRES` writer - DRES"]
pub type DRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IPUMP` reader - IPUMP"]
pub type IPUMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPUMP` writer - IPUMP"]
pub type IPUMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TESTEN` reader - TESTEN"]
pub type TESTEN_R = crate::BitReader<bool>;
#[doc = "Field `TESTEN` writer - TESTEN"]
pub type TESTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, bool, O>;
#[doc = "Field `DTC` reader - DTC"]
pub type DTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTC` writer - DTC"]
pub type DTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATC` reader - ATC"]
pub type ATC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATC` writer - ATC"]
pub type ATC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TESTSW` reader - TESTSW"]
pub type TESTSW_R = crate::BitReader<bool>;
#[doc = "Field `TESTSW` writer - TESTSW"]
pub type TESTSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, bool, O>;
#[doc = "Field `MBIAS` reader - MBIAS"]
pub type MBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBIAS` writer - MBIAS"]
pub type MBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SBIAS2_0` reader - SBIAS2_0"]
pub type SBIAS2_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBIAS2_0` writer - SBIAS2_0"]
pub type SBIAS2_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BPS200` reader - BPS200"]
pub type BPS200_R = crate::BitReader<bool>;
#[doc = "Field `BPS200` writer - BPS200"]
pub type BPS200_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, bool, O>;
#[doc = "Field `SBIAS5_3` reader - SBIAS5_3"]
pub type SBIAS5_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBIAS5_3` writer - SBIAS5_3"]
pub type SBIAS5_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FDTRMSL` reader - FDTRMSL"]
pub type FDTRMSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDTRMSL` writer - FDTRMSL"]
pub type FDTRMSL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOCKDET` reader - LOCKDET"]
pub type LOCKDET_R = crate::BitReader<bool>;
#[doc = "Field `LOCKDET` writer - LOCKDET"]
pub type LOCKDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, bool, O>;
#[doc = "Field `DLLRSVD2` reader - DLLRSVD2"]
pub type DLLRSVD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLLRSVD2` writer - DLLRSVD2"]
pub type DLLRSVD2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DLLGCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    pub fn ipump(&self) -> IPUMP_R {
        IPUMP_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    pub fn atc(&self) -> ATC_R {
        ATC_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    pub fn testsw(&self) -> TESTSW_R {
        TESTSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    pub fn mbias(&self) -> MBIAS_R {
        MBIAS_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    pub fn sbias2_0(&self) -> SBIAS2_0_R {
        SBIAS2_0_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    pub fn bps200(&self) -> BPS200_R {
        BPS200_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    pub fn sbias5_3(&self) -> SBIAS5_3_R {
        SBIAS5_3_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    pub fn fdtrmsl(&self) -> FDTRMSL_R {
        FDTRMSL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    pub fn lockdet(&self) -> LOCKDET_R {
        LOCKDET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    pub fn dllrsvd2(&self) -> DLLRSVD2_R {
        DLLRSVD2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DRES_W<0> {
        DRES_W::new(self)
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    #[must_use]
    pub fn ipump(&mut self) -> IPUMP_W<2> {
        IPUMP_W::new(self)
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    #[must_use]
    pub fn testen(&mut self) -> TESTEN_W<5> {
        TESTEN_W::new(self)
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    #[must_use]
    pub fn dtc(&mut self) -> DTC_W<6> {
        DTC_W::new(self)
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    #[must_use]
    pub fn atc(&mut self) -> ATC_W<9> {
        ATC_W::new(self)
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    #[must_use]
    pub fn testsw(&mut self) -> TESTSW_W<11> {
        TESTSW_W::new(self)
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    #[must_use]
    pub fn mbias(&mut self) -> MBIAS_W<12> {
        MBIAS_W::new(self)
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    #[must_use]
    pub fn sbias2_0(&mut self) -> SBIAS2_0_W<20> {
        SBIAS2_0_W::new(self)
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    #[must_use]
    pub fn bps200(&mut self) -> BPS200_W<23> {
        BPS200_W::new(self)
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    #[must_use]
    pub fn sbias5_3(&mut self) -> SBIAS5_3_W<24> {
        SBIAS5_3_W::new(self)
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    #[must_use]
    pub fn fdtrmsl(&mut self) -> FDTRMSL_W<27> {
        FDTRMSL_W::new(self)
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    #[must_use]
    pub fn lockdet(&mut self) -> LOCKDET_W<29> {
        LOCKDET_W::new(self)
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    #[must_use]
    pub fn dllrsvd2(&mut self) -> DLLRSVD2_W<30> {
        DLLRSVD2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DDR global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dllgcr](index.html) module"]
pub struct DDRPHYC_DLLGCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DLLGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dllgcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DLLGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dllgcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DLLGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DLLGCR to value 0x0373_7000"]
impl crate::Resettable for DDRPHYC_DLLGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0373_7000;
}
