#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_EOP` reader - Bank 1 EOP1 flag clear bit"]
pub type CLR_EOP_R = crate::BitReader<bool>;
#[doc = "Field `CLR_EOP` writer - Bank 1 EOP1 flag clear bit"]
pub type CLR_EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_WRPERR` reader - Bank 1 WRPERR1 flag clear bit"]
pub type CLR_WRPERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_WRPERR` writer - Bank 1 WRPERR1 flag clear bit"]
pub type CLR_WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_PGSERR` reader - Bank 1 PGSERR1 flag clear bi"]
pub type CLR_PGSERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_PGSERR` writer - Bank 1 PGSERR1 flag clear bi"]
pub type CLR_PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_STRBERR` reader - Bank 1 STRBERR1 flag clear bit"]
pub type CLR_STRBERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_STRBERR` writer - Bank 1 STRBERR1 flag clear bit"]
pub type CLR_STRBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_INCERR` reader - Bank 1 INCERR1 flag clear bit"]
pub type CLR_INCERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_INCERR` writer - Bank 1 INCERR1 flag clear bit"]
pub type CLR_INCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_OPERR` reader - Bank 1 OPERR1 flag clear bit"]
pub type CLR_OPERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_OPERR` writer - Bank 1 OPERR1 flag clear bit"]
pub type CLR_OPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_RDPERR` reader - Bank 1 RDPERR1 flag clear bit"]
pub type CLR_RDPERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_RDPERR` writer - Bank 1 RDPERR1 flag clear bit"]
pub type CLR_RDPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_RDSERR` reader - Bank 1 RDSERR1 flag clear bit"]
pub type CLR_RDSERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_RDSERR` writer - Bank 1 RDSERR1 flag clear bit"]
pub type CLR_RDSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_SNECCERR` reader - Bank 1 SNECCERR1 flag clear bit"]
pub type CLR_SNECCERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_SNECCERR` writer - Bank 1 SNECCERR1 flag clear bit"]
pub type CLR_SNECCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_DBECCERR` reader - Bank 1 DBECCERR1 flag clear bit"]
pub type CLR_DBECCERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_DBECCERR` writer - Bank 1 DBECCERR1 flag clear bit"]
pub type CLR_DBECCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CLR_CRCEND` reader - Bank 1 CRCEND1 flag clear bit"]
pub type CLR_CRCEND_R = crate::BitReader<bool>;
#[doc = "Field `CLR_CRCEND` writer - Bank 1 CRCEND1 flag clear bit"]
pub type CLR_CRCEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop(&self) -> CLR_EOP_R {
        CLR_EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr(&self) -> CLR_WRPERR_R {
        CLR_WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bi"]
    #[inline(always)]
    pub fn clr_pgserr(&self) -> CLR_PGSERR_R {
        CLR_PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr(&self) -> CLR_STRBERR_R {
        CLR_STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr(&self) -> CLR_INCERR_R {
        CLR_INCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr(&self) -> CLR_OPERR_R {
        CLR_OPERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr(&self) -> CLR_RDPERR_R {
        CLR_RDPERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr(&self) -> CLR_RDSERR_R {
        CLR_RDSERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr(&self) -> CLR_SNECCERR_R {
        CLR_SNECCERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr(&self) -> CLR_DBECCERR_R {
        CLR_DBECCERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend(&self) -> CLR_CRCEND_R {
        CLR_CRCEND_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_eop(&mut self) -> CLR_EOP_W<16> {
        CLR_EOP_W::new(self)
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_wrperr(&mut self) -> CLR_WRPERR_W<17> {
        CLR_WRPERR_W::new(self)
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bi"]
    #[inline(always)]
    #[must_use]
    pub fn clr_pgserr(&mut self) -> CLR_PGSERR_W<18> {
        CLR_PGSERR_W::new(self)
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_strberr(&mut self) -> CLR_STRBERR_W<19> {
        CLR_STRBERR_W::new(self)
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_incerr(&mut self) -> CLR_INCERR_W<21> {
        CLR_INCERR_W::new(self)
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_operr(&mut self) -> CLR_OPERR_W<22> {
        CLR_OPERR_W::new(self)
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rdperr(&mut self) -> CLR_RDPERR_W<23> {
        CLR_RDPERR_W::new(self)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rdserr(&mut self) -> CLR_RDSERR_W<24> {
        CLR_RDSERR_W::new(self)
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_sneccerr(&mut self) -> CLR_SNECCERR_W<25> {
        CLR_SNECCERR_W::new(self)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_dbeccerr(&mut self) -> CLR_DBECCERR_W<26> {
        CLR_DBECCERR_W::new(self)
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_crcend(&mut self) -> CLR_CRCEND_W<27> {
        CLR_CRCEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH clear control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
