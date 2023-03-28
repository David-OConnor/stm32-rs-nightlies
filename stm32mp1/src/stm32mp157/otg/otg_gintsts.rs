#[doc = "Register `OTG_GINTSTS` reader"]
pub struct R(crate::R<OTG_GINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GINTSTS` writer"]
pub struct W(crate::W<OTG_GINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GINTSTS_SPEC>;
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
impl From<crate::W<OTG_GINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMOD` reader - CMOD"]
pub type CMOD_R = crate::BitReader<bool>;
#[doc = "Field `MMIS` reader - MMIS"]
pub type MMIS_R = crate::BitReader<bool>;
#[doc = "Field `MMIS` writer - MMIS"]
pub type MMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `OTGINT` reader - OTGINT"]
pub type OTGINT_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - SOF"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - SOF"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `RXFLVL` reader - RXFLVL"]
pub type RXFLVL_R = crate::BitReader<bool>;
#[doc = "Field `NPTXFE` reader - NPTXFE"]
pub type NPTXFE_R = crate::BitReader<bool>;
#[doc = "Field `GINAKEFF` reader - GINAKEFF"]
pub type GINAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `GONAKEFF` reader - GONAKEFF"]
pub type GONAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `ESUSP` reader - ESUSP"]
pub type ESUSP_R = crate::BitReader<bool>;
#[doc = "Field `ESUSP` writer - ESUSP"]
pub type ESUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `USBSUSP` reader - USBSUSP"]
pub type USBSUSP_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSP` writer - USBSUSP"]
pub type USBSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `USBRST` reader - USBRST"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - USBRST"]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `ENUMDNE` reader - ENUMDNE"]
pub type ENUMDNE_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDNE` writer - ENUMDNE"]
pub type ENUMDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `ISOODRP` reader - ISOODRP"]
pub type ISOODRP_R = crate::BitReader<bool>;
#[doc = "Field `ISOODRP` writer - ISOODRP"]
pub type ISOODRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `EOPF` reader - EOPF"]
pub type EOPF_R = crate::BitReader<bool>;
#[doc = "Field `EOPF` writer - EOPF"]
pub type EOPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `IEPINT` reader - IEPINT"]
pub type IEPINT_R = crate::BitReader<bool>;
#[doc = "Field `OEPINT` reader - OEPINT"]
pub type OEPINT_R = crate::BitReader<bool>;
#[doc = "Field `IISOIXFR` reader - IISOIXFR"]
pub type IISOIXFR_R = crate::BitReader<bool>;
#[doc = "Field `IISOIXFR` writer - IISOIXFR"]
pub type IISOIXFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `IPXFR` reader - IPXFR"]
pub type IPXFR_R = crate::BitReader<bool>;
#[doc = "Field `IPXFR` writer - IPXFR"]
pub type IPXFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `DATAFSUSP` reader - DATAFSUSP"]
pub type DATAFSUSP_R = crate::BitReader<bool>;
#[doc = "Field `DATAFSUSP` writer - DATAFSUSP"]
pub type DATAFSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `HPRTINT` reader - HPRTINT"]
pub type HPRTINT_R = crate::BitReader<bool>;
#[doc = "Field `HCINT` reader - HCINT"]
pub type HCINT_R = crate::BitReader<bool>;
#[doc = "Field `PTXFE` reader - PTXFE"]
pub type PTXFE_R = crate::BitReader<bool>;
#[doc = "Field `CIDSCHG` reader - CIDSCHG"]
pub type CIDSCHG_R = crate::BitReader<bool>;
#[doc = "Field `CIDSCHG` writer - CIDSCHG"]
pub type CIDSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `DISCINT` reader - DISCINT"]
pub type DISCINT_R = crate::BitReader<bool>;
#[doc = "Field `DISCINT` writer - DISCINT"]
pub type DISCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `SRQINT` reader - SRQINT"]
pub type SRQINT_R = crate::BitReader<bool>;
#[doc = "Field `SRQINT` writer - SRQINT"]
pub type SRQINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
#[doc = "Field `WKUPINT` reader - WKUPINT"]
pub type WKUPINT_R = crate::BitReader<bool>;
#[doc = "Field `WKUPINT` writer - WKUPINT"]
pub type WKUPINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GINTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMOD"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFLVL"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NPTXFE"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GINAKEFF"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GONAKEFF"]
    #[inline(always)]
    pub fn gonakeff(&self) -> GONAKEFF_R {
        GONAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    pub fn ipxfr(&self) -> IPXFR_R {
        IPXFR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - HPRTINT"]
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HCINT"]
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PTXFE"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    #[must_use]
    pub fn mmis(&mut self) -> MMIS_W<1> {
        MMIS_W::new(self)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<3> {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    #[must_use]
    pub fn esusp(&mut self) -> ESUSP_W<10> {
        ESUSP_W::new(self)
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> USBSUSP_W<11> {
        USBSUSP_W::new(self)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<12> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    #[must_use]
    pub fn enumdne(&mut self) -> ENUMDNE_W<13> {
        ENUMDNE_W::new(self)
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrp(&mut self) -> ISOODRP_W<14> {
        ISOODRP_W::new(self)
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EOPF_W<15> {
        EOPF_W::new(self)
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W<20> {
        IISOIXFR_W::new(self)
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    #[must_use]
    pub fn ipxfr(&mut self) -> IPXFR_W<21> {
        IPXFR_W::new(self)
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W<22> {
        DATAFSUSP_W::new(self)
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    #[must_use]
    pub fn cidschg(&mut self) -> CIDSCHG_W<28> {
        CIDSCHG_W::new(self)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<29> {
        DISCINT_W::new(self)
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    #[must_use]
    pub fn srqint(&mut self) -> SRQINT_W<30> {
        SRQINT_W::new(self)
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    #[must_use]
    pub fn wkupint(&mut self) -> WKUPINT_W<31> {
        WKUPINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gintsts](index.html) module"]
pub struct OTG_GINTSTS_SPEC;
impl crate::RegisterSpec for OTG_GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_gintsts::R](R) reader structure"]
impl crate::Readable for OTG_GINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_gintsts::W](W) writer structure"]
impl crate::Writable for OTG_GINTSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_GINTSTS to value 0x1400_0020"]
impl crate::Resettable for OTG_GINTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1400_0020;
}
