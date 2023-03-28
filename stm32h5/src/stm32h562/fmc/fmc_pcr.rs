#[doc = "Register `FMC_PCR` reader"]
pub struct R(crate::R<FMC_PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PCR` writer"]
pub struct W(crate::W<FMC_PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PCR_SPEC>;
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
impl From<crate::W<FMC_PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWAITEN` reader - Wait feature enable bit Enables the Wait feature for the NAND Flash memory bank:"]
pub type PWAITEN_R = crate::BitReader<bool>;
#[doc = "Field `PWAITEN` writer - Wait feature enable bit Enables the Wait feature for the NAND Flash memory bank:"]
pub type PWAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `PBKEN` reader - NAND Flash memory bank enable bit Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus"]
pub type PBKEN_R = crate::BitReader<bool>;
#[doc = "Field `PBKEN` writer - NAND Flash memory bank enable bit Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus"]
pub type PBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `PTYP` reader - Memory type Defines the type of device attached to the corresponding memory bank:"]
pub type PTYP_R = crate::BitReader<bool>;
#[doc = "Field `PTYP` writer - Memory type Defines the type of device attached to the corresponding memory bank:"]
pub type PTYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `PWID` reader - Data bus width Defines the external memory device width."]
pub type PWID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWID` writer - Data bus width Defines the external memory device width."]
pub type PWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECCEN` reader - ECC computation logic enable bit"]
pub type ECCEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCEN` writer - ECC computation logic enable bit"]
pub type ECCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `TCLR` reader - CLE to RE delay Sets time from CLE low to RE low in number of AHB clock cycles (HCLK). Time is t_clr = (TCLR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
pub type TCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLR` writer - CLE to RE delay Sets time from CLE low to RE low in number of AHB clock cycles (HCLK). Time is t_clr = (TCLR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
pub type TCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TAR` reader - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
pub type TAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAR` writer - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
pub type TAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TAR3` reader - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
pub type TAR3_R = crate::BitReader<bool>;
#[doc = "Field `TAR3` writer - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
pub type TAR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `ECCPS` reader - ECC page size Defines the page size for the extended ECC:"]
pub type ECCPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCPS` writer - ECC page size Defines the page size for the extended ECC:"]
pub type ECCPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 1 - Wait feature enable bit Enables the Wait feature for the NAND Flash memory bank:"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NAND Flash memory bank enable bit Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Memory type Defines the type of device attached to the corresponding memory bank:"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data bus width Defines the external memory device width."]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay Sets time from CLE low to RE low in number of AHB clock cycles (HCLK). Time is t_clr = (TCLR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tar3(&self) -> TAR3_R {
        TAR3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - ECC page size Defines the page size for the extended ECC:"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable bit Enables the Wait feature for the NAND Flash memory bank:"]
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<1> {
        PWAITEN_W::new(self)
    }
    #[doc = "Bit 2 - NAND Flash memory bank enable bit Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB bus"]
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<2> {
        PBKEN_W::new(self)
    }
    #[doc = "Bit 3 - Memory type Defines the type of device attached to the corresponding memory bank:"]
    #[inline(always)]
    #[must_use]
    pub fn ptyp(&mut self) -> PTYP_W<3> {
        PTYP_W::new(self)
    }
    #[doc = "Bits 4:5 - Data bus width Defines the external memory device width."]
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<4> {
        PWID_W::new(self)
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<6> {
        ECCEN_W::new(self)
    }
    #[doc = "Bits 9:12 - CLE to RE delay Sets time from CLE low to RE low in number of AHB clock cycles (HCLK). Time is t_clr = (TCLR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<9> {
        TCLR_W::new(self)
    }
    #[doc = "Bits 13:15 - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<13> {
        TAR_W::new(self)
    }
    #[doc = "Bit 16 - ALE to RE delay Sets time from ALE low to RE low in number of AHB clock cycles (HCLK). Time is: t_ar = (TAR + SET + 2) � THCLK where THCLK is the HCLK clock period Note: SET is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    #[must_use]
    pub fn tar3(&mut self) -> TAR3_W<16> {
        TAR3_W::new(self)
    }
    #[doc = "Bits 17:19 - ECC page size Defines the page size for the extended ECC:"]
    #[inline(always)]
    #[must_use]
    pub fn eccps(&mut self) -> ECCPS_W<17> {
        ECCPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Flash control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcr](index.html) module"]
pub struct FMC_PCR_SPEC;
impl crate::RegisterSpec for FMC_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_pcr::R](R) reader structure"]
impl crate::Readable for FMC_PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_pcr::W](W) writer structure"]
impl crate::Writable for FMC_PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_PCR to value 0x18"]
impl crate::Resettable for FMC_PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
