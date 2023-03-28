#[doc = "Register `MACPFR` reader"]
pub struct R(crate::R<MACPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPFR` writer"]
pub struct W(crate::W<MACPFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPFR_SPEC>;
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
impl From<crate::W<MACPFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - Promiscuous Mode"]
pub type PR_R = crate::BitReader<bool>;
#[doc = "Field `PR` writer - Promiscuous Mode"]
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `HUC` reader - Hash Unicast"]
pub type HUC_R = crate::BitReader<bool>;
#[doc = "Field `HUC` writer - Hash Unicast"]
pub type HUC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `HMC` reader - Hash Multicast"]
pub type HMC_R = crate::BitReader<bool>;
#[doc = "Field `HMC` writer - Hash Multicast"]
pub type HMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering"]
pub type DAIF_R = crate::BitReader<bool>;
#[doc = "Field `DAIF` writer - DA Inverse Filtering"]
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `PM` reader - Pass All Multicast"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Pass All Multicast"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `DBF` reader - Disable Broadcast Packets"]
pub type DBF_R = crate::BitReader<bool>;
#[doc = "Field `DBF` writer - Disable Broadcast Packets"]
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `PCF` reader - Pass Control Packets"]
pub type PCF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCF` writer - Pass Control Packets"]
pub type PCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACPFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAIF` reader - SA Inverse Filtering"]
pub type SAIF_R = crate::BitReader<bool>;
#[doc = "Field `SAIF` writer - SA Inverse Filtering"]
pub type SAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `SAF` reader - Source Address Filter Enable"]
pub type SAF_R = crate::BitReader<bool>;
#[doc = "Field `SAF` writer - Source Address Filter Enable"]
pub type SAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter"]
pub type HPF_R = crate::BitReader<bool>;
#[doc = "Field `HPF` writer - Hash or Perfect Filter"]
pub type HPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `VTFE` reader - VLAN Tag Filter Enable"]
pub type VTFE_R = crate::BitReader<bool>;
#[doc = "Field `VTFE` writer - VLAN Tag Filter Enable"]
pub type VTFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `IPFE` reader - Layer 3 and Layer 4 Filter Enable"]
pub type IPFE_R = crate::BitReader<bool>;
#[doc = "Field `IPFE` writer - Layer 3 and Layer 4 Filter Enable"]
pub type IPFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `DNTU` reader - Drop Non-TCP/UDP over IP Packets"]
pub type DNTU_R = crate::BitReader<bool>;
#[doc = "Field `DNTU` writer - Drop Non-TCP/UDP over IP Packets"]
pub type DNTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
#[doc = "Field `RA` reader - Receive All"]
pub type RA_R = crate::BitReader<bool>;
#[doc = "Field `RA` writer - Receive All"]
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Packets"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Packets"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable"]
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drop Non-TCP/UDP over IP Packets"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HUC_W<1> {
        HUC_W::new(self)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HMC_W<2> {
        HMC_W::new(self)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<4> {
        PM_W::new(self)
    }
    #[doc = "Bit 5 - Disable Broadcast Packets"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<5> {
        DBF_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass Control Packets"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<8> {
        SAIF_W::new(self)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<9> {
        SAF_W::new(self)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<10> {
        HPF_W::new(self)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VTFE_W<16> {
        VTFE_W::new(self)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipfe(&mut self) -> IPFE_W<20> {
        IPFE_W::new(self)
    }
    #[doc = "Bit 21 - Drop Non-TCP/UDP over IP Packets"]
    #[inline(always)]
    #[must_use]
    pub fn dntu(&mut self) -> DNTU_W<21> {
        DNTU_W::new(self)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<31> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet filtering control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpfr](index.html) module"]
pub struct MACPFR_SPEC;
impl crate::RegisterSpec for MACPFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macpfr::R](R) reader structure"]
impl crate::Readable for MACPFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macpfr::W](W) writer structure"]
impl crate::Writable for MACPFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPFR to value 0"]
impl crate::Resettable for MACPFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
