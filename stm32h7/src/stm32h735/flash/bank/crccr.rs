#[doc = "Register `CRCCR` reader"]
pub struct R(crate::R<CRCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCR` writer"]
pub struct W(crate::W<CRCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCR_SPEC>;
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
impl From<crate::W<CRCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_SECT` reader - Bank 1 CRC sector number"]
pub type CRC_SECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC_SECT` writer - Bank 1 CRC sector number"]
pub type CRC_SECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ALL_BANK` reader - Bank 1 CRC select bit"]
pub type ALL_BANK_R = crate::BitReader<bool>;
#[doc = "Field `ALL_BANK` writer - Bank 1 CRC select bit"]
pub type ALL_BANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `CRC_BY_SECT` reader - Bank 1 CRC sector mode select bit"]
pub type CRC_BY_SECT_R = crate::BitReader<bool>;
#[doc = "Field `CRC_BY_SECT` writer - Bank 1 CRC sector mode select bit"]
pub type CRC_BY_SECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `ADD_SECT` reader - Bank 1 CRC sector select bit"]
pub type ADD_SECT_R = crate::BitReader<bool>;
#[doc = "Field `ADD_SECT` writer - Bank 1 CRC sector select bit"]
pub type ADD_SECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `CLEAN_SECT` reader - Bank 1 CRC sector list clear bit"]
pub type CLEAN_SECT_R = crate::BitReader<bool>;
#[doc = "Field `CLEAN_SECT` writer - Bank 1 CRC sector list clear bit"]
pub type CLEAN_SECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `START_CRC` reader - Bank 1 CRC start bit"]
pub type START_CRC_R = crate::BitReader<bool>;
#[doc = "Field `START_CRC` writer - Bank 1 CRC start bit"]
pub type START_CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `CLEAN_CRC` reader - Bank 1 CRC clear bit"]
pub type CLEAN_CRC_R = crate::BitReader<bool>;
#[doc = "Field `CLEAN_CRC` writer - Bank 1 CRC clear bit"]
pub type CLEAN_CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCCR_SPEC, bool, O>;
#[doc = "Field `CRC_BURST` reader - Bank 1 CRC burst size"]
pub type CRC_BURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC_BURST` writer - Bank 1 CRC burst size"]
pub type CRC_BURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - Bank 1 CRC sector number"]
    #[inline(always)]
    pub fn crc_sect(&self) -> CRC_SECT_R {
        CRC_SECT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Bank 1 CRC select bit"]
    #[inline(always)]
    pub fn all_bank(&self) -> ALL_BANK_R {
        ALL_BANK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bank 1 CRC sector mode select bit"]
    #[inline(always)]
    pub fn crc_by_sect(&self) -> CRC_BY_SECT_R {
        CRC_BY_SECT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bank 1 CRC sector select bit"]
    #[inline(always)]
    pub fn add_sect(&self) -> ADD_SECT_R {
        ADD_SECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bank 1 CRC sector list clear bit"]
    #[inline(always)]
    pub fn clean_sect(&self) -> CLEAN_SECT_R {
        CLEAN_SECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 CRC start bit"]
    #[inline(always)]
    pub fn start_crc(&self) -> START_CRC_R {
        START_CRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 CRC clear bit"]
    #[inline(always)]
    pub fn clean_crc(&self) -> CLEAN_CRC_R {
        CLEAN_CRC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Bank 1 CRC burst size"]
    #[inline(always)]
    pub fn crc_burst(&self) -> CRC_BURST_R {
        CRC_BURST_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bank 1 CRC sector number"]
    #[inline(always)]
    #[must_use]
    pub fn crc_sect(&mut self) -> CRC_SECT_W<0> {
        CRC_SECT_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 CRC select bit"]
    #[inline(always)]
    #[must_use]
    pub fn all_bank(&mut self) -> ALL_BANK_W<7> {
        ALL_BANK_W::new(self)
    }
    #[doc = "Bit 8 - Bank 1 CRC sector mode select bit"]
    #[inline(always)]
    #[must_use]
    pub fn crc_by_sect(&mut self) -> CRC_BY_SECT_W<8> {
        CRC_BY_SECT_W::new(self)
    }
    #[doc = "Bit 9 - Bank 1 CRC sector select bit"]
    #[inline(always)]
    #[must_use]
    pub fn add_sect(&mut self) -> ADD_SECT_W<9> {
        ADD_SECT_W::new(self)
    }
    #[doc = "Bit 10 - Bank 1 CRC sector list clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clean_sect(&mut self) -> CLEAN_SECT_W<10> {
        CLEAN_SECT_W::new(self)
    }
    #[doc = "Bit 16 - Bank 1 CRC start bit"]
    #[inline(always)]
    #[must_use]
    pub fn start_crc(&mut self) -> START_CRC_W<16> {
        START_CRC_W::new(self)
    }
    #[doc = "Bit 17 - Bank 1 CRC clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clean_crc(&mut self) -> CLEAN_CRC_W<17> {
        CLEAN_CRC_W::new(self)
    }
    #[doc = "Bits 20:21 - Bank 1 CRC burst size"]
    #[inline(always)]
    #[must_use]
    pub fn crc_burst(&mut self) -> CRC_BURST_W<20> {
        CRC_BURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH CRC control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccr](index.html) module"]
pub struct CRCCR_SPEC;
impl crate::RegisterSpec for CRCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crccr::R](R) reader structure"]
impl crate::Readable for CRCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crccr::W](W) writer structure"]
impl crate::Writable for CRCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCR to value 0"]
impl crate::Resettable for CRCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
