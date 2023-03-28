#[doc = "Register `OTG_HCINT5` reader"]
pub struct R(crate::R<OTG_HCINT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCINT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCINT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCINT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HCINT5` writer"]
pub struct W(crate::W<OTG_HCINT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCINT5_SPEC>;
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
impl From<crate::W<OTG_HCINT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCINT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `CHH` reader - CHH"]
pub type CHH_R = crate::BitReader<bool>;
#[doc = "Field `CHH` writer - CHH"]
pub type CHH_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `AHBERR` reader - AHBERR"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR` writer - AHBERR"]
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `STALL` reader - STALL"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `NAK` reader - NAK"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `NAK` writer - NAK"]
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `ACK` reader - ACK"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - ACK"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `NYET` reader - NYET"]
pub type NYET_R = crate::BitReader<bool>;
#[doc = "Field `NYET` writer - NYET"]
pub type NYET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `TXERR` reader - TXERR"]
pub type TXERR_R = crate::BitReader<bool>;
#[doc = "Field `TXERR` writer - TXERR"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `BBERR` reader - BBERR"]
pub type BBERR_R = crate::BitReader<bool>;
#[doc = "Field `BBERR` writer - BBERR"]
pub type BBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `FRMOR` reader - FRMOR"]
pub type FRMOR_R = crate::BitReader<bool>;
#[doc = "Field `FRMOR` writer - FRMOR"]
pub type FRMOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `DTERR` reader - DTERR"]
pub type DTERR_R = crate::BitReader<bool>;
#[doc = "Field `DTERR` writer - DTERR"]
pub type DTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `BNA` reader - BNA"]
pub type BNA_R = crate::BitReader<bool>;
#[doc = "Field `BNA` writer - BNA"]
pub type BNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `XCSXACTERR` reader - XCSXACTERR"]
pub type XCSXACTERR_R = crate::BitReader<bool>;
#[doc = "Field `XCSXACTERR` writer - XCSXACTERR"]
pub type XCSXACTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
#[doc = "Field `DESCLSTROLL` reader - DESCLSTROLL"]
pub type DESCLSTROLL_R = crate::BitReader<bool>;
#[doc = "Field `DESCLSTROLL` writer - DESCLSTROLL"]
pub type DESCLSTROLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINT5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CHH"]
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBERR"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXERR"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BBERR"]
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FRMOR"]
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BNA"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XCSXACTERR"]
    #[inline(always)]
    pub fn xcsxacterr(&self) -> XCSXACTERR_R {
        XCSXACTERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DESCLSTROLL"]
    #[inline(always)]
    pub fn desclstroll(&self) -> DESCLSTROLL_R {
        DESCLSTROLL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - CHH"]
    #[inline(always)]
    #[must_use]
    pub fn chh(&mut self) -> CHH_W<1> {
        CHH_W::new(self)
    }
    #[doc = "Bit 2 - AHBERR"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - STALL"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<3> {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - NAK"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<4> {
        NAK_W::new(self)
    }
    #[doc = "Bit 5 - ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<5> {
        ACK_W::new(self)
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<6> {
        NYET_W::new(self)
    }
    #[doc = "Bit 7 - TXERR"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<7> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 8 - BBERR"]
    #[inline(always)]
    #[must_use]
    pub fn bberr(&mut self) -> BBERR_W<8> {
        BBERR_W::new(self)
    }
    #[doc = "Bit 9 - FRMOR"]
    #[inline(always)]
    #[must_use]
    pub fn frmor(&mut self) -> FRMOR_W<9> {
        FRMOR_W::new(self)
    }
    #[doc = "Bit 10 - DTERR"]
    #[inline(always)]
    #[must_use]
    pub fn dterr(&mut self) -> DTERR_W<10> {
        DTERR_W::new(self)
    }
    #[doc = "Bit 11 - BNA"]
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BNA_W<11> {
        BNA_W::new(self)
    }
    #[doc = "Bit 12 - XCSXACTERR"]
    #[inline(always)]
    #[must_use]
    pub fn xcsxacterr(&mut self) -> XCSXACTERR_W<12> {
        XCSXACTERR_W::new(self)
    }
    #[doc = "Bit 13 - DESCLSTROLL"]
    #[inline(always)]
    #[must_use]
    pub fn desclstroll(&mut self) -> DESCLSTROLL_W<13> {
        DESCLSTROLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint5](index.html) module"]
pub struct OTG_HCINT5_SPEC;
impl crate::RegisterSpec for OTG_HCINT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcint5::R](R) reader structure"]
impl crate::Readable for OTG_HCINT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hcint5::W](W) writer structure"]
impl crate::Writable for OTG_HCINT5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HCINT5 to value 0"]
impl crate::Resettable for OTG_HCINT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
