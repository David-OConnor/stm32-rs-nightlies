#[doc = "Register `DDRPHYC_PGCR` reader"]
pub struct R(crate::R<DDRPHYC_PGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_PGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_PGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_PGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_PGCR` writer"]
pub struct W(crate::W<DDRPHYC_PGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_PGCR_SPEC>;
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
impl From<crate::W<DDRPHYC_PGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_PGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITMDMD` reader - ITMDMD"]
pub type ITMDMD_R = crate::BitReader<bool>;
#[doc = "Field `ITMDMD` writer - ITMDMD"]
pub type ITMDMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `DQSCFG` reader - DQSCFG"]
pub type DQSCFG_R = crate::BitReader<bool>;
#[doc = "Field `DQSCFG` writer - DQSCFG"]
pub type DQSCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `DFTCMP` reader - DFTCMP"]
pub type DFTCMP_R = crate::BitReader<bool>;
#[doc = "Field `DFTCMP` writer - DFTCMP"]
pub type DFTCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `DFTLMT` reader - DFTLMT"]
pub type DFTLMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFTLMT` writer - DFTLMT"]
pub type DFTLMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTOSEL` reader - DTOSEL"]
pub type DTOSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTOSEL` writer - DTOSEL"]
pub type DTOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CKEN` reader - CKEN"]
pub type CKEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKEN` writer - CKEN"]
pub type CKEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CKDV` reader - CKDV"]
pub type CKDV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDV` writer - CKDV"]
pub type CKDV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKINV` reader - CKINV"]
pub type CKINV_R = crate::BitReader<bool>;
#[doc = "Field `CKINV` writer - CKINV"]
pub type CKINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `IOLB` reader - IOLB"]
pub type IOLB_R = crate::BitReader<bool>;
#[doc = "Field `IOLB` writer - IOLB"]
pub type IOLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `IODDRM` reader - IODDRM"]
pub type IODDRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IODDRM` writer - IODDRM"]
pub type IODDRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RANKEN` reader - RANKEN"]
pub type RANKEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANKEN` writer - RANKEN"]
pub type RANKEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ZKSEL` reader - ZKSEL"]
pub type ZKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ZKSEL` writer - ZKSEL"]
pub type ZKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PDDISDX` reader - PDDISDX"]
pub type PDDISDX_R = crate::BitReader<bool>;
#[doc = "Field `PDDISDX` writer - PDDISDX"]
pub type PDDISDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `RFSHDT` reader - RFSHDT"]
pub type RFSHDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFSHDT` writer - RFSHDT"]
pub type RFSHDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PGCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `LBDQSS` reader - LBDQSS"]
pub type LBDQSS_R = crate::BitReader<bool>;
#[doc = "Field `LBDQSS` writer - LBDQSS"]
pub type LBDQSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `LBGDQS` reader - LBGDQS"]
pub type LBGDQS_R = crate::BitReader<bool>;
#[doc = "Field `LBGDQS` writer - LBGDQS"]
pub type LBGDQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
#[doc = "Field `LBMODE` reader - LBMODE"]
pub type LBMODE_R = crate::BitReader<bool>;
#[doc = "Field `LBMODE` writer - LBMODE"]
pub type LBMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PGCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    pub fn itmdmd(&self) -> ITMDMD_R {
        ITMDMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    pub fn dqscfg(&self) -> DQSCFG_R {
        DQSCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    pub fn dftcmp(&self) -> DFTCMP_R {
        DFTCMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    pub fn dftlmt(&self) -> DFTLMT_R {
        DFTLMT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    pub fn dtosel(&self) -> DTOSEL_R {
        DTOSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    pub fn ckinv(&self) -> CKINV_R {
        CKINV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    pub fn iolb(&self) -> IOLB_R {
        IOLB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    pub fn ioddrm(&self) -> IODDRM_R {
        IODDRM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    pub fn ranken(&self) -> RANKEN_R {
        RANKEN_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    pub fn zksel(&self) -> ZKSEL_R {
        ZKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    pub fn pddisdx(&self) -> PDDISDX_R {
        PDDISDX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    pub fn rfshdt(&self) -> RFSHDT_R {
        RFSHDT_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    pub fn lbdqss(&self) -> LBDQSS_R {
        LBDQSS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    pub fn lbgdqs(&self) -> LBGDQS_R {
        LBGDQS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    pub fn lbmode(&self) -> LBMODE_R {
        LBMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    #[must_use]
    pub fn itmdmd(&mut self) -> ITMDMD_W<0> {
        ITMDMD_W::new(self)
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn dqscfg(&mut self) -> DQSCFG_W<1> {
        DQSCFG_W::new(self)
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    #[must_use]
    pub fn dftcmp(&mut self) -> DFTCMP_W<2> {
        DFTCMP_W::new(self)
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    #[must_use]
    pub fn dftlmt(&mut self) -> DFTLMT_W<3> {
        DFTLMT_W::new(self)
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    #[must_use]
    pub fn dtosel(&mut self) -> DTOSEL_W<5> {
        DTOSEL_W::new(self)
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CKEN_W<9> {
        CKEN_W::new(self)
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    #[must_use]
    pub fn ckdv(&mut self) -> CKDV_W<12> {
        CKDV_W::new(self)
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    #[must_use]
    pub fn ckinv(&mut self) -> CKINV_W<14> {
        CKINV_W::new(self)
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    #[must_use]
    pub fn iolb(&mut self) -> IOLB_W<15> {
        IOLB_W::new(self)
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    #[must_use]
    pub fn ioddrm(&mut self) -> IODDRM_W<16> {
        IODDRM_W::new(self)
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    #[must_use]
    pub fn ranken(&mut self) -> RANKEN_W<18> {
        RANKEN_W::new(self)
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn zksel(&mut self) -> ZKSEL_W<22> {
        ZKSEL_W::new(self)
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    #[must_use]
    pub fn pddisdx(&mut self) -> PDDISDX_W<24> {
        PDDISDX_W::new(self)
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    #[must_use]
    pub fn rfshdt(&mut self) -> RFSHDT_W<25> {
        RFSHDT_W::new(self)
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    #[must_use]
    pub fn lbdqss(&mut self) -> LBDQSS_W<29> {
        LBDQSS_W::new(self)
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    #[must_use]
    pub fn lbgdqs(&mut self) -> LBGDQS_W<30> {
        LBGDQS_W::new(self)
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    #[must_use]
    pub fn lbmode(&mut self) -> LBMODE_W<31> {
        LBMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC PHY global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pgcr](index.html) module"]
pub struct DDRPHYC_PGCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_PGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_pgcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_PGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_pgcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_PGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PGCR to value 0x01bc_2e04"]
impl crate::Resettable for DDRPHYC_PGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01bc_2e04;
}
