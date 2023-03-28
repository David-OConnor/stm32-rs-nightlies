#[doc = "Register `MACCR` reader"]
pub struct R(crate::R<MACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACCR` writer"]
pub struct W(crate::W<MACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACCR_SPEC>;
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
impl From<crate::W<MACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver Enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit Packets"]
pub type PRELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit Packets"]
pub type PRELEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DC` reader - Deferral Check"]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `DC` writer - Deferral Check"]
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `BL` reader - Back-Off Limit"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - Back-Off Limit"]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DR` reader - Disable Retry"]
pub type DR_R = crate::BitReader<bool>;
#[doc = "Field `DR` writer - Disable Retry"]
pub type DR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission"]
pub type DCRS_R = crate::BitReader<bool>;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission"]
pub type DCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `DO` reader - Disable Receive Own"]
pub type DO_R = crate::BitReader<bool>;
#[doc = "Field `DO` writer - Disable Receive Own"]
pub type DO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `ECRSFD` reader - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
pub type ECRSFD_R = crate::BitReader<bool>;
#[doc = "Field `ECRSFD` writer - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
pub type ECRSFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DM_R = crate::BitReader<bool>;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `FES` reader - MAC Speed"]
pub type FES_R = crate::BitReader<bool>;
#[doc = "Field `FES` writer - MAC Speed"]
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `JE` reader - Jumbo Packet Enable"]
pub type JE_R = crate::BitReader<bool>;
#[doc = "Field `JE` writer - Jumbo Packet Enable"]
pub type JE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JD_R = crate::BitReader<bool>;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `WD` reader - Watchdog Disable"]
pub type WD_R = crate::BitReader<bool>;
#[doc = "Field `WD` writer - Watchdog Disable"]
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping"]
pub type ACS_R = crate::BitReader<bool>;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping"]
pub type ACS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `CST` reader - CRC stripping for Type packets"]
pub type CST_R = crate::BitReader<bool>;
#[doc = "Field `CST` writer - CRC stripping for Type packets"]
pub type CST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `S2KP` reader - IEEE 802.3as Support for 2K Packets"]
pub type S2KP_R = crate::BitReader<bool>;
#[doc = "Field `S2KP` writer - IEEE 802.3as Support for 2K Packets"]
pub type S2KP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `GPSLCE` reader - Giant Packet Size Limit Control Enable"]
pub type GPSLCE_R = crate::BitReader<bool>;
#[doc = "Field `GPSLCE` writer - Giant Packet Size Limit Control Enable"]
pub type GPSLCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `IPG` reader - Inter-Packet Gap"]
pub type IPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPG` writer - Inter-Packet Gap"]
pub type IPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IPC` reader - Checksum Offload"]
pub type IPC_R = crate::BitReader<bool>;
#[doc = "Field `IPC` writer - Checksum Offload"]
pub type IPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control"]
pub type SARC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SARC` writer - Source Address Insertion or Replacement Control"]
pub type SARC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ARPEN` reader - ARP Offload Enable"]
pub type ARPEN_R = crate::BitReader<bool>;
#[doc = "Field `ARPEN` writer - ARP Offload Enable"]
pub type ARPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit Packets"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MAC Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Jumbo Packet Enable"]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IEEE 802.3as Support for 2K Packets"]
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable"]
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap"]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - ARP Offload Enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<0> {
        RE_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<1> {
        TE_W::new(self)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit Packets"]
    #[inline(always)]
    #[must_use]
    pub fn prelen(&mut self) -> PRELEN_W<2> {
        PRELEN_W::new(self)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    #[doc = "Bit 8 - Disable Retry"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<8> {
        DR_W::new(self)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DCRS_W<9> {
        DCRS_W::new(self)
    }
    #[doc = "Bit 10 - Disable Receive Own"]
    #[inline(always)]
    #[must_use]
    pub fn do_(&mut self) -> DO_W<10> {
        DO_W::new(self)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ecrsfd(&mut self) -> ECRSFD_W<11> {
        ECRSFD_W::new(self)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<13> {
        DM_W::new(self)
    }
    #[doc = "Bit 14 - MAC Speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    #[doc = "Bit 16 - Jumbo Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn je(&mut self) -> JE_W<16> {
        JE_W::new(self)
    }
    #[doc = "Bit 17 - Jabber Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<17> {
        JD_W::new(self)
    }
    #[doc = "Bit 19 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<19> {
        WD_W::new(self)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> ACS_W<20> {
        ACS_W::new(self)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<21> {
        CST_W::new(self)
    }
    #[doc = "Bit 22 - IEEE 802.3as Support for 2K Packets"]
    #[inline(always)]
    #[must_use]
    pub fn s2kp(&mut self) -> S2KP_W<22> {
        S2KP_W::new(self)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpslce(&mut self) -> GPSLCE_W<23> {
        GPSLCE_W::new(self)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IPG_W<24> {
        IPG_W::new(self)
    }
    #[doc = "Bit 27 - Checksum Offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IPC_W<27> {
        IPC_W::new(self)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    #[must_use]
    pub fn sarc(&mut self) -> SARC_W<28> {
        SARC_W::new(self)
    }
    #[doc = "Bit 31 - ARP Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<31> {
        ARPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operating mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maccr](index.html) module"]
pub struct MACCR_SPEC;
impl crate::RegisterSpec for MACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maccr::R](R) reader structure"]
impl crate::Readable for MACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maccr::W](W) writer structure"]
impl crate::Writable for MACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACCR to value 0"]
impl crate::Resettable for MACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
