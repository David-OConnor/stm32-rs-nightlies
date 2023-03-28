#[doc = "Register `SPI_CFG2` reader"]
pub struct R(crate::R<SPI_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CFG2` writer"]
pub struct W(crate::W<SPI_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CFG2_SPEC>;
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
impl From<crate::W<SPI_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSSI` reader - MSSI"]
pub type MSSI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSSI` writer - MSSI"]
pub type MSSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `MIDI` reader - MIDI"]
pub type MIDI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIDI` writer - MIDI"]
pub type MIDI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `IOSWP` reader - IOSWP"]
pub type IOSWP_R = crate::BitReader<bool>;
#[doc = "Field `IOSWP` writer - IOSWP"]
pub type IOSWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `COMM` reader - COMM"]
pub type COMM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMM` writer - COMM"]
pub type COMM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `SP` reader - SP"]
pub type SP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SP` writer - SP"]
pub type SP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `MASTER` reader - MASTER"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - MASTER"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `LSBFRST` reader - LSBFRST"]
pub type LSBFRST_R = crate::BitReader<bool>;
#[doc = "Field `LSBFRST` writer - LSBFRST"]
pub type LSBFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `CPHA` reader - CPHA"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - CPHA"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - CPOL"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - CPOL"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `SSM` reader - SSM"]
pub type SSM_R = crate::BitReader<bool>;
#[doc = "Field `SSM` writer - SSM"]
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `SSIOP` reader - SSIOP"]
pub type SSIOP_R = crate::BitReader<bool>;
#[doc = "Field `SSIOP` writer - SSIOP"]
pub type SSIOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `SSOE` reader - SSOE"]
pub type SSOE_R = crate::BitReader<bool>;
#[doc = "Field `SSOE` writer - SSOE"]
pub type SSOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `SSOM` reader - SSOM"]
pub type SSOM_R = crate::BitReader<bool>;
#[doc = "Field `SSOM` writer - SSOM"]
pub type SSOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
#[doc = "Field `AFCNTR` reader - AFCNTR"]
pub type AFCNTR_R = crate::BitReader<bool>;
#[doc = "Field `AFCNTR` writer - AFCNTR"]
pub type AFCNTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - MSSI"]
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MIDI"]
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - IOSWP"]
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:18 - COMM"]
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - SP"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - MASTER"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LSBFRST"]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SSM"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SSIOP"]
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SSOE"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SSOM"]
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AFCNTR"]
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MSSI"]
    #[inline(always)]
    #[must_use]
    pub fn mssi(&mut self) -> MSSI_W<0> {
        MSSI_W::new(self)
    }
    #[doc = "Bits 4:7 - MIDI"]
    #[inline(always)]
    #[must_use]
    pub fn midi(&mut self) -> MIDI_W<4> {
        MIDI_W::new(self)
    }
    #[doc = "Bit 15 - IOSWP"]
    #[inline(always)]
    #[must_use]
    pub fn ioswp(&mut self) -> IOSWP_W<15> {
        IOSWP_W::new(self)
    }
    #[doc = "Bits 17:18 - COMM"]
    #[inline(always)]
    #[must_use]
    pub fn comm(&mut self) -> COMM_W<17> {
        COMM_W::new(self)
    }
    #[doc = "Bits 19:21 - SP"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<19> {
        SP_W::new(self)
    }
    #[doc = "Bit 22 - MASTER"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<22> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 23 - LSBFRST"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<23> {
        LSBFRST_W::new(self)
    }
    #[doc = "Bit 24 - CPHA"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<24> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 25 - CPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<25> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 26 - SSM"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<26> {
        SSM_W::new(self)
    }
    #[doc = "Bit 28 - SSIOP"]
    #[inline(always)]
    #[must_use]
    pub fn ssiop(&mut self) -> SSIOP_W<28> {
        SSIOP_W::new(self)
    }
    #[doc = "Bit 29 - SSOE"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<29> {
        SSOE_W::new(self)
    }
    #[doc = "Bit 30 - SSOM"]
    #[inline(always)]
    #[must_use]
    pub fn ssom(&mut self) -> SSOM_W<30> {
        SSOM_W::new(self)
    }
    #[doc = "Bit 31 - AFCNTR"]
    #[inline(always)]
    #[must_use]
    pub fn afcntr(&mut self) -> AFCNTR_W<31> {
        AFCNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cfg2](index.html) module"]
pub struct SPI_CFG2_SPEC;
impl crate::RegisterSpec for SPI_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cfg2::R](R) reader structure"]
impl crate::Readable for SPI_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cfg2::W](W) writer structure"]
impl crate::Writable for SPI_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CFG2 to value 0"]
impl crate::Resettable for SPI_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
