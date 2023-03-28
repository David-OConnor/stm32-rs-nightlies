#[doc = "Register `PSSI_CR` reader"]
pub struct R(crate::R<PSSI_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSI_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSI_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSI_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSSI_CR` writer"]
pub struct W(crate::W<PSSI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSSI_CR_SPEC>;
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
impl From<crate::W<PSSI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSSI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKPOL` reader - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN."]
pub type CKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CKPOL` writer - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN."]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSSI_CR_SPEC, bool, O>;
#[doc = "Field `DEPOL` reader - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface."]
pub type DEPOL_R = crate::BitReader<bool>;
#[doc = "Field `DEPOL` writer - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface."]
pub type DEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSSI_CR_SPEC, bool, O>;
#[doc = "Field `RDYPOL` reader - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface."]
pub type RDYPOL_R = crate::BitReader<bool>;
#[doc = "Field `RDYPOL` writer - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface."]
pub type RDYPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSSI_CR_SPEC, bool, O>;
#[doc = "Field `EDM` reader - Extended data mode"]
pub type EDM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDM` writer - Extended data mode"]
pub type EDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSSI_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENABLE` reader - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSSI_CR_SPEC, bool, O>;
#[doc = "Field `DERDYCFG` reader - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity."]
pub type DERDYCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DERDYCFG` writer - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity."]
pub type DERDYCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSSI_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSSI_CR_SPEC, bool, O>;
#[doc = "Field `OUTEN` reader - Data direction selection bit"]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - Data direction selection bit"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSSI_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN."]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    pub fn rdypol(&self) -> RDYPOL_R {
        RDYPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity."]
    #[inline(always)]
    pub fn derdycfg(&self) -> DERDYCFG_R {
        DERDYCFG_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data direction selection bit"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<5> {
        CKPOL_W::new(self)
    }
    #[doc = "Bit 6 - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<6> {
        DEPOL_W::new(self)
    }
    #[doc = "Bit 8 - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    #[must_use]
    pub fn rdypol(&mut self) -> RDYPOL_W<8> {
        RDYPOL_W::new(self)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<10> {
        EDM_W::new(self)
    }
    #[doc = "Bit 14 - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<14> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 18:20 - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity."]
    #[inline(always)]
    #[must_use]
    pub fn derdycfg(&mut self) -> DERDYCFG_W<18> {
        DERDYCFG_W::new(self)
    }
    #[doc = "Bit 30 - DMA enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<30> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 31 - Data direction selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<31> {
        OUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PSSI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi_cr](index.html) module"]
pub struct PSSI_CR_SPEC;
impl crate::RegisterSpec for PSSI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssi_cr::R](R) reader structure"]
impl crate::Readable for PSSI_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pssi_cr::W](W) writer structure"]
impl crate::Writable for PSSI_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSSI_CR to value 0x4000_0000"]
impl crate::Resettable for PSSI_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
