#[doc = "Register `DDRPHYC_DCR` reader"]
pub struct R(crate::R<DDRPHYC_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DCR` writer"]
pub struct W(crate::W<DDRPHYC_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDRMD` reader - DDRMD"]
pub type DDRMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDRMD` writer - DDRMD"]
pub type DDRMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DDR8BNK` reader - DDR8BNK"]
pub type DDR8BNK_R = crate::BitReader<bool>;
#[doc = "Field `DDR8BNK` writer - DDR8BNK"]
pub type DDR8BNK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DCR_SPEC, bool, O>;
#[doc = "Field `PDQ` reader - PDQ"]
pub type PDQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDQ` writer - PDQ"]
pub type PDQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MPRDQ` reader - MPRDQ"]
pub type MPRDQ_R = crate::BitReader<bool>;
#[doc = "Field `MPRDQ` writer - MPRDQ"]
pub type MPRDQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DCR_SPEC, bool, O>;
#[doc = "Field `DDRTYPE` reader - DDRTYPE"]
pub type DDRTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDRTYPE` writer - DDRTYPE"]
pub type DDRTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NOSRA` reader - NOSRA"]
pub type NOSRA_R = crate::BitReader<bool>;
#[doc = "Field `NOSRA` writer - NOSRA"]
pub type NOSRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DCR_SPEC, bool, O>;
#[doc = "Field `DDR2T` reader - DDR2T"]
pub type DDR2T_R = crate::BitReader<bool>;
#[doc = "Field `DDR2T` writer - DDR2T"]
pub type DDR2T_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DCR_SPEC, bool, O>;
#[doc = "Field `UDIMM` reader - UDIMM"]
pub type UDIMM_R = crate::BitReader<bool>;
#[doc = "Field `UDIMM` writer - UDIMM"]
pub type UDIMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DCR_SPEC, bool, O>;
#[doc = "Field `RDIMM` reader - RDIMM"]
pub type RDIMM_R = crate::BitReader<bool>;
#[doc = "Field `RDIMM` writer - RDIMM"]
pub type RDIMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DCR_SPEC, bool, O>;
#[doc = "Field `TPD` reader - TPD"]
pub type TPD_R = crate::BitReader<bool>;
#[doc = "Field `TPD` writer - TPD"]
pub type TPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    pub fn ddrmd(&self) -> DDRMD_R {
        DDRMD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    pub fn ddr8bnk(&self) -> DDR8BNK_R {
        DDR8BNK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    pub fn pdq(&self) -> PDQ_R {
        PDQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    pub fn mprdq(&self) -> MPRDQ_R {
        MPRDQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    pub fn ddrtype(&self) -> DDRTYPE_R {
        DDRTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    pub fn nosra(&self) -> NOSRA_R {
        NOSRA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    pub fn ddr2t(&self) -> DDR2T_R {
        DDR2T_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    pub fn udimm(&self) -> UDIMM_R {
        UDIMM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    pub fn rdimm(&self) -> RDIMM_R {
        RDIMM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    #[must_use]
    pub fn ddrmd(&mut self) -> DDRMD_W<0> {
        DDRMD_W::new(self)
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    #[must_use]
    pub fn ddr8bnk(&mut self) -> DDR8BNK_W<3> {
        DDR8BNK_W::new(self)
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    #[must_use]
    pub fn pdq(&mut self) -> PDQ_W<4> {
        PDQ_W::new(self)
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    #[must_use]
    pub fn mprdq(&mut self) -> MPRDQ_W<7> {
        MPRDQ_W::new(self)
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn ddrtype(&mut self) -> DDRTYPE_W<8> {
        DDRTYPE_W::new(self)
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    #[must_use]
    pub fn nosra(&mut self) -> NOSRA_W<27> {
        NOSRA_W::new(self)
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2t(&mut self) -> DDR2T_W<28> {
        DDR2T_W::new(self)
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    #[must_use]
    pub fn udimm(&mut self) -> UDIMM_W<29> {
        UDIMM_W::new(self)
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    #[must_use]
    pub fn rdimm(&mut self) -> RDIMM_W<30> {
        RDIMM_W::new(self)
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TPD_W<31> {
        TPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcr](index.html) module"]
pub struct DDRPHYC_DCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DCR to value 0x0b"]
impl crate::Resettable for DDRPHYC_DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
